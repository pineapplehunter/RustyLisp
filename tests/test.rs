use std::vec;

use rusty_lisp::{
    Functions::{Add, Div, Mul, Sub},
    LispExpr::{self, Function, List, Number},
};

#[test]
fn test_funcs() {
    let input = "(+ 1 2 3 4)";
    let expr: LispExpr = input.parse().unwrap();
    assert_eq!(expr.eval(), 10);

    let input = "(* 1 2 3 4)";
    let expr: LispExpr = input.parse().unwrap();
    assert_eq!(expr.eval(), 24);

    let input = "(- 1 2 3 4)";
    let expr: LispExpr = input.parse().unwrap();
    assert_eq!(expr.eval(), -8);

    let input = "(/ 24 6 2)";
    let expr: LispExpr = input.parse().unwrap();
    assert_eq!(expr.eval(), 2);
}

#[test]
fn test_expr() {
    let input = "(+ 5 (- 20 (* (/ 4 2) 3 5) 2) 8)";
    let expr: LispExpr = input.parse().unwrap();
    assert_eq!(
        expr,
        List(vec![
            Function(Add),
            Number(5),
            List(vec![
                Function(Sub),
                Number(20),
                List(vec![
                    Function(Mul),
                    List(vec![Function(Div), Number(4), Number(2)]),
                    Number(3),
                    Number(5)
                ]),
                Number(2)
            ]),
            Number(8)
        ])
    );
}
