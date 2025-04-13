## Simple Rust CLI chat for Gemini AI

```shell
 ✗ export GEMINI_API_KEY=XXX  # free from https://aistudio.google.com/app/apikey
 ✗  # building
 ✗ cargo r --release
~...~
 ✗  # testing
 ✗ ./target/release/gemini-chat "In Rust what is the difference between Fn, FnMut, and FnOnce?"  # simple prompt as a CLI param
~...~
 ✗ cat my_question.md | ./target/release/gemini-chat  # prompt as a stdin pipe, e. g. from Markdown file with your code and detailed question
~...~
 ✗
```
