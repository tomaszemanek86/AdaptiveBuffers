mod as_memory;
mod enumeration;
mod error;
mod interpret;
mod structure;
mod type_variant;
mod types;
mod view;

use super::*;
use crate::parser::{self};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

#[derive(Debug)]
pub enum InterpretError {
    TypeContainsItself(DataView<parser::Struct>),
    UnknownType(DataView<String>),
    CyclicalReference(Vec<String>),
    StructMemberNotUnique,
    UnknownStructMemberReference(CodeView),
    MemberReferenceDoesntPointToView(CodeView),
    MemberReferenceDoesntPointToArray(CodeView),
    StructMemberConstantCanBeApliedOnlyForInt(CodeView),
    ViewReferenceTypeTooSmall(CodeView),
    UnknownIntSize(u8),
    EnumAlreadyExists(DataView<parser::Enum>),
    EnumConstantNotUnique(CodeView),
    EnumConstantValueNotUnique(CodeView),
    EnumConstantValueNotFitting(CodeView),
    UnknownEnumMember(CodeView),
    UnknownEnum(CodeView),
    ViewAlreadyExists(CodeView),
    BitMaskAlreadyExists(String, CodeView),
    ViewItemNotUniqueWithinView(CodeView),
    ViewReferenceKeyIsTooBig(CodeView),
    ViewEmpty(String),
    VievConstantsMustBeAllEnumsOrAllIntsOrAllUndefined,
    EndianNotSet,
    EndianOverrided(CodeView, CodeView),
    GenericError(String),
    GenericWithPosError(CodeView, String),
    CannotAsignUsizeCstToNonUnsignedMemory(usize),
    ExpectedOperator(CodeView),
    ExpectedMemberSize(CodeView),
    MemberValueNoUnsigned(CodeView),
    InvalidBitExpression(CodeView),
    NotSingleBitValue(CodeView)
}

#[derive(variation::Variation, Clone)]
pub enum TypeVariant {
    Struct(Rc<RefCell<DataView<Struct>>>),
    Enum(Rc<DataView<Enum>>),
    BitMask(Rc<DataView<BitMask>>),
    View(Rc<RefCell<DataView<View>>>),
    Int(DataView<Int>),
    Unknown(DataView<String>),
}

#[derive(Clone)]
pub struct Type {
    typ: TypeVariant,
    array_size: ArraySize
}

#[derive(Clone)]
pub struct ViewPosibility {
    typ: Type,
    constant: Option<parser::ViewConstantValue>
}

#[derive(Default, Clone)]
pub struct View {
    name: String,
    types: Vec<ViewPosibility>
}

#[derive(Clone, variation::Variation)]
pub enum StructMemberConstant {
    ViewReferenceKey(parser::MemberReference),
    ArrayDimension(parser::MemberReference),
    Usize(usize),
    Size(parser::MemberReference),
    EnumMemberValue(parser::EnumMemberRef),
    SizeArithmetics(Vec<DataView<parser::SizeArithmetics>>),
}

#[derive(Clone)]
pub struct StructMember {
    name: DataView<String>,
    index: usize,
    typ: Type,
    constant: Option<StructMemberConstant>,
}

#[derive(Default, Clone)]
pub struct Struct {
    name: DataView<String>,
    members: Vec<StructMember>,
}

#[derive(Clone, Default)]
pub struct EnumConstant {
    name: String,
    value: usize,
}

#[derive(Default, Clone)]
pub struct Enum {
    name: String,
    underlaying_int: Int,
    constants: Vec<DataView<EnumConstant>>,
}

#[derive(Default, Clone)]
pub struct Types {
    types: HashMap<String, TypeVariant>,
    order: Vec<String>,
}

impl Types {
    fn get_type(&self, name: &str) -> Result<Option<TypeVariant>, InterpretError> {
        match self.types.get(name) {
            Some(t) => {
                if t.is_unknown() {
                    Err(InterpretError::UnknownType(t.as_unknown().unwrap().clone()))
                } else {
                    Ok(Some(t.clone()))
                }
            }
            None => Ok(None),
        }
    }

    fn get_enum_member_value(
        &self,
        enum_name: &str,
        enum_member: &str,
    ) -> Result<usize, InterpretError> {
        if let Some(enum_type) = self.get_type(enum_name)? {
            if let TypeVariant::Enum(e) = enum_type {
                for constant in &e.constants {
                    if constant.name == enum_member {
                        return Ok(constant.value);
                    }
                }
                return Err(InterpretError::UnknownEnumMember(
                    e.code_view()
                ));
            }
        }
        panic!("enum not found")
    }
}

#[derive(Default)]
struct Interpreter {
    types: Types,
    order: Vec<String>,
    big_endian: Option<DataView<bool>>,
    required_version: Option<[u8; 3]>,
}

pub trait AsMemory {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError>;
}

pub fn interpret(
    tokens: Vec<parser::SyntaxToken>,
) -> Result<MemoryImage, InterpretError> {
    let mut interpreter = Interpreter::default();
    interpreter = interpreter.interpret(tokens)?;
    Ok(MemoryImage {
        big_endian: interpreter.big_endian()?,
        memory_decl: interpreter.get_memory()?,
    })
}
