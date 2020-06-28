
use crate::parsing::ast::*;

pub fn gen_mod( module : Mod ) -> String { 
    module.fun_defs.into_iter().map(|def| gen_fun_def(def)).collect::<Vec<String>>().join("\n")
}

fn gen_fun_def( fun_def : FunDef ) -> String {
    fn p( params : Vec<FunParam> ) -> String {
        params.into_iter().map(|p| p.name.value).collect::<Vec<String>>().join(", ")
    }
    fn d( statements : Vec<Statement> ) -> String {
        statements.into_iter().map(|s| gen_statement(s, 0)).collect::<Vec<String>>().join("\n")
    }
    format!(r#"
function {}({})
{}
end"#, fun_def.name.value, p(fun_def.params), d(fun_def.definition))
}

fn gen_statement( statement : Statement, _indent : usize ) -> String {
    match statement {
        Statement::Return(Some(s)) => format!("return {}", gen_expr(s)),
        Statement::Return(None) => format!("return"),
        _ => panic!("TODO finish gen statement"),
    }
}

fn gen_expr( expr : Expr ) -> String {
    match expr {
        Expr::Number(s) => format!(" {} ", s.value ),
        _ => panic!("TODO finish expr code gen"),
    }
}
