pub mod markov {
    use rand::{Rng, distr::weighted::WeightedIndex};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use unicode_segmentation::UnicodeSegmentation;

    #[derive(Serialize, Deserialize, Default, Debug, Clone)]
    pub struct Markov {
        pub v2v2cnt: HashMap<String, HashMap<String, u32>>,
    }

    impl Markov {
        pub fn new(raw_text: &str) -> Markov {
            let splited = Self::split(raw_text);
            let mut generator = Markov {
                v2v2cnt: HashMap::new(),
            };
            generator.update_v2v2cnt(&splited);
            generator
        }

        fn split(raw_text: &str) -> Vec<&str> {
            UnicodeSegmentation::graphemes(raw_text, true).collect::<Vec<&str>>()
        }

        fn update_v2v2cnt(&mut self, splited: &Vec<&str>) -> () {
            if splited.len() < 2 {
                return;
            }
            for i in 0..splited.len() - 1 {
                let v2cnt = self
                    .v2v2cnt
                    .entry(splited[i].to_string())
                    .or_insert(HashMap::new());
                let cnt = v2cnt.entry(splited[i + 1].to_string()).or_insert(0);
                *cnt += 1;
            }
        }

        pub fn generate(&self) -> String {
            let mut generated: Vec<&str> = vec![self.choice("\n")];

            loop {
                let next = self.choice(generated.last().unwrap());

                if next == "\n" {
                    break;
                }

                generated.push(next);
            }

            let mut generated_str = String::new();
            generated
                .iter()
                .for_each(|element| generated_str.push_str(element));

            generated_str
        }

        fn choice(&self, before: &str) -> &str {
            let mut rng = rand::rng();

            let v2cnt = &self.v2v2cnt[before];

            let mut cnts = Vec::new();
            v2cnt.values().for_each(|cnt| cnts.push(cnt));

            let weighted_index = WeightedIndex::new(cnts).unwrap();

            let i = rng.sample(weighted_index);

            let mut next = "\n";
            for (j, (v, _)) in v2cnt.iter().enumerate() {
                if j == i {
                    next = v;
                    break;
                }
            }

            next
        }

        pub fn add(&mut self, raw_text: &str) -> () {
            let v = Self::split(raw_text);
            self.update_v2v2cnt(&v);
        }
    }
}
