// todo: split this up into modules

pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;

#[derive(Debug)]
pub struct ClassFile {
    pub magic: U4,
    pub version: Version,
    pub constant_pool: Vec<ConstantPoolType>,
    pub fields: Vec<FieldInfo>,
}

#[derive(Debug)]
pub struct Version {
    pub minor: U2,
    pub major: U2,
}

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
    Unknown,
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
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeType {
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    BootstrapMethods,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    ConstantValue {
        constant_value_index: U2,
    },
    Code {
        max_stack: U2,
        max_locals: U2,
        code: Vec<U1>,
        exception_table: Vec<ExceptionHandler>,
        attributes: Vec<AttributeType>,
    },
    StackMapTable {
        entries: Vec<StackMapFrame>,
    },
    Exceptions {
        number_of_exceptions: U2,
        exception_index_table: Vec<U2>,
    },
    InnerClasses {
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: U2,
        method_index: U2,
    },
    Synthetic,
    Signature {
        signature_index: U2,
    },
    SourceFile {
        source_file_index: U2,
    },
    SourceDebugExtension {
        debug_extension: Vec<U1>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table: Vec<LocalVariable>,
    },
    Deprecated,
    RuntimeVisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleParameterAnnotations {
        annotations: Vec<Annotation>,
    },
    AnnotationDefault {
        default_value: ElementValue,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethod>,
    },
}

#[derive(Debug, Clone)]
pub struct ExceptionHandler {
    pub start_pc: U2,
    pub end_pc: U2,
    pub handler_pc: U2,
    pub catch_type: U2,
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    Same {
        tag: U1,
        offset_delta: U2,
    },
    SameLocals1StackItem {
        tag: U1,
        offset_delta: U2,
        stack: [VerificationTypeInfo; 1],
    },
    SameLocalsStackItemExtended {
        tag: U1,
        offset_delta: U2,
        stack: [VerificationTypeInfo; 1],
    },
    Chop {
        tag: U1,
        offset_delta: U2,
    },
    SameExtended {
        tag: U1,
        offset_delta: U2,
    },
    Append {
        tag: U1,
        offset_delta: U2,
        locals: Vec<VerificationTypeInfo>,
    },
    Full {
        tag: U1,
        offset_delta: U2,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    TopVariable,
    IntegerVariable,
    FloatVariable,
    LongVariable,
    DoubleVariable,
    NullVariable,
    UninitializedThisVariable,
    ObjectVariable { cpool_index: U2 },
    UninitializedVariable { offset: U2 },
}

#[derive(Debug, Clone, Copy)]
pub struct InnerClass {
    pub inner_class_info: U2,
    pub outer_class_info: U2,
    pub inner_name_index: U2,
    pub inner_class_access_flags: U2,
}

#[derive(Debug, Clone, Copy)]
pub struct LineNumber {
    pub start_pc: U2,
    pub line_number: U2,
}

#[derive(Debug, Clone, Copy)]
pub struct LocalVariable {
    pub start_pc: U2,
    pub length: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub index: U2,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: U2,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone)]
pub struct ElementValuePair {
    pub element_name_index: U2,
    pub value: ElementValue,
}

#[derive(Debug, Clone, Copy)]
pub enum ElementValueTag {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    String,
    EnumConstant,
    Class,
    Annotation,
    Array,
    Unknown,
}

impl From<u8> for ElementValueTag {
    fn from(b: u8) -> Self {
        match b {
            b'B' => Self::Byte,
            b'C' => Self::Char,
            b'D' => Self::Double,
            b'F' => Self::Float,
            b'I' => Self::Int,
            b'J' => Self::Long,
            b'S' => Self::Short,
            b'Z' => Self::Boolean,
            b's' => Self::String,
            b'e' => Self::EnumConstant,
            b'c' => Self::Class,
            b'@' => Self::Annotation,
            b'[' => Self::Array,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementValue {
    ConstValueIndex {
        index: U2,
    },
    EnumConstValue {
        type_name_index: U2,
        const_name_index: U2,
    },
    ClassInfoIndex {
        index: U2,
    },
    AnnotationValue {
        annotation_value: Annotation,
    },
    ArrayValue {
        values: Vec<ElementValue>,
    },
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: U2,
    pub bootstrap_arguments: Vec<U2>,
}
