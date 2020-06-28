
use crate::parsing::ast::*;
use super::code::Code;

fn extract(c : Code) -> String {
    match c {
        Code::S(s) => s,
        _ => panic!("extract"),
    }
}

pub fn gen_mod( module : Mod ) -> Code { 
    Code::Nop
}

fn gen_fun_def( fun_def : FunDef ) -> Code {
    Code::Nop
}

fn gen_statement( statement : Statement ) -> Code {
    match statement {
        Statement::Return(expr) => {
            match expr {
                Some(e) => Code::S(format!("return {}", extract( gen_expr(e) ))),
                None => Code::S("return".to_string()),
            }
        },
        _ => panic!("TODO implement code gen for statements"),
    }
}

fn gen_expr( expr : Expr ) -> Code {
    match expr {
        Expr::Number(s) => {
            Code::S(s.value)
        },
        _ => panic!("TODO implement code gen for expressions"),
    }
}

