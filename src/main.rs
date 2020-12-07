use rusty_lisp::LispExpr;

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn main() {
    loop {
        let s = input();
        if s.trim() == "quit" {
            break;
        }

        match s.parse::<LispExpr>() {
            Ok(expr) => {
                println!("{:?}", expr);
                println!("output = {}", expr.eval());
            }
            Err(e) => println!("error! = {}", e),
        }
    }
    println!("stop");
}