## Simple Rust CLI chat for Gemini AI

```shell
 ✗ export GEMINI_API_KEY=XXX  # free from https://aistudio.google.com/app/apikey
 ✗ cargo r --release  # building
~...~
 ✗ ./target/release/gemini-chat --help
Simple Rust CLI chat for Gemini AI

Usage: gemini-chat <PROMPT>

Arguments:
  <PROMPT>  The prompt to send to Gemini AI

Options:
  -h, --help     Print help
  -V, --version  Print version
 ✗ ./target/release/gemini-chat "In Rust what is the difference between Fn, FnMut, and FnOnce?"  # testing
~...~
 ✗
```
