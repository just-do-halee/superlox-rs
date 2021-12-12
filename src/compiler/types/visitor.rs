// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

derive_debug_partials! {

    #[derive(Clone, Copy)]
    pub struct ExprVisitor;

}

impl<'s> Visitor<Expr<'s>> for ExprVisitor {
    fn visit<E: AsMut<Expr<'s>>>(&self, mut expr: E) {
        match expr.as_mut() {
            Expr::Binary(_l_e, _t, _r_e) => {
                // self.visit(l_e);
            }
            Expr::Grouping(_e) => {}
            Expr::Literal(_l) => {}
            Expr::Unary(_t, _e) => {}
            Expr::None => {}
        }
    }
}
