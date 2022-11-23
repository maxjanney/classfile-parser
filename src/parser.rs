use nom::{
    bytes::streaming::take,
    multi::count,
    number::streaming::{be_u16, be_u32, be_u8},
    IResult,
};

use crate::{
    constant_pool::{get_utf8, ConstantPoolTag},
    Annotation, AttributeTag, BootstrapMethod, ElementValue, ElementValuePair, ElementValueTag,
    ExceptionHandler, InnerClass, LineNumber, LocalVariable, StackMapFrame, VerificationTypeInfo,
    {Attribute, ClassFile, ConstantPoolType, FieldInfo, Version},
};

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
    let (input, fields) = count(|i| field_info(i, constant_pool), fields_count as usize)(input)?;
    Ok((input, fields))
}

fn field_info<'a>(
    input: &'a [u8],
    constant_pool: &[ConstantPoolType],
) -> IResult<&'a [u8], FieldInfo> {
    let (input, access_flags) = be_u16(input)?;
    let (input, name_index) = be_u16(input)?;
    let (input, descriptor_index) = be_u16(input)?;
    let (input, attributes) = attributes(input, constant_pool)?;
    Ok((
        input,
        FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        },
    ))
}

fn attributes<'a>(
    input: &'a [u8],
    constant_pool: &[ConstantPoolType],
) -> IResult<&'a [u8], Vec<Attribute>> {
    let (input, attributes_count) = be_u16(input)?;
    let (input, attributes) =
        count(|i| attribute(i, constant_pool), attributes_count as usize)(input)?;
    Ok((input, attributes))
}

fn attribute<'a>(
    input: &'a [u8],
    constant_pool: &[ConstantPoolType],
) -> IResult<&'a [u8], Attribute> {
    let (input, attr_name_index) = be_u16(input)?;
    let (input, attribute_length) = be_u32(input)?;
    let name_bytes = get_utf8(constant_pool, attr_name_index as usize);
    Ok(match AttributeTag::from(name_bytes) {
        AttributeTag::ConstantValue => {
            let (input, constant_value_index) = be_u16(input)?;
            (
                input,
                Attribute::ConstantValue {
                    constant_value_index,
                },
            )
        }
        AttributeTag::Code => {
            let (input, max_stack) = be_u16(input)?;
            let (input, max_locals) = be_u16(input)?;
            let (input, code_len) = be_u32(input)?;
            let (input, code) = take(code_len as usize)(input)?;
            let (input, exception_table) = exception_table(input)?;
            let (input, attributes) = attributes(input, constant_pool)?;
            (
                input,
                Attribute::Code {
                    max_stack,
                    max_locals,
                    code: code.to_vec(),
                    exception_table,
                    attributes,
                },
            )
        }
        AttributeTag::StackMapTable => {
            let (input, entries) = stack_map_table(input)?;
            (input, Attribute::StackMapTable { entries })
        }
        AttributeTag::Exceptions => {
            let (input, num_exceptions) = be_u16(input)?;
            let (input, exception_index_table) = count(be_u16, num_exceptions as usize)(input)?;
            (
                input,
                Attribute::Exceptions {
                    number_of_exceptions: num_exceptions,
                    exception_index_table,
                },
            )
        }
        AttributeTag::InnerClasses => {
            let (input, num_classes) = be_u16(input)?;
            let (input, classes) = count(inner_class, num_classes as usize)(input)?;
            (input, Attribute::InnerClasses { classes })
        }
        AttributeTag::EnclosingMethod => {
            let (input, class_index) = be_u16(input)?;
            let (input, method_index) = be_u16(input)?;
            (
                input,
                Attribute::EnclosingMethod {
                    class_index,
                    method_index,
                },
            )
        }
        AttributeTag::Synthetic => (input, Attribute::Synthetic),
        AttributeTag::Signature => {
            let (input, signature_index) = be_u16(input)?;
            (input, Attribute::Signature { signature_index })
        }
        AttributeTag::SourceFile => {
            let (input, source_file_index) = be_u16(input)?;
            (input, Attribute::SourceFile { source_file_index })
        }
        AttributeTag::SourceDebugExtension => {
            let (input, debug_extension) = take(attribute_length)(input)?;
            (
                input,
                Attribute::SourceDebugExtension {
                    debug_extension: debug_extension.try_into().expect("nom error"),
                },
            )
        }
        AttributeTag::LineNumberTable => {
            let (input, line_number_table_length) = be_u16(input)?;
            let (input, line_number_table) =
                count(line_number, line_number_table_length as usize)(input)?;
            (input, Attribute::LineNumberTable { line_number_table })
        }
        AttributeTag::LocalVariableTable => {
            let (input, local_variable_table_length) = be_u16(input)?;
            let (input, local_variable_table) =
                count(local_variable, local_variable_table_length as usize)(input)?;
            (
                input,
                Attribute::LocalVariableTable {
                    local_variable_table,
                },
            )
        }
        AttributeTag::LocalVariableTypeTable => {
            let (input, local_variable_type_table_length) = be_u16(input)?;
            let (input, local_variable_type_table) =
                count(local_variable, local_variable_type_table_length as usize)(input)?;
            (
                input,
                Attribute::LocalVariableTypeTable {
                    local_variable_type_table,
                },
            )
        }
        AttributeTag::Deprecated => (input, Attribute::Deprecated),
        AttributeTag::RuntimeVisibleAnnotations => {
            let (input, num_annotations) = be_u16(input)?;
            let (input, annotations) =
                count(|i| annotation(i, constant_pool), num_annotations as usize)(input)?;
            (input, Attribute::RuntimeVisibleAnnotations { annotations })
        }
        AttributeTag::RuntimeInvisibleAnnotations => {
            let (input, num_annotations) = be_u16(input)?;
            let (input, annotations) =
                count(|i| annotation(i, constant_pool), num_annotations as usize)(input)?;
            (
                input,
                Attribute::RuntimeInvisibleAnnotations { annotations },
            )
        }
        AttributeTag::RuntimeVisibleParameterAnnotations => {
            todo!()
        }
        AttributeTag::RuntimeInvisibleParameterAnnotations => {
            todo!()
        }
        AttributeTag::AnnotationDefault => {
            let (input, default_value) = element_value(input, constant_pool)?;
            (input, Attribute::AnnotationDefault { default_value })
        }
        AttributeTag::BootstrapMethods => {
            let (input, num_bootstrap_methods) = be_u16(input)?;
            let (input, bootstrap_methods) =
                count(bootstrap_method, num_bootstrap_methods as usize)(input)?;
            (input, Attribute::BootstrapMethods { bootstrap_methods })
        }
        _ => unreachable!(),
    })
}

fn exception_table(input: &[u8]) -> IResult<&[u8], Vec<ExceptionHandler>> {
    let (input, table_len) = be_u16(input)?;
    let (input, table) = count(exception_handler, table_len as usize)(input)?;
    Ok((input, table))
}

fn exception_handler(input: &[u8]) -> IResult<&[u8], ExceptionHandler> {
    let (input, start_pc) = be_u16(input)?;
    let (input, end_pc) = be_u16(input)?;
    let (input, handler_pc) = be_u16(input)?;
    let (input, catch_type) = be_u16(input)?;
    Ok((
        input,
        ExceptionHandler {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        },
    ))
}

fn stack_map_table(input: &[u8]) -> IResult<&[u8], Vec<StackMapFrame>> {
    let (input, entries_count) = be_u16(input)?;
    let (input, entries) = count(stack_map_frame, entries_count as usize)(input)?;
    Ok((input, entries))
}

fn stack_map_frame(input: &[u8]) -> IResult<&[u8], StackMapFrame> {
    let (input, frame_type) = be_u8(input)?;
    Ok(match frame_type {
        // same
        0..=63 => (
            input,
            StackMapFrame::Same {
                tag: frame_type,
                offset_delta: frame_type as u16,
            },
        ),
        // same locals 1 stack item
        64..=127 => {
            let offset_delta = frame_type as u16 - 64;
            let (input, type_info) = verification_type_info(input)?;
            (
                input,
                StackMapFrame::SameLocals1StackItem {
                    tag: frame_type,
                    offset_delta,
                    stack: [type_info],
                },
            )
        }
        // reserved
        128..=246 => (input, StackMapFrame::Reserved(frame_type)),
        // same locals stack item extended
        247 => {
            let (input, offset_delta) = be_u16(input)?;
            let (input, type_info) = verification_type_info(input)?;
            (
                input,
                StackMapFrame::SameLocalsStackItemExtended {
                    tag: frame_type,
                    offset_delta,
                    stack: [type_info],
                },
            )
        }
        // chop
        248..=250 => {
            let (input, offset_delta) = be_u16(input)?;
            (
                input,
                StackMapFrame::Chop {
                    tag: frame_type,
                    offset_delta,
                },
            )
        }
        // same extended
        251 => {
            let (input, offset_delta) = be_u16(input)?;
            (
                input,
                StackMapFrame::SameExtended {
                    tag: frame_type,
                    offset_delta,
                },
            )
        }
        // append
        252..=254 => {
            let (input, offset_delta) = be_u16(input)?;
            let k = frame_type as usize - 251;
            let (input, locals) = count(verification_type_info, k)(input)?;
            (
                input,
                StackMapFrame::Append {
                    tag: frame_type,
                    offset_delta,
                    locals,
                },
            )
        }
        // full
        255 => {
            let (input, offset_delta) = be_u16(input)?;
            let (input, num_locals) = be_u16(input)?;
            let (input, locals) = count(verification_type_info, num_locals as usize)(input)?;
            let (input, num_stack_items) = be_u16(input)?;
            let (input, stack) = count(verification_type_info, num_stack_items as usize)(input)?;
            (
                input,
                StackMapFrame::Full {
                    tag: frame_type,
                    offset_delta,
                    locals,
                    stack,
                },
            )
        }
    })
}

fn verification_type_info(input: &[u8]) -> IResult<&[u8], VerificationTypeInfo> {
    use VerificationTypeInfo::*;

    let (input, tag) = be_u8(input)?;
    Ok(match tag {
        0 => (input, TopVariable),
        1 => (input, IntegerVariable),
        2 => (input, FloatVariable),
        4 => (input, LongVariable),
        3 => (input, DoubleVariable),
        5 => (input, NullVariable),
        6 => (input, UninitializedThisVariable),
        7 => {
            let (input, cpool_index) = be_u16(input)?;
            (input, ObjectVariable { cpool_index })
        }
        8 => {
            let (input, offset) = be_u16(input)?;
            (input, UninitializedVariable { offset })
        }
        _ => unreachable!(),
    })
}

fn inner_class(input: &[u8]) -> IResult<&[u8], InnerClass> {
    let (input, inner_class_info) = be_u16(input)?;
    let (input, outer_class_info) = be_u16(input)?;
    let (input, inner_name_index) = be_u16(input)?;
    let (input, inner_class_access_flags) = be_u16(input)?;
    Ok((
        input,
        InnerClass {
            inner_class_info,
            outer_class_info,
            inner_name_index,
            inner_class_access_flags,
        },
    ))
}

fn line_number(input: &[u8]) -> IResult<&[u8], LineNumber> {
    let (input, start_pc) = be_u16(input)?;
    let (input, line_number) = be_u16(input)?;
    Ok((
        input,
        LineNumber {
            start_pc,
            line_number,
        },
    ))
}

fn local_variable(input: &[u8]) -> IResult<&[u8], LocalVariable> {
    let (input, start_pc) = be_u16(input)?;
    let (input, length) = be_u16(input)?;
    let (input, name_index) = be_u16(input)?;
    let (input, descriptor_index) = be_u16(input)?;
    let (input, index) = be_u16(input)?;
    Ok((
        input,
        LocalVariable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        },
    ))
}

fn annotation<'a>(
    input: &'a [u8],
    constant_pool: &[ConstantPoolType],
) -> IResult<&'a [u8], Annotation> {
    let (input, type_index) = be_u16(input)?;
    let (input, num_element_value_pairs) = be_u16(input)?;
    let (input, element_value_pairs) =
        count(element_value_pair, num_element_value_pairs as usize)(input)?;
    let type_name = get_utf8(constant_pool, type_index as usize);
    Ok((
        input,
        Annotation {
            type_name: type_name.to_vec(),
            element_value_pairs,
        },
    ))
}

fn element_value_pair(input: &[u8]) -> IResult<&[u8], ElementValuePair> {
    todo!()
}

fn element_value<'a>(
    input: &'a [u8],
    constant_pool: &[ConstantPoolType],
) -> IResult<&'a [u8], ElementValue> {
    let (input, tag) = element_value_tag(input)?;
    Ok(match tag {
        ElementValueTag::Byte => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Byte { index })
        }
        ElementValueTag::Char => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Char { index })
        }
        ElementValueTag::Double => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Double { index })
        }
        ElementValueTag::Float => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Float { index })
        }
        ElementValueTag::Int => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Int { index })
        }
        ElementValueTag::Long => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Long { index })
        }
        ElementValueTag::Short => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Short { index })
        }
        ElementValueTag::Boolean => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Boolean { index })
        }
        ElementValueTag::String => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::String { index })
        }
        ElementValueTag::EnumConstant => {
            let (input, type_name_index) = be_u16(input)?;
            let (input, const_name_index) = be_u16(input)?;
            (
                input,
                ElementValue::EnumConstValue {
                    type_name_index,
                    const_name_index,
                },
            )
        }
        ElementValueTag::Class => {
            let (input, index) = be_u16(input)?;
            (input, ElementValue::Class { index })
        }
        ElementValueTag::Annotation => {
            let (input, annotation) = annotation(input, constant_pool)?;
            (input, ElementValue::Annotation { annotation })
        }
        ElementValueTag::Array => {
            let (input, num_values) = be_u16(input)?;
            let (input, values) =
                count(|i| element_value(i, constant_pool), num_values as usize)(input)?;
            (input, ElementValue::Array { values })
        }
        _ => unreachable!(),
    })
}

fn element_value_tag(input: &[u8]) -> IResult<&[u8], ElementValueTag> {
    let (input, tag) = be_u8(input)?;
    Ok((input, ElementValueTag::from(tag)))
}

fn bootstrap_method(input: &[u8]) -> IResult<&[u8], BootstrapMethod> {
    let (input, method_ref) = be_u16(input)?;
    let (input, num_args) = be_u16(input)?;
    let (input, args) = count(be_u16, num_args as usize)(input)?;
    Ok((input, BootstrapMethod { method_ref, args }))
}
