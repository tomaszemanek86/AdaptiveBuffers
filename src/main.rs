mod code_view;
mod data_view;
mod generator;
mod int;
mod interpret;
mod memory_details;
mod native_type;
mod parser;
mod struct_member_constant_memory;
mod struct_memory;
mod view_memory;
mod enum_member_ref_memory;
mod view_posibility_constant_memory;
mod memory_type;
mod enum_memory;

use clap::Parser;
use std::{fmt::Display, fs, process::exit, rc::Rc, cell::RefCell};

#[derive(Clone, Default, Debug)]
pub struct CodeView {
    pub origin: Rc<String>,
    pub from: usize,
    pub to: usize,
}

#[derive(Default, Debug, Clone)]
pub struct DataView<TData: Default + Clone> {
    pub data: TData,
    pub code_view: CodeView,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Int {
    pub signed: bool,
    pub bytes: u8,
}

#[derive(Debug, Clone)]
pub struct ViewMemberKeyReferenceMemory {
    pub code_view: CodeView,
    pub member: Rc<StructMemberMemory>,
}

#[derive(variation::Variation, Debug, Clone)]
pub enum StructMemberConstantMemory {
    ViewMemberKey(ViewMemberKeyReferenceMemory)
}

#[derive(Debug, Clone)]
pub enum Language {
    Cpp,
    C,
    Python,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C => write!(f, "C"),
            Self::Cpp => write!(f, "Cpp"),
            Self::Python => write!(f, "Python"),
        }
    }
}

#[derive(Debug)]
pub struct StructMemberMemory {
    pub name: String,
    pub index: usize,
    pub memory: RefCell<Memory>
}

#[derive(Debug)]
pub struct StructMemory {
    name: String,
    fields: Vec<Rc<StructMemberMemory>>,
}

#[derive(Debug)]
pub struct EnumMemberRefMemory {
    enum_typ: Rc<EnumMemory>,
    index: usize
}

#[derive(Debug, variation::Variation)]
pub enum ViewPosibilityConstantMemory {
    Default(usize),
    Usize(usize),
    EnumMemberRef(EnumMemberRefMemory),
}

#[derive(Debug)]
pub struct ViewPosibilityMemory {
    memory: MemoryType,
    constant: ViewPosibilityConstantMemory,
}

#[derive(Debug)]
pub struct ViewMemory {
    name: String,
    types: Vec<ViewPosibilityMemory>,
}

#[derive(Debug, Clone)]
pub struct EnumConstantMemory {
    name: String,
    value: usize,
}

#[derive(Debug, Clone)]
pub struct EnumMemory {
    name: String,
    underlaying_type: NativeType,
    constants: Vec<EnumConstantMemory>,
}

#[derive(Debug, Clone, variation::Variation)]
pub enum NativeType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Unknown,
    ViewKeyReference(Rc<StructMemberMemory>),
    ArrayDimensionReference(Rc<StructMemberMemory>),
}

trait ExactSize {
    fn exact_size(&self) -> Option<usize>;
}

trait KnownSize {
    fn known_size(&self) -> Option<usize>;
}

trait MaxSize {
    fn max_size(&self) -> usize;
}

trait IsDirectlyDeserializable {
    fn is_directly_deserializable(&self) -> bool;
}

pub struct MemoryDeclaration {
    pub name: String,
    pub memory: Memory,
}

pub trait MemoryDetails {
    fn exact_size(&self) -> Option<usize>;
    fn max_size(&self) -> Option<usize>;
    fn buffer_size(&self) -> Option<usize>;
    fn submembers(&self) -> usize;
    fn is_sized(&self) -> bool {
        self.exact_size().is_some()
    }
}

#[derive(Debug)]
pub struct Memory {
    memory: MemoryType,
    max_array_size: Option<u16>
}

#[derive(Debug, variation::Variation)]
pub enum MemoryType {
    Native(NativeType),
    Struct(StructMemory),
    View(ViewMemory),
    Enum(Rc<EnumMemory>)
}

fn generate_memory(content: String) -> Vec<MemoryDeclaration> {
    let tokens = parser::parse(content)
        .or_else(|e| -> Result<Vec<parser::SyntaxToken>, String> {
            log::error!("parse error: {}", e.to_string());
            exit(1);
        })
        .unwrap();
    interpret::interpret(tokens)
        .or_else(|e| -> Result<Vec<MemoryDeclaration>, String> {
            log::error!("interpreting failed: {}", e.to_string());
            exit(1);
        })
        .unwrap()
}

fn generate_cpp(memory: Vec<MemoryDeclaration>, args: &Args) {
    if let Err(e) = generator::generate(&memory, args) {
        log::error!("generator error: {}", e.to_string());
        exit(1)
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "c" => Language::C,
            "cpp" => Language::Cpp,
            "python" => Language::Python,
            _ => {
                log::error!("Unknown language '{value}'");
                exit(1);
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Input <***.psd> file from which serialization/deserialization library will be generated.
    #[arg(short, long)]
    protofile: String,

    // Target language (cpp,c,python).
    #[arg(short, long)]
    language: Language,

    // Output directory where library will be generated.
    #[arg(short, long)]
    output_dir: String,
}

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    let args = Args::parse();
    simple_logger::SimpleLogger::new()
        .without_timestamps()
        .init()
        .unwrap();
    log::info!("Protofile: {}", &args.protofile);
    let language = Language::from(args.language.clone());
    log::info!("Language: {}", args.language);
    let content = fs::read_to_string(args.protofile.as_str())
        .or_else(|_| -> Result<String, String> {
            log::error!("Unable to read {}", &args.protofile);
            exit(1);
        })
        .unwrap();
    let memory = generate_memory(content);
    match language {
        Language::Cpp => generate_cpp(memory, &args),
        Language::C => todo!(),
        Language::Python => todo!(),
    }
}

#[cfg(test)]
mod test;
