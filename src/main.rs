use std::str::FromStr;

#[derive(Debug)]
enum Functions{
    Add,
}

enum LispExpr {
    Function(Functions),
    Number(i32),
    List(Vec<LispExpr>),
}

impl FromStr for LispExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        LispExpr::parser(&chars).0
    }
}

impl LispExpr {
    fn parser(chars: &[char]) -> Result<(LispExpr, idx), <Self as FromStr>::Err> {
        let mut idx = 0;
        let mut encounter_start = false;
        loop {
            if encounter_start {

            }
            idx += 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
