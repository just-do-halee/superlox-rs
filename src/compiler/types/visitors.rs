// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

derive_debug_partials! {

    #[derive(Clone, Copy)]
    pub struct ExprVisitor;

}

impl<'s> Visitor<Expr<'s>> for ExprVisitor {
    fn visit<E: AsMut<Expr<'s>>>(&self, mut expr: E) {
        match expr.as_mut() {
            Expr::Binary(l_e, t, r_e) => {
                // self.visit(l_e);
            }
            Expr::Grouping(e) => {}
            Expr::Literal(l) => {}
            Expr::Unary(t, e) => {}
        }
    }
}
