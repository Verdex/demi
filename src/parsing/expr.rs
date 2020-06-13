
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;
use super::misc;

impl<'a> Input<'a> {

    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.choice( &[ |input| Ok(Expr::Number(input.parse_number()?))
                      , |input| Ok(Expr::PString(input.parse_string()?))
                      ] )
    }

}

#[cfg(test)]
mod test {
    use super::*;

}
