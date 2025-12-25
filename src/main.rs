use std::collections::HashMap;
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
                    let v = e2e.entry(splited[i]).or_insert(vec!["<end>"]);
                    v.push(splited[i + 1]);
                }

                // last element may not be pushed
                if let Some(last_element) = splited.last() {
                    e2e.entry(*last_element).or_insert(vec!["<end>"]);
                }
            }
        }
    }

    fn setup(&mut self) -> () {
        self.setup_splited();
        self.setup_e2e();
    }

    fn generate(&self) -> &'a str {}
}

fn main() {
    println!("Setup markov generator,,,.");
    let mut generator = MarkovGenerator {
        raw_text: Some("POCKYCHOCOLATE"),
        splited: None,
        e2e: None,
    };
    &mut generator.setup();
    println!("Finished setup!");
    println!("raw_text: {:?}", generator.raw_text);
    println!("splited: {:?}", generator.splited);
    println!("e2e: {:?}", generator.e2e);
}
