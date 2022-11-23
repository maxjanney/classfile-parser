use crate::{U1, U2};

#[derive(Debug, Clone)]
pub enum ConstantPoolType {
    Class {
        name_index: U2,
    },
    Fieldref {
        class_index: U2,
        name_and_type_index: U2,
    },
    Methodref {
        class_index: U2,
        name_and_type_index: U2,
    },
    InterfaceMethodref {
        class_index: U2,
        name_and_type_index: U2,
    },
    String {
        string_index: U2,
    },
    Integer {
        bytes: [U1; 4],
    },
    Float {
        bytes: [U1; 4],
    },
    Long {
        val: [U1; 8],
    },
    Double {
        val: [U1; 8],
    },
    NameAndType {
        name_index: U2,
        descriptor_index: U2,
    },
    Utf8 {
        bytes: Vec<U1>,
    },
    MethodHandle {
        reference_kind: U1,
        reference_index: U2,
    },
    MethodType {
        descriptor_index: U2,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: U2,
        name_and_type_index: U2,
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
}

impl From<u8> for ConstantPoolTag {
    fn from(b: u8) -> Self {
        match b {
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
            _ => unreachable!(),
        }
    }
}

pub fn get_utf8(constant_pool: &[ConstantPoolType], index: usize) -> &[u8] {
    match constant_pool.get(index) {
        Some(ConstantPoolType::Utf8 { bytes }) => bytes,
        _ => unreachable!(),
    }
}
