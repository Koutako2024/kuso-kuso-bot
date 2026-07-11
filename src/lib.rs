pub mod markov {
    use rand::random_range;
    use std::collections::HashMap;
    use unicode_segmentation::UnicodeSegmentation;

    pub struct Markov<'a> {
        pub raw_text: &'a str,
        pub splited: Vec<&'a str>,
        pub v2v: HashMap<&'a str, Vec<&'a str>>,
    }

    impl<'a> Markov<'a> {
        pub fn new(raw_text: &'a str) -> Markov<'a> {
            let splited = Self::split(raw_text);
            let v2v = Self::setup_v2v(&splited);
            Markov {
                raw_text,
                splited,
                v2v,
            }
        }

        fn split(raw_text: &'a str) -> Vec<&'a str> {
            UnicodeSegmentation::graphemes(raw_text, true).collect::<Vec<&str>>()
        }

        fn setup_v2v(splited: &Vec<&'a str>) -> HashMap<&'a str, Vec<&'a str>> {
            let mut v2v = HashMap::new();
            for i in 0..splited.len() - 1 {
                let v = v2v.entry(splited[i]).or_insert(Vec::new());
                v.push(splited[i + 1]);
            }
            v2v
        }

        pub fn generate(&self) -> String {
            let r = self.v2v["\n"].len();
            let hoge = random_range(0..r);
            let mut generated = vec![self.v2v["\n"][hoge]];

            loop {
                let predicted = self.v2v[generated.last().expect("wtf")]
                    [random_range(0..self.v2v[generated.last().expect("wtf")].len())];

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
}
