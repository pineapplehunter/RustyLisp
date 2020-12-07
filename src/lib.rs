use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Functions {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LispExpr {
    Function(Functions),
    Number(i32),
    List(Vec<LispExpr>),
}

impl FromStr for LispExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        Ok(LispExpr::parser(&chars)?.0)
    }
}

impl LispExpr {
    pub fn parser(chars: &[char]) -> Result<(LispExpr, usize), <Self as FromStr>::Err> {
        let mut idx = 0;
        let mut first = true;
        let mut encounter_start = false;
        let mut read_start = false;
        let mut num_flg = true;
        let mut buf = String::new();

        let mut list = Vec::new();

        loop {
            if chars.len() <= idx {
                return Err("index out of range error!".to_string());
            }
            // println!("{}", chars[idx]);
            if !encounter_start {
                if chars[idx] == '(' {
                    encounter_start = true;
                }
            } else {
                match chars[idx] {
                    '(' => {
                        let (expr, offset) = LispExpr::parser(&chars[idx..])?;
                        list.push(expr);
                        idx += offset;
                    }
                    ')' => {
                        if read_start {
                            if first {
                                list.push(LispExpr::Function(buf.parse()?));
                            } else if num_flg {
                                list.push(LispExpr::Number(buf.parse().unwrap()));
                            } else {
                                unimplemented!();
                            }
                        }

                        break;
                    }
                    ' ' => {
                        if read_start {
                            if first {
                                list.push(LispExpr::Function(buf.parse()?));
                            } else if num_flg {
                                list.push(LispExpr::Number(buf.parse().unwrap()));
                            } else {
                                unimplemented!();
                            }
                            first = false;
                        }
                        read_start = false;
                        num_flg = true;
                        buf.clear();
                    }
                    n if n.is_digit(10) => {
                        read_start = true;
                        buf.push(n);
                    }
                    c => {
                        if !read_start {
                            num_flg = false;
                        }
                        read_start = true;
                        buf.push(c)
                    }
                };
            }
            idx += 1;
        }

        Ok((LispExpr::List(list), idx))
    }
}

impl FromStr for Functions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.to_ascii_lowercase();
        let val = val.as_str();

        // println!("'{}'", val);

        match val {
            "+" => Ok(Functions::Add),
            "-" => Ok(Functions::Sub),
            "*" => Ok(Functions::Mul),
            "/" => Ok(Functions::Div),
            _ => Err("could not parse function.".to_string()),
        }
    }
}

impl LispExpr {
    pub fn eval(&self) -> i32 {
        use LispExpr::*;
        match self {
            List(list) => match &list[0] {
                Function(f) => {
                    use Functions::*;
                    match f {
                        Add => list.iter().skip(1).map(|e| e.eval()).sum(),
                        Sub => list[1].eval() - list.iter().skip(2).map(|e| e.eval()).sum::<i32>(),
                        Mul => list.iter().skip(1).fold(1, |a, v| a * v.eval()),
                        Div => list
                            .iter()
                            .skip(2)
                            .fold(list[1].eval(), |a, v| a / v.eval()),
                    }
                }
                _ => unreachable!(),
            },
            Number(n) => *n,
            _ => unreachable!(),
        }
    }
}
