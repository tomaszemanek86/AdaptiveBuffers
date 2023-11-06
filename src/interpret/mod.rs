mod interpret;
mod structures;

use std::cell::Cell;
use std::collections::HashMap;
use std::vec::Vec;
use std::rc::Rc;
use crate::parser;

pub enum InterpretError {
    StructureRedefined(parser::Struct),
    StructureContainsItself(parser::Struct),
    UnknownType(String),
    NoMembersInStruct,
}

#[derive(is_variant::IsVariant)]
pub enum Type {
    Struct(Rc<Struct>),
    Int(parser::Int),
    UserDefined(String)
}

pub struct Member {
    name: String,
    typ: Cell<Type>,
}

pub struct Struct {
    members: Vec<Member>
}

#[derive(Default)]
pub struct Structs {
    structs: HashMap<String, Rc<Struct>>
}

#[derive(Default)]
struct Interpreter {
    structs: Structs
}

pub fn interpret(tokens: Vec<parser::SyntaxToken>) -> Result<Vec<Type>, InterpretError> {
    let mut interpreter = Interpreter::default();
    for token in tokens {
        let _ = interpreter.put_token(token)?;
    }
    interpreter.get_types()
}