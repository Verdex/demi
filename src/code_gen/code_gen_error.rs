
#[derive(Debug)]
pub enum CodeGenError {
    LuaGenAt(usize, String),
    CodeGenErrorAt(usize, String),
}

