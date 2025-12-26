pub mod markov {
    use rand::random_range;
    use std::collections::HashMap;
    use unicode_segmentation::UnicodeSegmentation;

    pub struct Markov<'a> {
        pub raw_text: &'a str,
        pub splited: Vec<&'a str>,
        pub e2e: HashMap<&'a str, Vec<&'a str>>,
    }

    impl<'a> Markov<'a> {
        pub fn new(raw_text: &'a str) -> Markov<'a> {
            let splited = Self::split(raw_text);
            let e2e = Self::setup_e2e(&splited);
            Markov {
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
}
