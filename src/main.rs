
mod parsing;
mod code_gen;

use parsing::parse;
use parsing::parse_error::ParseError;
use code_gen::lua_gen;

fn main() -> Result<(), ParseError>{
    let s = r#"
fun blah( x : number ) {
    return 0;
}
"#;
    let m = parse::parse_module(&s)?;
    let o = lua_gen::gen_mod(m);
    println!("::{}", o);
    Ok(())
}
