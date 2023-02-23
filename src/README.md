# SSahinDB
A very simple Brainfuck interpreter, written in Rust.

## About
In my journey of learning Rust I decided to write a simple Brainfuck interpreter, as I love the concept of Brainfuck (the concept of a Turing machine itself) and practicing Rust.

At the time of writing (23/02/2023 in dd/mm/yy format) I am still new to Rust, and I do not claim that the code is optimal. There are probably lots of improvements to be made.  

This particular interpreter might have some problems.

## Installation & Running
After running `cargo build`, preferably with the `--release` flag, the `brainfuck-interpreter` binary can be run directly. There is one mandatory and one optional stdin, the mandatory is the expression input (your "code"), the optional is the data input. Both can be typed in manually after running the program, or can be `echo`ed with `echo ">++++[>++++<-]>" | ./brainfuck-interpreter` for expression input, and `echo "exprhere\ndatainputhere" | ./brainfuck-interpreter` for expression & data input.

## Known Bugs & Issues
`,` inside loops have problems I have yet to debug. This project might have some problems, I haven't tested it extensively.

## What to Do
Fix the above mentioned problem?