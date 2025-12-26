use rand::random_range;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

struct MarkovGenerator<'a> {
    raw_text: &'a str,
    splited: Vec<&'a str>,
    e2e: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> MarkovGenerator<'a> {
    pub fn new(raw_text: &'a str) -> MarkovGenerator<'a> {
        let splited = Self::split(raw_text);
        let e2e = Self::setup_e2e(&splited);
        MarkovGenerator {
            raw_text,
            splited,
            e2e,
        }
    }

    fn split(raw_text: &'a str) -> Vec<&'a str> {
        UnicodeSegmentation::graphemes(raw_text, true).collect::<Vec<&str>>()
    }

    fn setup_e2e(splited: &Vec<&'a str>) -> HashMap<&'a str, Vec<&'a str>> {
        let mut e2e = HashMap::new();
        for i in 0..splited.len() - 1 {
            let v = e2e.entry(splited[i]).or_insert(Vec::new());
            v.push(splited[i + 1]);
        }
        e2e
    }

    pub fn generate(&self) -> String {
        let mut generated = vec![self.e2e["\n"][random_range(0..self.e2e["\n"].len())]];

        loop {
            let predicted = self.e2e[generated.last().expect("wtf")]
                [random_range(0..self.e2e[generated.last().expect("wtf")].len())];

            if predicted == "\n" {
                break;
            }

            generated.push(predicted);
        }

        let mut generated_str = String::new();
        generated
            .iter()
            .for_each(|element| generated_str.push_str(element));

        generated_str
    }
}

fn main() {
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
    let generator = MarkovGenerator::new(&content);
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
