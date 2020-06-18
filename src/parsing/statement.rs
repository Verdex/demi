
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;
use super::misc;

impl<'a> Input<'a> {

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        Err(ParseError::EndOfFile("".to_string()))
    }

}

#[cfg(test)]
mod test {
    use super::*;

}
