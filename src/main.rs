use kuso_kuso_bot::markov::Markov;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() -> () {
    serve_cli();
}

fn serve_cli() -> () {
    let filepath = "./data.txt";
    println!("Open {},,,.", filepath);
    let mut f = File::open(filepath).expect("File not found!");
    println!("File has opened successfully!");

    println!("Load content,,,.");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Something went wrong reading file!");
    println!("Content has loaded successfully!");

    println!("Setup markov generator,,,.");
    let generator = Markov::new(&content);
    println!("Finished setup!");
    println!("raw_text: {:?}", generator.raw_text);
    println!("splited: {:?}", generator.splited);
    println!("e2e: {:?}", generator.e2e);

    println!("Start generating.");
    let duration = Duration::from_millis(500);
    loop {
        println!("{}", generator.generate());
        sleep(duration);
    }
}
