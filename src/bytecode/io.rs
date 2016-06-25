use std::io::{ Read, Write, Error, ErrorKind };
use super::classfile::*;

pub struct ClassReader {
}

impl ClassReader {

    pub fn read_class<T>(source: &mut T) -> Result<Classfile, Error> where T: Read {
        let mut reader = BlockReader::new(source);

        let fns: Vec<fn(&mut BlockReader, &ClassFragment) -> Result<ClassFragment, Error>> = vec![
            ClassReader::read_magic_bytes,
            ClassReader::read_classfile_version
        ];

        let result = fns.iter().fold(Ok(ClassFragment::default()), |acc, x| {
            match acc {
                Ok(acc_fragment) => match x(&mut reader, &acc_fragment) {
                    Ok(cur_fragment) => Ok(acc_fragment.merge(cur_fragment)),
                    err@_ => err
                },
                err@_ => err
            }
        });

        match result {
            Ok(fragment) => Ok(fragment.to_class()),
            Err(err) => Err(err)
        }
    }

    fn read_magic_bytes(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match reader.read_u32() {
            Ok(0xCAFEBABE) => Ok(ClassFragment::default()),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid magic bytes"))
        }
    }

    fn read_classfile_version(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match (reader.read_u16(), reader.read_u16()) {
            (Ok(minor_version), Ok(major_version)) => {
                Ok(ClassFragment {
                    version: Some(ClassfileVersion::new(major_version, minor_version)),
                    ..Default::default()
                })
            },
            _ => Err(Error::new(ErrorKind::UnexpectedEof, "Could not read classfile version number"))
        }
    }

    fn read_constant_pool(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        Ok(ClassFragment::default())
    }

}

struct BlockReader<'a> {
    source: &'a mut Read
}

impl<'a> BlockReader<'a> {

    pub fn new<T>(source: &'a mut T) -> BlockReader where T: Read {
        BlockReader { source: source }
    }

    #[allow(dead_code)] // there is some chance this method will never be used
    pub fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buf: [u8; 8] = [0; 8];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(
                ((buf[0] as u64) << 56) +
                ((buf[1] as u64) << 48) +
                ((buf[2] as u64) << 40) +
                ((buf[3] as u64) << 32) +
                ((buf[4] as u64) << 24) +
                ((buf[5] as u64) << 16) +
                ((buf[6] as u64) << 8) +
                buf[7] as u64),
            Err(err) => Err(err)
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buf: [u8; 4] = [0; 4];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(
                ((buf[0] as u32) << 24) +
                ((buf[1] as u32) << 16) +
                ((buf[2] as u32) << 8) +
                buf[3] as u32),
            Err(err) => Err(err)
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let mut buf: [u8; 2] = [0; 2];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(((buf[0] as u16) << 8) + buf[1] as u16),
            Err(err) => Err(err)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0; 1];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(buf[0]),
            Err(err) => Err(err)
        }
    }
}

pub struct ClassWriter<'a> {
    target: &'a mut Write
}

impl<'a> ClassWriter<'a> {
    pub fn new<T>(target: &'a mut T) -> ClassWriter where T: Write {
        ClassWriter { target: target }
    }

    pub fn write_class(&mut self, classfile: &Classfile) -> Result<usize, Error> {
        self.write_magic_bytes()
        .and(self.write_classfile_version(&classfile.version))
    }

    pub fn write_magic_bytes(&mut self) -> Result<usize, Error> {
        self.write_u32(0xCAFEBABE)
    }

    pub fn write_classfile_version(&mut self, version: &ClassfileVersion) -> Result<usize, Error> {
        self.write_u16(version.minor_version)
        .and(self.write_u16(version.major_version))
    }

    pub fn write_constant_pool(&mut self, cp: &ConstantPool) -> Result<usize, Error> {
        Ok(0)
    }

    pub fn write_u64(&mut self, value: u64) -> Result<usize, Error> {
        let buf: [u8; 8] = [
            ((value & 0xFF << 56) >> 56) as u8,
            ((value & 0xFF << 48) >> 48) as u8,
            ((value & 0xFF << 40) >> 40) as u8,
            ((value & 0xFF << 32) >> 32) as u8,
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ];

        self.target.write(&buf)
    }

    pub fn write_u32(&mut self, value: u32) -> Result<usize, Error> {
        let buf: [u8; 4] = [
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ];

        self.target.write(&buf)
    }

    pub fn write_u16(&mut self, value: u16) -> Result<usize, Error> {
        let buf: [u8; 2] = [((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8];

        self.target.write(&buf)
    }

    pub fn write_u8(&mut self, value: u8) -> Result<usize, Error> {
        self.target.write(&[value])
    }
}

struct ClassFragment {
    pub version: Option<ClassfileVersion>,
    pub constant_pool: Option<ConstantPool>
}

impl ClassFragment {
    pub fn merge(mut self, other: Self) -> Self {
        self.version = other.version.or(self.version);
        self.constant_pool = other.constant_pool.or(self.constant_pool);
    //    self.access_flags = other.access_flags.or(self.access_flags);
    //    self.this_class = other.this_class.or(self.this_class);
    //    self.super_class = other.super_class.or(self.super_class);
    //    self.interfaces = other.interfaces.or(self.interfaces);
    //    self.fields = other.fields.or(self.fields);
    //    self.methods = other.methods.or(self.methods);
    //    self.attributes = other.attributes.or(self.attributes);
        self
    }

    /// Transform this class fragment into a final class file. Members set on the fragment will
    /// be defined on the class too, other members will be initialized with their default values
    pub fn to_class(self) -> Classfile {
        Classfile {
            version: self.version.unwrap_or(ClassfileVersion::default()),
    //        constant_pool: self.constant_pool.unwrap_or(ConstantPool::default()),
    //        access_flags: self.access_flags.unwrap_or(AccessFlags::new()),
    //        this_class: self.this_class.unwrap_or(ConstantPoolIndex::new()),
    //        super_class: self.super_class.unwrap_or(ConstantPoolIndex::new()),
    //        interfaces: self.interfaces.unwrap_or(vec![]),
    //        fields: self.fields.unwrap_or(vec![]),
    //        methods: self.methods.unwrap_or(vec![]),
    //        attributes: self.attributes.unwrap_or(vec![])
        }
    }
}

impl Default for ClassFragment {
    fn default() -> Self {
        ClassFragment {
            version: None,
            constant_pool: None
        }
    }
}
