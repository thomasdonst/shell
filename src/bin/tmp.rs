#![allow(warnings)]

// #![feature(generator_trait)]
// #![feature(generators)]

use std::io;
use std::io::Read;
// use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::process::Stdio;

fn main() {
    // a();
}

fn a() {
    // let mut generator = || -> Result<String, String> {
    //     loop {
    //         let mut buffer = String::new();
    //         let _ = io::stdin().read_line(&mut buffer);
    //         yield Ok(buffer);
    //         yield Err("error".to_string());
    //     }
    // };
    //
    // loop {
    //     match Pin::new(&mut generator).resume(()) {
    //         GeneratorState::Yielded(x) =>
    //             match x {
    //                 Ok(s) => print!("{}", s),
    //                 Err(e) => eprint!("{}", e)
    //             },
    //         GeneratorState::Complete(_) => ()
    //     };
    // }
}