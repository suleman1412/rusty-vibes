use std::{io::{self, Write}, thread};

fn main(){
    io::stdout().flush().unwrap();
    println!("How many threads to spawn: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let threads = input.trim().parse::<i32>().unwrap();
    thread::spawn(move ||{
        for i in 1..=threads{
            println!("hi from spawned thread {i}");
        };
    });
    println!("{threads}");
    // Declaring an inf loop to see the spawned threads
    loop {
        
    }
}