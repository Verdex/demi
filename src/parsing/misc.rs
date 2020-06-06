
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    /* 

    (a,b,c)
    a
    a::b::c
    fun(a,b,c) -> d
    a<b>
    ()

    */

    fn parse_tuple_type(&mut self) -> Result<Type, ParseError> {
        
        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_fun_type(&mut self) -> Result<Type, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_namespace_type(&mut self, initial : PSym) -> Result<(Vec<PSym>, PSym), ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_index_type(&mut self, init : PSym) -> Result<Type, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }
    
    pub fn parse_type(&mut self) -> Result<Type, ParseError> {

        match self.parse_tuple_type() {
            Ok(t) => return Ok(t),
            _ => (),
        }

        match self.parse_fun_type() {
            Ok(t) => return Ok(t),
            _ => (),
        }

        let simple = self.parse_symbol()?;

        match self.expect("::") {
            Ok(_) => {
                let (namespace, symbol) = self.parse_namespace_type(simple)?;
                match self.expect("<") {
                    Ok(_) => {
                        let index_type = self.parse_index_type(symbol)?;
                        Ok(Type::Namespace(namespace, Box::new(index_type)))
                    },
                    Err(_) => Ok(Type::Namespace(namespace, Box::new(Type::Simple(symbol)))),
                }
            },
            Err(_) =>
                match self.expect("<") {
                    Ok(_) => Ok(self.parse_index_type(simple)?),
                    Err(_) => Ok(Type::Simple(simple)),
                },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sym_proj( v : PSym ) -> String {
        match v {
            PSym { value, .. } => value,
        }
    }

    #[test]
    fn should_parse_simple_type() -> Result<(), ParseError> {
        let i = "simple".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        let name = match u {
            Type::Simple(PSym { value, .. }) => value,
            _ => panic!("should be simple type"), 
        };
        assert_eq!( name, "simple" );
        Ok(())
    }

    #[test]
    fn should_parse_indexed_type() -> Result<(), ParseError> {
        let i = "simple<alpha, beta> ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        let (name, types) = match u {
            Type::Index(PSym { value, .. }, ts) => (value, ts),
            _ => panic!("should be indexed type"),
        };
        assert_eq!( name, "simple" );
        assert_eq!( types.len(), 2 );

        let i0_name = match &types[0] {
            Type::Simple(PSym { value, .. }) => value,
            _ => panic!("index 0 should be simple type"),
        };
        
        let i1_name = match &types[1] {
            Type::Simple(PSym { value, .. }) => value,
            _ => panic!("index 1 should be simple type"),
        };

        assert_eq!( i0_name, "alpha" );
        assert_eq!( i1_name, "beta" );

        Ok(())
    }

    #[test]
    fn should_parse_namespace_type() -> Result<(), ParseError> {
        let i = "mod1::mod2::Trait::Type ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        let (names, t) = match u {
            Type::Namespace(ns, t) => (ns, t),
            _ => panic!("should be namespace type"),
        };

        assert_eq!( names.len(), 3 );
        assert_eq!( sym_proj(names[0]), "mod1" );
        assert_eq!( sym_proj(names[1]), "mod2" );
        assert_eq!( sym_proj(names[2]), "Trait" );

        let st_name = match *t {
            Type::Simple(PSym { value, .. }) => value,
            _ => panic!("type should be simple type"),
        };

        assert_eq!( st_name, "Type" );

        Ok(())
    }

    #[test]
    fn should_parse_unit_type() -> Result<(), ParseError> {
        let i = "() ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        match u {
            Type::Unit => Ok(()),
            _ => panic!("should be unit type"),
        }
    }

    #[test]
    fn should_parse_tuple_type() -> Result<(), ParseError> {
        let i = "(alpha, beta, gamma) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let types = match u {
            Type::Tuple(ts) => ts, 
            _ => panic!("should be tuple type"),
        };

        assert_eq!( types.len(), 3 );

        let t1_name = match &types[0] {
            Type::Simple(PSym { value, .. }) => value,
            _ => panic!("t1 should be simple type"),
        };

        assert_eq!( t1_name, "alpha" );
        Ok(())
    }

    #[test]
    fn should_parse_arrow_type() -> Result<(), ParseError> {
        let i = "alpha -> beta ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input, output) = match u {
            Type::Arrow { input, output } => (input, output), 
            _ => panic!("should be arrow type"),
        };

        let i_name = match *input {
            Type::Simple(PSym { value, .. }) => value,
            _ => panic!("input type should be simple"),
        };

        assert_eq!( i_name, "alpha" );

        let o_name = match *output {
            Type::Simple(PSym { value, .. }) => value,
            _ => panic!("input type should be simple"),
        };

        assert_eq!( o_name, "beta" );
        Ok(())
    }

    #[test]
    fn should_parse_paren_type() -> Result<(), ParseError> {
        let i = "(((alpha))) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let name = match u {
            Type::Simple(PSym { value, .. }) => value, 
            _ => panic!("should be simple type"),
        };

        assert_eq!( name, "alpha" );
        Ok(())
    }

    #[test]
    fn should_parse_arrow_past_arrow_parameter() -> Result<(), ParseError> {
        let i = "a -> (b -> c) -> d ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;


        let (input_a, output_bc_etc) = match u {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("should be arrow type, but found: {:?}", x),
        };

        let name = match input_a {
            Type::Simple( PSym { value, .. }) => value,
            x => panic!("first input should be simple type, but found: {:?}", x),
        };

        assert_eq!( name, "a" );

        let (input_bc, output_d) = match output_bc_etc {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("first output should be arrow type, but found: {:?}", x),
        };

        let (input_b, output_c) = match input_bc {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("second input should be arrow type, but found {:?}", x),
        };

        let name = match input_b {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("second input input should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let name = match output_c {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("second input output should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "c" );

        let name = match output_d {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("final output should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "d" );

        Ok(())
    }

    #[test]
    fn should_parse_paren_arrows() -> Result<(), ParseError> {
        let i = "a -> b -> (c -> d) -> ((e -> f) -> g) -> i ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input_a, output_b_etc) = match u {
            Type::Arrow{ input, output } => (*input, *output), 
            x => panic!("should be arrow type, but found {:?}", x),
        };

        let name = match input_a {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("input_a should be simple type, but found: {:?}", x),
        };

        assert_eq!(name, "a");

        let (input_b, output_cd_etc) = match output_b_etc {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("input_b_etc should be arrow type, but found: {:?}", x),
        };

        let name = match input_b {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("input_b should be simple type, but found: {:?}", x),
        };
        
        assert_eq!(name, "b");

        let (input_cd, output_efg_etc) = match output_cd_etc {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("output_cd_etc should be arrow type, but found: {:?}", x),
        };

        let (input_c, output_d) = match input_cd {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("input_cd should be arrow type, but found: {:?}", x),
        };

        let name = match input_c {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("input_c should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "c" );

        let name = match output_d {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("output_d should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "d" );
        
        let (input_efg, output_i) = match output_efg_etc {
            Type::Arrow{input, output} => (*input, *output),
            x => panic!("input_efg_etc should be arrow type, but found {:?}", x),
        };

        let (input_ef, output_g) = match input_efg {
            Type::Arrow{input, output} => (*input, *output),
            x => panic!("input_efg should be arrow type, but found {:?}", x),
        };

        let (input_e, output_f) = match input_ef {
            Type::Arrow{input, output} => (*input, *output),
            x => panic!("input_ef should be arrow type, but found {:?}", x),
        };

        let name = match input_e {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("input_e should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "e" );

        let name = match output_f {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("output_f should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "f" );

        let name = match output_g {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("output_g should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "g" );

        let name = match output_i {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("output_i should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "i" );

        Ok(())
    }

    #[test]
    fn should_parse_complex_tuple() -> Result<(), ParseError> {
        let i = "(a -> b, c::d::e, (), i<j,k,l>, (m, n)) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let mut types = match u {
            Type::Tuple(types) => types, 
            _ => panic!("should be tuple type"),
        };

        assert_eq!( types.len(), 5 );

        let one = types.remove(0);

        let (one_input, one_output) = match one {
            Type::Arrow{input, output} => (*input, *output),   
            x => panic!("one should be arrow type, but found {:?}", x),
        };

        let name = match one_input {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("one_input should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "a" );

        let name = match one_output {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("one_output should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let two = types.remove(0);
        
        let (names, t) = match two {
            Type::Namespace(ns, t) => (ns, *t),
            x => panic!("two should be namespace type, but found {:?}", x),
        };
        
        assert_eq!( names.len(), 2 );
        assert_eq!( sym_proj(names[0]), "c" );
        assert_eq!( sym_proj(names[1]), "d" );

        let name = match t {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("t should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "e" );

        let three = types.remove(0);

        assert_eq!( matches!( three, Type::Unit ), true );

        let four = types.remove(0);

        let (name, mut ts) = match four {
            Type::Index(PSym { value, .. }, ts) => (value, ts),
            x => panic!("four should be indexed type, but found {:?}", x),
        };

        assert_eq!( name, "i" );

        assert_eq!( ts.len(), 3 );

        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!( "index_one should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "j" );

        let index_two = ts.remove(0);

        let name = match index_two {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!( "index_two should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "k" );

        let index_three = ts.remove(0);

        let name = match index_three {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!( "index_three should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "l" );

        let five = types.remove(0);

        let mut ts = match five {
            Type::Tuple(ts) => ts,
            x => panic!( "five should be tuple type, but found {:?}", x),
        };

        assert_eq!( ts.len(), 2 );
        
        let tuple_one = ts.remove(0);

        let name = match tuple_one {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!( "tuple_one should be simple type but found {:?}", x),
        };

        assert_eq!( name, "m" );

        let tuple_two = ts.remove(0);

        let name = match tuple_two {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!( "tuple_two should be simple type but found {:?}", x),
        };

        assert_eq!( name, "n" );
        Ok(())
    }

    #[test]
    fn should_parse_index_namespace() -> Result<(), ParseError> {
        let i = "a::e<f> ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        
        let (names, t) = match u {
            Type::Namespace(names, t) => (names, *t),
            x => panic!("should be namespace type, but found {:?}", x),
        };

        assert_eq!( names.len(), 1 );
        assert_eq!( sym_proj(names[0]), "a" );

        let (name, mut ts) = match t {
            Type::Index(PSym { value, .. }, ts) => (value, ts),
            x => panic!("t should be indexed type, but found {:?}", x),
        };

        assert_eq!( name, "e" );

        assert_eq!( ts.len(), 1 );
        
        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("index_one should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "f" );

        Ok(())
    }

    #[test]
    fn should_parse_indexed_arrow_param() -> Result<(), ParseError> {
        let i = "a<b> -> c<d>".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input, output) = match u {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("should be arrow type, but found {:?}", x),
        };

        let (name, mut ts) = match input {
            Type::Index(PSym { value, .. }, ts) => (value, ts),
            x => panic!("input should be index type, but found {:?}", x),
        };

        assert_eq!( name, "a" );

        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("index_one should be index type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let (name, mut ts) = match output {
            Type::Index(PSym { value, .. }, ts) => (value, ts),
            x => panic!("output should be index type, but found {:?}", x),
        };

        assert_eq!( name, "c" );

        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("index_one should be index type, but found {:?}", x),
        };

        assert_eq!( name, "d" );

        Ok(())
    }

    #[test]
    fn should_parse_namespace_arrow_param() -> Result<(), ParseError> {
        let i = "a::b -> c::d ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input_ab, output_cd) = match u {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("should be arrow type, but found {:?}", x),
        };

        let (names, t) = match input_ab {
            Type::Namespace(ns, t) => (ns, *t),
            x => panic!("input_ab should be indexed type, but found {:?}", x),
        };

        assert_eq!( names.len(), 1 );
        assert_eq!( sym_proj(names[0]), "a" );

        let name = match t {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("t should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let (names, t) = match output_cd {
            Type::Namespace(ns, t) => (ns, *t),
            x => panic!("output_cd should be indexed type, but found {:?}", x),
        };

        assert_eq!( names.len(), 1 );
        assert_eq!( sym_proj(names[0]), "c" );

        let name = match t {
            Type::Simple(PSym { value, .. }) => value,
            x => panic!("t should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "d" );

        Ok(())
    }
}

