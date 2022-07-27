use std::env;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::str;
use ansi_term::Colour;
use ansi_term::Style;

fn main() {
    let white_bold_style = Style::new().bold();
    let info_copyright_and_warranty = white_bold_style.paint("
Caesar's Cipher cracker

Command Line Tool that can decode text that is encoded with the caesar's cipher

Copyright (C) 2021-2022, Jared M. Bennett

This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it
under certain conditions.
    ");
    
    println!("{}", info_copyright_and_warranty);

    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Less => panic!("Too few args"),
        Ordering::Greater => panic!("Too many args"),
        Ordering::Equal => println!("Ciphered Text is: \"{}\"", Colour::Red.paint(&args[1])),
    }
    let cipher_text = &args[1];
    while !decipher(cipher_text) {
        print_simple("Cycled through whole alphabeth.\n Retry? ");
        if !read_yes_no() {
            break;
        }
        println!("Retrying...");
    }
        
}

fn decipher(in_s: &str) -> bool {
    let mut in_vec: Vec<u8> = in_s.bytes().collect::<Vec<u8>>();
    let mut byte_vec;
    for l in 0..25 {
        byte_vec = Vec::new();
        for b in &in_vec {
            let current_char = to_uppercase(*b);
            if  (65..=90).contains(&current_char) {
                if current_char == 65 {
                    byte_vec.push(current_char + 25);
                } else {
                    byte_vec.push(current_char - 1);
                }
            } else {
                byte_vec.push(current_char)
            }
        }
        let buf = &byte_vec[..];
        let s = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence {}", e),
        };
        println!("Is this readable?\n{}", s);

        if read_yes_no() {
            println!("Deciphered: {}", Colour::Green.paint(s));
            println!("The alphabeth was shifted {} places", l + 1);
            return true;
        }
        in_vec = byte_vec;
    }
    false
}

fn to_uppercase(b: u8) -> u8 {
    if (97..=122).contains(&b) {
        b - 32u8
    } else {
        b
    }
}

fn read_yes_no() -> bool {
    print_simple("[y/N] ");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line!");
    if buf.trim() == "y" || buf.trim() == "Y" {
        return true;
    }
    false
}

fn print_simple(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}
