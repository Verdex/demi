
mod parsing;
mod code_gen;

use parsing::parse;
use parsing::parse_error::ParseError;
use code_gen::lua_gen;

fn main() -> Result<(), ParseError>{
    let s = r#""#;
    let m = parse::parse_module(&s)?;
    let o = lua_gen::gen_mod(m);
    println!("{}", o);
    Ok(())
}
