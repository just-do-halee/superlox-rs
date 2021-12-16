// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! 'tests'

use super::*;

#[test]
fn displaying_expr() {
    let source = Source::new("-123 * (\"45.67\")");
    let expr = parser::run(lexer::run(&source).unwrap()).unwrap();

    assert_eq!(expr.to_string(), "(* (- 123) (group \"45.67\"))");
}

mod eval_expr {
    use super::*;

    #[inline]
    fn compile(s: &str) -> Object {
        let source = Source::new(s);
        ExprVisitor
            .visit(parser::run(lexer::run(&source).unwrap()).unwrap())
            .unwrap()
    }

    #[inline]
    fn compile_assert_eq(input: &str, output: &str) {
        let out = compile(input).to_string();
        assert_eq!(out, output)
    }

    #[test]
    fn mix() {
        compile_assert_eq(
            r#"5 | 8, !(123.523 / 123.523), 40 / 5 - 7, "hahaha" + " fuck!";"#,
            r#"13, false, 1, "hahaha fuck!""#,
        );
        compile_assert_eq(
            r#"0.42+2.58 ^ 3, 1*2 & 5/2, 1.0/2.0 == 4 & 2 - (2*3) | 5-2;"#,
            r#"0, 2, false"#,
        );
        compile_assert_eq(
            r#"12 == (81 - 4 * 20) * ((32 + 8) / 5) | 2.34 + -8 + 9.66, "test " + "man" + " yes.", ----8"#,
            r#"true, "test man yes.", 8"#,
        );
    }

    #[test]
    fn equality() {
        compile_assert_eq(
            r#"1==2, 2==2, "haha"=="haha", true==false, 1.3434344123==1.3434344123"#,
            "false, true, true, false, true",
        );
        compile_assert_eq(
            r#"1!=2, 2!=2, "haha"!="haha", true!=false, 1.3434344123!=1.3434344123"#,
            "true, false, false, true, false",
        );
    }
    #[test]
    fn comparison() {
        compile_assert_eq("1<2, 1<=2", "true, true");
        compile_assert_eq("2<=2, 2>=2, 1>=2", "true, true, false");
        compile_assert_eq("1>2, -1>2", "false, false");
    }
    #[test]
    fn bitwise() {
        compile_assert_eq("7 & 10", "2");
        compile_assert_eq("5 | 8", "13");
        compile_assert_eq("7 ^ 4", "3");
    }
    #[test]
    fn term() {
        compile_assert_eq(r#"7 + 10, 5 - 8, "sexy" + " guy""#, r#"17, -3, "sexy guy""#);
    }
    #[test]
    fn factor() {
        compile_assert_eq("7 * 4, 8 / 4", "28, 2");
    }
    #[test]
    fn unary() {
        compile_assert_eq("!true", "false");
        compile_assert_eq("!false", "true");
        compile_assert_eq("!!!!!false", "true");
        compile_assert_eq("-5", "-5");
        compile_assert_eq("--12", "12");
    }
}
