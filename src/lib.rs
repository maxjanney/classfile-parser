pub type u1 = u8;
pub type u2 = u16;
pub type u4 = u32;

pub type ConstantPool = Vec<ConstantPoolType>;

#[derive(Debug)]
pub struct ClassFile {
    magic: u4,
    version: Version,
    constant_pool: ConstantPool,
}

#[derive(Debug)]
pub struct Version {
    minor: u2,
    major: u2,
}

#[derive(Debug, Clone)]
pub enum ConstantPoolType {
    Class {
        name_index: u2,
    },
    Fieldref {
        class_index: u2,
        name_and_type_index: u2,
    },
    Methodref {
        class_index: u2,
        name_and_type_index: u2,
    },
    InterfaceMethodref {
        class_index: u2,
        name_and_type_index: u2,
    },
    String {
        string_index: u2,
    },
    Integer {
        bytes: [u1; 4],
    },
    Float {
        bytes: [u1; 4],
    },
    Long {
        val: [u1; 8],
    },
    Double {
        val: [u1; 8],
    },
    NameAndType {
        name_index: u2,
        descriptor_index: u2,
    },
    Utf8 {
        bytes: Vec<u1>,
    },
    MethodHandle {
        reference_kind: u1,
        reference_index: u2,
    },
    MethodType {
        descriptor_index: u2,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u2,
        name_and_type_index: u2,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ConstantPoolTag {
    Class,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    InvokeDynamic,
    Unknown,
}

impl From<u8> for ConstantPoolTag {
    fn from(byte: u8) -> Self {
        match byte {
            9 => Self::Fieldref,
            7 => Self::Class,
            10 => Self::Methodref,
            11 => Self::InterfaceMethodref,
            8 => Self::String,
            3 => Self::Integer,
            4 => Self::Float,
            5 => Self::Long,
            6 => Self::Double,
            12 => Self::NameAndType,
            1 => Self::Utf8,
            15 => Self::MethodHandle,
            16 => Self::MethodType,
            18 => Self::InvokeDynamic,
            _ => Self::Unknown,
        }
    }
}
