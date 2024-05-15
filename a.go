package create_credit_notes

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/cosmart-tech/cronmart"
	"github.com/go-playground/validator/v10"
	"github.com/shopspring/decimal"
	"github.com/spf13/viper"
	creditnotesrequest "go-lambda-helloworld/internal/create_credit_notes/structs"
	"go-lambda-helloworld/internal/databases"
	"go-lambda-helloworld/internal/entities"
	"go-lambda-helloworld/internal/helpers"
	"go-lambda-helloworld/internal/repositories"
	synccreditnotestructs "go-lambda-helloworld/internal/sync_credit_notes/structs"
	"gorm.io/gorm"
	"io"
	"log"
	"net/http"
	"time"
)

type CreateCreditNoteService struct {
	Cronmart                        cronmart.Cronmart
	SaleCreditNoteRepository        repositories.SaleCreditNoteRepository
	SaleCreditNoteItemRepository    repositories.SaleCreditNoteItemRepository
	SaleCreditNoteRestockRepository repositories.SaleCreditNoteRestockRepository
	OrderItemRepository             repositories.OrderItemRepository
	SaleRepository                  repositories.SaleRepository
}

func getSales(salesId *string) (*entities.SalesResponseModel, error) {
	url := fmt.Sprintf("https://inventory.dearsystems.com/ExternalApi/v2/sale?ID={%s}", *salesId)
	req, err := http.NewRequest(http.MethodGet, url, nil)
	if err != nil {
		return nil, err
	}

	req.Header = http.Header{
		"Content-Type":            {"application/json"},
		"api-auth-accountid":      {viper.GetString("dear.accountId")},
		"api-auth-applicationkey": {viper.GetString("dear.applicationKey")},
	}

	client := http.Client{}

	response, err := client.Do(req)
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			log.Default().Println(err)
		}
	}(response.Body)
	if err != nil {
		return nil, err
	}
	bodyBytes, err := io.ReadAll(response.Body)
	if err != nil {
		return nil, err
	}

	if response.StatusCode == 200 {
		var body entities.SalesResponseModel
		if err := json.Unmarshal(bodyBytes, &body); err != nil {
			log.Default().Println(err, fmt.Sprintf("[DEAR_INTEGRATION_SERVICE] [GetSales] Error unmarshal response for sales ID %s", *salesId))
			return nil, err
		}
		return &body, nil
	} else {
		bodyString := string(bodyBytes)
		return nil, errors.New(bodyString)
	}
}

func (c CreateCreditNoteService) Handler(ctx context.Context, rawRequestBody interface{}) (interface{}, error) {
	var jsonBytes []byte
	var err error
	if body, ok := rawRequestBody.(map[string]interface{}); ok {
		jsonBytes = []byte(body["body"].(string))
	}

	createSaleCreditNoteBody := creditnotesrequest.CreditNoteRequest{}
	err = json.Unmarshal(jsonBytes, &createSaleCreditNoteBody)
	if err != nil {
		return helpers.LambdaResult(http.StatusInternalServerError, err.Error()), nil
	}

	validate := validator.New(validator.WithRequiredStructEnabled())
	err = validate.Struct(createSaleCreditNoteBody)
	if err != nil {
		log.Default().Println("one or more fields is not valid: ", err)
		return helpers.LambdaResult(http.StatusInternalServerError, "one or more required fields is not valid"), nil
	}
	sale, err := c.SaleRepository.FindOneByOrderId(ctx, createSaleCreditNoteBody.OrderId)
	if err != nil {
		log.Default().Println(err)
		return helpers.LambdaResult(http.StatusInternalServerError, "invalid order_id"), nil
	}
	dearSale, err := getSales(sale.ExternalSalesId)
	if err != nil {
		log.Default().Println(err)
		return helpers.LambdaResult(http.StatusInternalServerError, "DEAR get sales error"), err
	}

	newSaleCreditNote := entities.SaleCreditNoteEntity{
		OrderId:                 *createSaleCreditNoteBody.OrderId,
		Timestamp:               time.Now().In(helpers.Jakarta()),
		Reason:                  *createSaleCreditNoteBody.Reason,
		CreditNoteInvoiceNumber: dearSale.Invoices[0].InvoiceNumber, // Fetch this from opor.sales.external_invoice_id
		Memo:                    *createSaleCreditNoteBody.Memo,
		Status:                  *createSaleCreditNoteBody.Status,
		ImsId:                   *sale.ExternalSalesId, // can't work if the order has more than one invoices
	}
	err = databases.UseTransaction(c.SaleCreditNoteRepository, func(tx *gorm.DB) error {
		err = c.SaleCreditNoteRepository.CreateTx(ctx, tx, &newSaleCreditNote)
		if err != nil {
			return errors.New("failed to create sale credit note")
		}

		newSaleCreditNoteItems := make([]entities.SaleCreditNoteItemEntity, 0)
		newSaleCreditNoteRestocks := make([]entities.SaleCreditNoteRestockEntity, 0)
		for _, item := range createSaleCreditNoteBody.Items {
			itemMetadata, err := c.OrderItemRepository.CheckValidRestock(ctx, createSaleCreditNoteBody.OrderId, item.Sku, item.Quantity)
			if err != nil {
				return errors.New("failed to check restock validity")
			}

			quantityDecimal := decimal.NewFromInt(int64(*item.Quantity))
			discountDecimal := decimal.NewFromInt(*item.Discount)

			newSaleCreditNoteItem := entities.SaleCreditNoteItemEntity{
				CreditNoteID: newSaleCreditNote.ID,
				Sku:          *item.Sku,
				ImsSku:       *item.Sku,
				Quantity:     *item.Quantity,
				Price:        itemMetadata.PricePerItem,
				Discount:     discountDecimal,
				TaxRule:      "Tax on Sales",
				Total:        itemMetadata.PricePerItem.Mul(quantityDecimal),
				Account:      "6101",
				Comment:      *item.Comment,
				SkuName:      itemMetadata.Name,
			}
			newSaleCreditNoteItems = append(newSaleCreditNoteItems, newSaleCreditNoteItem)

			newSaleCreditNoteRestock := entities.SaleCreditNoteRestockEntity{
				CreditNoteID: newSaleCreditNote.ID,
				Sku:          *item.Sku,
				ImsSku:       *item.Sku,
				SkuName:      itemMetadata.Name,
				Quantity:     *item.Quantity,
				Location:     "Cosmart B2C Warehouse",
			}
			newSaleCreditNoteRestocks = append(newSaleCreditNoteRestocks, newSaleCreditNoteRestock)
		}

		err = c.SaleCreditNoteItemRepository.CreateManyTx(ctx, tx, newSaleCreditNoteItems)
		if err != nil {
			return errors.New("failed to create sale credit note item")
		}

		err = c.SaleCreditNoteRestockRepository.CreateManyTx(ctx, tx, newSaleCreditNoteRestocks)
		if err != nil {
			return errors.New("failed to create sale credit note restock")
		}

		syncCreditNoteRequest := synccreditnotestructs.SyncCreditNoteRequest{
			OrderId: createSaleCreditNoteBody.OrderId,
			ID:      &newSaleCreditNote.ID,
		}

		// Prepare the sync credit note payload
		jsonBytes, err = json.Marshal(syncCreditNoteRequest)
		if err != nil {
			return errors.New("failed to marshal sync credit note request")
		}

		syncCreditNoteUrl := viper.GetString("sync_url")
		requestPayload := c.Cronmart.CreateSchedulePayload(ctx, syncCreditNoteUrl, http.MethodPost, string(jsonBytes), false, nil)
		err = c.Cronmart.BookExecution(ctx, requestPayload, cronmart.Option{
			Verbose: true,
		})
		if err != nil {
			tx.Rollback()
			log.Default().Println(err)
			return errors.New("failed to create schedule payload")
		}

		return nil
	})
	if err != nil {
		return helpers.LambdaResult(http.StatusInternalServerError, err.Error()), nil
	}

	return "Success", nil
}
