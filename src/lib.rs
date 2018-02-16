extern crate rand;

use std::io;
use std::io::Write;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;


pub fn init(){
let mut a = File::open("first_parts.txt").expect("file not found");
let mut b = File::open("second_parts.txt").expect("file not found");
let mut first_parts= String::new();
let mut second_parts= String::new();

a.read_to_string(&mut first_parts)
        .expect("something went wrong reading the file");
b.read_to_string(&mut second_parts)
        .expect("something went wrong reading the file");

choose_first(&mut first_parts);
choose_second(&mut second_parts);
}

fn choose_first(first_parts: &mut String){
    let first: Vec<_> = first_parts.lines().collect();

    let numb=rand::thread_rng().gen_range(0,first.len() );
    print!("{}",&first[numb]);
    io::stdout().flush().unwrap();


}

fn choose_second(second_parts: &mut String){
    let second: Vec<_>=second_parts.lines().collect();

    let numb=rand::thread_rng().gen_range(0, second.len());
    println!(" {}",&second[numb]);
}


