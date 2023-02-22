use std::{io::{self, Read}, thread::current};

/*

A standard Brainfuck compiler/interpreter states 30k 1 byte memory blocks.

> = increases memory pointer, or moves the pointer to the right 1 block.
< = decreases memory pointer, or moves the pointer to the left 1 block.
+ = increases value stored at the block pointed to by the memory pointer
- = decreases value stored at the block pointed to by the memory pointer
[ = like c while(cur_block_value != 0) loop.
] = if block currently pointed to's value is not zero, jump back to [
, = like c getchar(). input 1 character.
. = like c putchar(). print 1 character to the console

*/

fn execute_loop(memory: &[u8], current_pointer: usize, loopexpr: &str) {
    println!("loop here: {}", loopexpr);
    while memory[current_pointer] != 0 {
    break;
    }
}

fn main() {
    let mut memory = [0 as u8; 30000];

    let mut current_pointer = 0;
    
    println!("Hello, world!");
    println!("input:");
    let mut input = String::from("");
    std::io::stdin().read_line(&mut input).expect("Couldn't read string");
    // Interpreting
    let mut skip: usize = 0;

    for (i, c) in input.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        //println!("Current char {}", inputchar);
        match c {
            '>' => {
                current_pointer += 1;
            },
            '<' => {
                current_pointer -= 1;
            },
            '+' => {
                memory[current_pointer] += 1;
            },
            '-' => {
                memory[current_pointer] -= 1;
            },
            '[' => {
                let startpos = i;
                let mut stack = 1;
                let mut exprindex = 1;
                println!("Starting loop lookup");
                while stack != 0 {
                    if input.chars().nth(startpos + exprindex) == Some('[') {
                        stack += 1;
                        println!("stack+");
                    }

                    if input.chars().nth(startpos + exprindex) == Some(']') {
                        stack -= 1;
                        println!("stack-");
                    }
                    exprindex += 1;
                }
                let exprlen = exprindex;
                println!("exprindex {}", exprindex);
                let loopexpr = &input[startpos..startpos+exprlen];
                //let Some(loopexpr) = &input[startpos..input.len()].find(']');
                execute_loop(&memory, current_pointer, loopexpr);
                skip = exprlen;
            },
            ']' => {},
            ',' => {},
            '.' => {print!("{}", memory[current_pointer] as char)},
            _ => {},
        }
    }
    println!("{:?}", &memory[0..32])
}
