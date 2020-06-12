
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;
use super::input::misc;

impl<'a> Input<'a> {

    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {

    }

}

#[cfg(test)]
mod test {
    use super::*;

}
