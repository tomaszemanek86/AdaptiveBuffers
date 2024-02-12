mod code_view;
mod data_view;
mod generator;
mod int;
mod interpret;
mod memory_details;
mod native_type;
mod parser;
mod struct_member_constant_memory;
mod struct_member_memory;
mod struct_memory;
mod view_memory;
mod enum_member_ref_memory;
mod view_posibility_constant_memory;
mod memory_type;
mod enum_memory;
mod array_size;
mod bit_mask;

use clap::Parser;
use interpret::InterpretError;
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
    pub code_view: Option<CodeView>,
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
    Unknown
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cpp => write!(f, "Cpp"),
            Self::Unknown => panic!("unknown language")
        }
    }
}

#[derive(Debug)]
pub struct StructMemberMemory {
    pub name: String,
    pub index: usize,
    pub memory: RefCell<Memory>,
    pub structure: Rc<RefCell<StructMemory>>,
    pub parsed: DataView<parser::Struct>,
}

#[derive(Debug)]
pub struct StructMemory {
    name: String,
    fields: Vec<Rc<StructMemberMemory>>,
    parsed: DataView<parser::Struct>
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
    underlaying_type: Native,
    constants: Vec<EnumConstantMemory>,
}

#[derive(Debug, Clone)]
pub struct ArrayDimensionReference {
    origin: Rc<Native>,
    size: Rc<StructMemberMemory>,
    array: Rc<StructMemberMemory>
}

#[derive(Debug, Clone)]
pub struct ViewKeyReference {
    native_key: Rc<Native>,
    key: Rc<StructMemberMemory>,
    view: Rc<StructMemberMemory>
}

#[derive(Debug, Clone)]
pub struct StructMemberSizeReference {
    native: Rc<Native>,
    origin: Rc<StructMemberMemory>,
    member: Rc<StructMemberMemory>
}

#[derive(Debug, Clone, variation::Variation)]
pub enum SizeArithmetics {
    StructMemberSizeReference(Rc<StructMemberMemory>),
    StructMemberValueReference(Rc<StructMemberMemory>),
    Plus,
    Minus,
    Usize(usize)
}

#[derive(Debug, Clone)]
pub struct StructMemberSizeArithmetics {
    native: Rc<Native>,
    arithmetics: Vec<SizeArithmetics>
}

#[derive(Debug, Clone, variation::Variation)]
enum BitArithmetic {
    Not,
    And,
    Value(usize),
}

#[derive(Debug, Clone, Default)]
struct Bits {
    name: String,
    bits: Vec<DataView<BitArithmetic>>
}

#[derive(Debug, Clone, Default)]
pub struct BitMask {
    name: String,
    native: Native,
    bits: Vec<Bits>
}


#[derive(Debug, Clone, Default)]
pub struct Native {
    typ: NativeType,
    endian: Option<bool>
}

#[derive(Debug, Clone, variation::Variation)]
pub enum NativeType {
    Bool,
    U8,
    U16,
    U24,
    U32,
    U64,
    NoSwapU8,
    NoSwapU16,
    NoSwapU24,
    NoSwapU32,
    NoSwapU64,
    ConstU8(u8),
    ConstU16(u16),
    ConstU24(u32),
    ConstU32(u32),
    ConstU64(u64),
    I8,
    I16,
    I32,
    I64,
    Unknown(DataView<Int>),
    ViewKeyReference(ViewKeyReference),
    ArrayDimensionReference(ArrayDimensionReference),
    StructMemberSize(StructMemberSizeReference),
    StructMemberSizeArithmetics(StructMemberSizeArithmetics),
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
    fn is_sized(&self) -> bool {
        self.exact_size().is_some()
    }
}

#[derive(Clone, variation::Variation, PartialEq, Debug)]
pub enum ArraySize {
    No,
    Dyn,
    Exact(u32)
}

#[derive(Debug)]
pub struct Memory {
    memory: MemoryType,
    array_size: ArraySize
}

#[derive(Debug, variation::Variation)]
pub enum MemoryType {
    Native(Native),
    BitMask(Rc<BitMask>),
    Struct(Rc<RefCell<StructMemory>>),
    View(ViewMemory),
    Enum(Rc<EnumMemory>)
}

pub struct MemoryImage {
    big_endian: bool,
    memory_decl: Vec<MemoryDeclaration>
}

fn interpet_memory(content: String) -> Result<MemoryImage, InterpretError> {
    let tokens = parser::parse(content)
        .or_else(|e| -> Result<Vec<parser::SyntaxToken>, String> {
            log::error!("parse error: {}", e.to_string());
            Err(e.to_string())
        })
        .unwrap();
    interpret::interpret(tokens)
}

fn generate_cpp(memory_image: MemoryImage, args: &Args) {
    if let Err(e) = generator::generate(memory_image, args) {
        log::error!("generator error: {}", e.to_string());
        exit(1)
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cpp" => Language::Cpp,
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

    // Target language (only cpp supported for now).
    #[arg(short, long)]
    language: Language,

    // target machine endianess (big or little)
    #[arg(short, long)]
    endian: String,

    // Output directory where library will be generated.
    #[arg(short, long)]
    output_dir: String,
}

fn cpp_ptr_size() -> usize {
    std::mem::size_of::<usize>()
}

fn main() {
    let args = Args::parse();
    let logger = simple_logger::SimpleLogger::new()
        .without_timestamps();
    log::set_max_level(logger.max_level());
    logger.init()
        .unwrap();
    log::info!("Protofile: {}", &args.protofile);
    let language = Language::from(args.language.clone());
    log::info!("Language: {}", args.language);
    let content = fs::read_to_string(args.protofile.as_str())
        .or_else(|_| -> Result<String, String> {
            log::error!("Unable to read {}", &args.protofile);
            exit(1)
        })
        .unwrap();
    let memory_image = interpet_memory(content);
    if memory_image.is_err() {
        log::error!("interpreting failed: {}", memory_image.err().unwrap().to_string());
        return
    }
    match language {
        Language::Cpp => generate_cpp(memory_image.unwrap(), &args),
        _ => panic!("unexpected langage")
    }
}

