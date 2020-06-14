
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_top_level(&mut self) -> Result<TopLevel, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

}


#[cfg(test)]
mod test {
    use super::*;

}
