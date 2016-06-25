pub use self::classfile::*;
pub use self::io::*;

pub mod classfile;
pub mod io;

/*

// --

pub enum BytecodeError {
    NotImplemented
}

pub type BytecodeResult<T> = Result<T, BytecodeError>;

// -- Stream

pub struct BytecodeReader<'a> {
    source: &'a Read,
}

impl <'a> BytecodeReader<'a> {
    pub fn new(source: &'a Read) -> BytecodeReader {
        BytecodeReader { source: source }
    }

    pub fn read_u16(&self) -> Option<u16> {
        None
    }
}

pub struct BytecodeWriter<'a> {
    target: &'a mut Write
}

impl<'a> BytecodeWriter<'a> {
    /// Create a new bytecode writer that outputs the generated bytecode to the specified target
    pub fn new(target: &'a mut Write) -> BytecodeWriter {
        BytecodeWriter { target: target }
    }

    pub fn write_bytecode<T>(&mut self, bytecode: &T) -> Result<usize, Error> where T: Bytecode {
        bytecode.write_bytecode(self)
    }

    pub fn write_u64(&mut self, value: u64) -> Result<usize, Error> {
        self.target.write(&*vec![
            ((value & 0xFF << 56) >> 56) as u8,
            ((value & 0xFF << 48) >> 48) as u8,
            ((value & 0xFF << 40) >> 40) as u8,
            ((value & 0xFF << 32) >> 32) as u8,
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ])
    }

    pub fn write_u32(&mut self, value: u32) -> Result<usize, Error> {
        self.target.write(&*vec![
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ])
    }

    pub fn write_u16(&mut self, value: u16) -> Result<usize, Error> {
        self.target.write(&*vec![ ((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8 ])
    }

    pub fn write_u8(&mut self, value: u8) -> Result<usize, Error> {
        self.target.write(&*vec![value])
    }

    pub fn write_n(&mut self, value: Vec<u8>) -> Result<usize, Error> {
        value.iter().map(|v| self.write_u8(*v)).fold(Ok(0), |acc, x| {
            match (acc, x) {
                (Ok(i), Ok(s)) => Ok(i + s),
                (e@Err(_), _) => e,
                (_, Err(err)) => Err(err)
            }
        })
    }
}

pub trait Bytecode: Sized {
    fn read_bytecode(reader: &BytecodeReader) -> Result<Self, BytecodeError>;
    fn write_bytecode(&self, writer: &mut BytecodeWriter) -> Result<usize, Error>;
}

// -- Constants --

pub struct ConstantPool {
}

impl ConstantPool {
}

pub struct ConstantPoolIndex {
    pub index: u16
}

impl Bytecode for ConstantPoolIndex {

    fn read_bytecode(reader: &BytecodeReader) -> Result<Self, BytecodeError> {
        match reader.read_u16() {
            Some(index) => Ok(ConstantPoolIndex { index: index }),
            None => Err(BytecodeError::NotImplemented)
        }
    }

    fn write_bytecode(&self, writer: &mut BytecodeWriter) -> Result<usize, Error> {
        writer.write_u16(self.index)
    }
}

// -- Attributes --

pub enum Attribute {
    ConstantValue(ConstantValue),
    Code(Code)
}

impl Bytecode for Attribute {

    fn read_bytecode(reader: &BytecodeReader) -> Result<Self, BytecodeError> {
        Err(BytecodeError::NotImplemented)
    }

    fn write_bytecode(&self, writer: &mut BytecodeWriter) -> Result<usize, Error> {
        match self {
            &Attribute::ConstantValue(ref val) => val.write_bytecode(writer),
            &Attribute::Code(ref val) => val.write_bytecode(writer)
        }
    }
}

pub struct ConstantValue {
    pub index: ConstantPoolIndex
}

impl Bytecode for ConstantValue {
    fn read_bytecode(bytes: &BytecodeReader) -> Result<Self, BytecodeError> {
        Err(BytecodeError::NotImplemented)
    }

    fn write_bytecode(&self, writer: &mut BytecodeWriter) -> Result<usize, Error> {
        Ok(0)
    }
}

pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionHandler>,
    pub attributes: Vec<Attribute>
}

impl Bytecode for Code {
    fn read_bytecode(bytes: &BytecodeReader) -> Result<Self, BytecodeError> {
        Err(BytecodeError::NotImplemented)
    }

    fn write_bytecode(&self, writer: &mut BytecodeWriter) -> Result<usize, Error> {
        Ok(0)
    }
}

pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16
}

pub struct StackMapTable {
    pub entries: Vec<StackMapFrame>
}

pub enum StackMapFrame {
    // TODO incomplete
}

pub struct Exceptions {
    pub exception_index_table: Vec<ConstantPoolIndex>
}

*/
