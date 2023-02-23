use std::{io::{self, Read, stdin}, thread::current};

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

fn evaluate_expression(memory: &mut [i32], current_pointer: &mut usize, expr: &str) {
    let mut skip: usize = 0;

    for (i, c) in expr.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        //println!("Current char {}", c);
        match c {
            '>' => {
                *current_pointer += 1;
            },
            '<' => {
                *current_pointer -= 1;
            },
            '+' => {
                memory[*current_pointer] += 1;
            },
            '-' => {
                memory[*current_pointer] -= 1;
            },
            '[' => {
                let startpos = i;
                let mut stack = 1;
                let mut subindex = 1;
                //println!("Starting loop lookup");
                while stack != 0 {
                    if expr.chars().nth(startpos + subindex) == Some('[') {
                        stack += 1;
                        //println!("stack+");
                    }

                    if expr.chars().nth(startpos + subindex) == Some(']') {
                        stack -= 1;
                        //println!("stack-");
                    }
                    subindex += 1;
                }
                let exprlen = subindex;
                //println!("subindex {}", subindex);
                let loopexpr = &expr[startpos+1..startpos+exprlen-1]; // To not include the start and end brackets
                //println!("loopexpr {}", &loopexpr);

                //println!("{:?}", &memory[0..32]);
                //let Some(loopexpr) = &input[startpos..input.len()].find(']');
                //execute_loop(memory, current_pointer, loopexpr);
                let thisindex = *current_pointer;
                while memory[thisindex] != 0 {
                    evaluate_expression(memory, current_pointer, loopexpr);
                }
                skip = exprlen-1;
            },
            ']' => {},
            ',' => {
                let mut readbuf: [u8; 1] = [0];
                stdin().read_exact(&mut readbuf).expect("Couldn't read from stdin");
                memory[*current_pointer] = readbuf[0] as i32;
            },
            '.' => {print!("{}", (memory[*current_pointer] as u8) as char)}, //(memory[*current_pointer]+30) as char)},
            _ => {},
        }
    }
}

fn main() {
    let mut memory = [0 as i32; 30000];

    let mut current_pointer = 0;
    
    println!("Hello, world!");
    println!("ptr: {}", current_pointer);
    println!("input:");
    let mut input = String::from("");
    std::io::stdin().read_line(&mut input).expect("Couldn't read string");
    //println!("evaluating...\n");
    // Interpreting
    evaluate_expression(&mut memory, &mut current_pointer, &input[0..input.len()-1]);
    //println!("\n\nevaluated\n");
    println!("\n\n");
    println!("{:?}", &memory[0..32]);
    let pointer_offset = 1+((current_pointer)*3);
    println!("{: <1$}{ptr}", "", (pointer_offset), ptr="^")
}
