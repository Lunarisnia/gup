<h1> Gup - Give Up Version Control System </h1>
<div style="text-align: center;">A Version Control System for the brave</div>

# What is this?
This is a result of me letting one of my intrusive thought win after one of my colleague [@ervanhohoho](https://github.com/ervanhohoho) suggested it most likely as a joke.
<br>
```What if you make your own Git?``` <br>
And here we are 2months later. I don't even remember the context of the conversation anymore but I am glad I did this because this taught
me a lot about designing software, rust and DSA. And yes this is also an excuse to use Rust.

# Why Use Gup?
1. It is slower than Git.
2. No single file staging support.
3. No Merging.
4. Questionable branch switching logic that may or may not broke someday and accidentally delete most of your uncommitted work.

# Usage
`gup init` initialize a Gup repository on current workdir
<br>
`gup add .` stage all files in current dir
<br>
`gup commit "message goes here"` commit all staged files
<br>
`gup branch new_branch_name` create a new branch based on the current active branch
<br>
`gup checkout branch_name` change active branch

# Getting Started
```
gup init
gup add .
gup commit "this work"
```

# Contributing
Don't