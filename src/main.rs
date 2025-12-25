use rand::random_range;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

struct MarkovGenerator<'a> {
    raw_text: Option<&'a str>,
    splited: Option<Vec<&'a str>>,
    e2e: Option<HashMap<&'a str, Vec<&'a str>>>,
}

impl<'a> MarkovGenerator<'a> {
    fn setup_splited(&mut self) -> () {
        if let Some(raw_text) = self.raw_text {
            self.splited =
                Some(UnicodeSegmentation::graphemes(raw_text, true).collect::<Vec<&str>>());
        } else {
            panic!("No value in raw_text!")
        }
    }

    fn setup_e2e(&mut self) -> () {
        self.e2e = Some(HashMap::new());
        if let Some(e2e) = &mut self.e2e {
            if let Some(splited) = &self.splited {
                for i in 0..splited.len() - 1 {
                    let v = e2e.entry(splited[i]).or_insert(Vec::new());
                    v.push(splited[i + 1]);
                }
            }
        }
    }

    fn setup(&mut self) -> () {
        self.setup_splited();
        self.setup_e2e();
    }

    fn generate(&self) -> String {
        let e2e = self.e2e.as_ref().expect("e2e has not setup yet!");
        let mut generated = vec![e2e["\n"][random_range(0..e2e["\n"].len())]];

        loop {
            let predicted = e2e[generated.last().expect("wtf")]
                [random_range(0..e2e[generated.last().expect("wtf")].len())];

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
    let mut generator = MarkovGenerator {
        raw_text: Some(&content),
        splited: None,
        e2e: None,
    };
    generator.setup();
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
