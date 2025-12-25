use rand::random_range;
use std::collections::HashMap;
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
                //
                // // last element may not be pushed
                // if let Some(last_element) = splited.last() {
                //     e2e.entry(*last_element).or_insert(vec!["<end>"]);
                // }
            }
        }
    }

    fn setup(&mut self) -> () {
        self.setup_splited();
        self.setup_e2e();
    }

    fn generate(&self) -> String {
        if let Some(e2e) = &self.e2e {
            if let Some(element) = e2e.keys().nth(random_range(0..e2e.keys().len())) {
                let mut generated = vec![*element];
                loop {
                    let hoge = e2e[generated.last().expect("wtf")]
                        [random_range(0..e2e[generated.last().expect("wtf")].len())];

                    if hoge == "\n" {
                        break;
                    }

                    generated.push(hoge);
                }

                let mut generated_str = String::new();
                generated
                    .iter()
                    .for_each(|element| generated_str.push_str(element));

                generated_str
            } else {
                panic!("wtf");
            }
        } else {
            panic!("e2e has not setup yet!");
        }
    }
}

fn main() {
    println!("Setup markov generator,,,.");
    let mut generator = MarkovGenerator {
        raw_text: Some(
            "重役募集ラストスパート
寒風の吹きすさぶ折、皆様いかがお過ごしでしょうか。
さて、次期重役の椅子も現在あと少しとなっております。とはいえ、あと少しといっても全て埋める必要があります。
皆様のご連絡、切にお待ちしております。
ピン留めにある、既に申し込まれた御学友と共に放課後に議論する。中々ステキな時間だと感じます。
",
        ),
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
