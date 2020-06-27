
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_module(&mut self) -> Result<Mod, ParseError> {
        let top_level_items = self.zero_or_more(|input| input.parse_top_level() )?;
        let mut fun_defs = vec![];
        for item in top_level_items.into_iter() {
            match item {
                TopLevel::FunDef { def, public } => {
                    // TODO we're going to be using public to determing if it's exported
                    fun_defs.push(def);
                }
            }
        }
        Ok( Mod { fun_defs } )
    }
}


#[cfg(test)]
mod test {
    use super::*;

}
