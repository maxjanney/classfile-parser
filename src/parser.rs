use nom::{
    bytes::streaming::take,
    multi::count,
    number::streaming::{be_u16, be_u32, be_u8},
    IResult,
};

use crate::{Attribute, ClassFile, ConstantPoolTag, ConstantPoolType, FieldInfo, Version};

pub fn class_file(input: &[u8]) -> IResult<&[u8], ClassFile> {
    // magic
    let (input, _) = be_u32(input)?;
    // version
    let (input, version) = version(input)?;
    // constant pool
    let (input, constant_pool) = constant_pool(input)?;
    // access flags
    let (input, access_flags) = be_u16(input)?;
    // this class
    let (input, this_class) = be_u16(input)?;
    // super class
    let (input, super_class) = be_u16(input)?;
    // interfaces
    let (input, interfaces) = interfaces(input)?;
    // fields
    let (input, fields) = fields(input, &constant_pool)?;
    todo!()
}

fn version(input: &[u8]) -> IResult<&[u8], Version> {
    let (input, minor) = be_u16(input)?;
    let (input, major) = be_u16(input)?;
    Ok((input, Version { minor, major }))
}

fn constant_pool(input: &[u8]) -> IResult<&[u8], Vec<ConstantPoolType>> {
    let (input, pool_count) = be_u16(input)?;
    let (input, constant_pool) = count(constant_type, pool_count as usize)(input)?;
    Ok((input, constant_pool))
}

fn constant_type(input: &[u8]) -> IResult<&[u8], ConstantPoolType> {
    let (input, tag) = constant_tag(input)?;
    Ok(match tag {
        ConstantPoolTag::Class => {
            let (input, name_index) = be_u16(input)?;
            (input, ConstantPoolType::Class { name_index })
        }
        ConstantPoolTag::Fieldref => {
            let (input, (class_index, name_and_type_index)) = ref_info(input)?;
            (
                input,
                ConstantPoolType::Fieldref {
                    class_index,
                    name_and_type_index,
                },
            )
        }
        ConstantPoolTag::Methodref => {
            let (input, (class_index, name_and_type_index)) = ref_info(input)?;
            (
                input,
                ConstantPoolType::Methodref {
                    class_index,
                    name_and_type_index,
                },
            )
        }
        ConstantPoolTag::InterfaceMethodref => {
            let (input, (class_index, name_and_type_index)) = ref_info(input)?;
            (
                input,
                ConstantPoolType::InterfaceMethodref {
                    class_index,
                    name_and_type_index,
                },
            )
        }
        ConstantPoolTag::String => {
            let (input, string_index) = be_u16(input)?;
            (input, ConstantPoolType::String { string_index })
        }
        ConstantPoolTag::Integer => {
            let (input, bytes) = take_n::<4>(input)?;
            (input, ConstantPoolType::Integer { bytes })
        }
        ConstantPoolTag::Float => {
            let (input, bytes) = take_n::<4>(input)?;
            (input, ConstantPoolType::Float { bytes })
        }
        ConstantPoolTag::Long => {
            let (input, val) = take_n::<8>(input)?;
            (input, ConstantPoolType::Long { val })
        }
        ConstantPoolTag::Double => {
            let (input, val) = take_n::<8>(input)?;
            (input, ConstantPoolType::Double { val })
        }
        ConstantPoolTag::NameAndType => {
            let (input, name_index) = be_u16(input)?;
            let (input, descriptor_index) = be_u16(input)?;
            (
                input,
                ConstantPoolType::NameAndType {
                    name_index,
                    descriptor_index,
                },
            )
        }
        ConstantPoolTag::Utf8 => {
            let (input, len) = be_u16(input)?;
            let (input, bytes) = take(len)(input)?;
            (
                input,
                ConstantPoolType::Utf8 {
                    bytes: bytes.to_vec(),
                },
            )
        }
        ConstantPoolTag::MethodHandle => {
            let (input, reference_kind) = be_u8(input)?;
            let (input, reference_index) = be_u16(input)?;
            (
                input,
                ConstantPoolType::MethodHandle {
                    reference_kind,
                    reference_index,
                },
            )
        }
        ConstantPoolTag::MethodType => {
            let (input, descriptor_index) = be_u16(input)?;
            (input, ConstantPoolType::MethodType { descriptor_index })
        }
        ConstantPoolTag::InvokeDynamic => {
            let (input, bootstrap_method_attr_index) = be_u16(input)?;
            let (input, name_and_type_index) = be_u16(input)?;
            (
                input,
                ConstantPoolType::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            )
        }
        _ => todo!(),
    })
}

fn constant_tag(input: &[u8]) -> IResult<&[u8], ConstantPoolTag> {
    let (input, byte) = be_u8(input)?;
    Ok((input, ConstantPoolTag::from(byte)))
}

fn ref_info(input: &[u8]) -> IResult<&[u8], (u16, u16)> {
    let (input, class_index) = be_u16(input)?;
    let (input, name_and_type_index) = be_u16(input)?;
    Ok((input, (class_index, name_and_type_index)))
}

fn take_n<const N: usize>(input: &[u8]) -> IResult<&[u8], [u8; N]> {
    let (input, bytes) = take(N)(input)?;
    Ok((input, bytes.try_into().expect("nom error")))
}

fn interfaces(input: &[u8]) -> IResult<&[u8], Vec<u16>> {
    let (input, interface_count) = be_u16(input)?;
    let (input, interfaces) = count(be_u16, interface_count as usize)(input)?;
    Ok((input, interfaces))
}

fn fields<'a>(
    input: &'a [u8],
    constant_pool: &[ConstantPoolType],
) -> IResult<&'a [u8], Vec<FieldInfo>> {
    let (input, fields_count) = be_u16(input)?;
    let (input, fields) = count(call(field_info, constant_pool), fields_count as usize)(input)?;
    Ok((input, fields))
}

fn field_info<'a>(
    input: &'a [u8],
    constant_pool: &[ConstantPoolType],
) -> IResult<&'a [u8], FieldInfo> {
    let (input, access_flags) = be_u16(input)?;
    let (input, name_index) = be_u16(input)?;
    let (input, descriptor_index) = be_u16(input)?;
    let (input, attributes) = attributes(input)?;
}

fn attributes(input: &[u8]) -> IResult<&[u8], Vec<Attribute>> {
    let (input, attributes_count) = be_u16(input)?;
    let (input, attributes) = count(call(), attributes_count as usize)(input)?;
    Ok((input, attributes))
}

fn attribute(input: &[u8]) -> IResult<&[u8], Attribute> {}
