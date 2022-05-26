use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    mem, ptr,
};

use object::Endianness;

use crate::{
    generated::{
        btf_array, btf_decl_tag, btf_enum, btf_func_linkage, btf_member, btf_param, btf_type,
        btf_type__bindgen_ty_1, btf_var, btf_var_secinfo, BTF_KIND_ARRAY, BTF_KIND_CONST,
        BTF_KIND_DATASEC, BTF_KIND_DECL_TAG, BTF_KIND_ENUM, BTF_KIND_FLOAT, BTF_KIND_FUNC,
        BTF_KIND_FUNC_PROTO, BTF_KIND_FWD, BTF_KIND_INT, BTF_KIND_PTR, BTF_KIND_RESTRICT,
        BTF_KIND_STRUCT, BTF_KIND_TYPEDEF, BTF_KIND_TYPE_TAG, BTF_KIND_UNION, BTF_KIND_UNKN,
        BTF_KIND_VAR, BTF_KIND_VOLATILE,
    },
    obj::btf::{Btf, BtfError, MAX_RESOLVE_DEPTH},
};

use super::btf;

#[derive(Clone, Debug)]
pub(crate) enum BtfType {
    Unknown,
    Fwd(Fwd),
    Const(Const),
    Volatile(Volatile),
    Restrict(Restrict),
    Ptr(Ptr),
    Typedef(Typedef),
    Func(Func),
    Int(Int),
    Float(Float),
    Enum(Enum),
    Array(Array),
    Struct(Struct),
    Union(Union),
    FuncProto(FuncProto),
    Var(Var),
    DataSec(DataSec),
    DeclTag(DeclTag),
    TypeTag(TypeTag),
}

#[derive(Clone, Debug)]
pub(crate) struct Fwd {
    btf_type: btf_type,
}

impl Fwd {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }

    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }

    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Fwd }
}

#[derive(Clone, Debug)]
pub(crate) struct Const {
    btf_type: btf_type,
}

impl Const {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
        fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Const }

    pub(crate) fn new(type_: u32) -> Self {
        let info = (BTF_KIND_CONST) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = 0;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = type_;
        Const { btf_type }
    }
    
}

#[derive(Clone, Debug)]
pub(crate) struct Volatile {
    btf_type: btf_type,
}

impl Volatile {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Volatile }
}

#[derive(Clone, Debug)]
pub(crate) struct Restrict {
    btf_type: btf_type,
}

impl Restrict {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Restrict }
}

#[derive(Clone, Debug)]
pub(crate) struct Ptr {
    btf_type: btf_type,
}

impl Ptr {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Ptr }

    pub(crate) fn new(name_off: u32, type_: u32) -> Self {
        let info = (BTF_KIND_PTR) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = type_;
        Ptr { btf_type }
    }
    
}

#[derive(Clone, Debug)]
pub(crate) struct Typedef {
    btf_type: btf_type,
}

impl Typedef {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Typedef }

    pub(crate) fn new(name_off: u32, type_: u32) -> Self {
        let info = (BTF_KIND_TYPEDEF) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = type_;
        Typedef { btf_type }
    }
    
}

#[derive(Clone, Debug)]
pub(crate) struct Float {
    btf_type: btf_type,
}

impl Float {
    pub(crate) fn size(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.size })
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Float }

    pub(crate) fn new(name_off: u32, size: u32) -> Self {
        let info = (BTF_KIND_FLOAT) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.size = size;
        Float { btf_type }
    }
    
}

#[derive(Clone, Debug)]
pub(crate) struct Func {
    btf_type: btf_type,
}

impl Func {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Func }

    pub(crate) fn new(name_off: u32, proto: u32, linkage: btf_func_linkage) -> Self {
        let mut info = (BTF_KIND_FUNC) << 24;
        info |= (linkage as u32) & 0xFFFF;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = proto;
        Func{btf_type}
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TypeTag {
    btf_type: btf_type,
}

impl TypeTag {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        bytes_of::<btf_type>(&self.btf_type)
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::TypeTag }

    pub(crate) fn new(name_off: u32, type_: u32) -> Self {
        let info = (BTF_KIND_TYPE_TAG) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = type_;
        TypeTag { btf_type }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Int {
    btf_type: btf_type,
    data: u32,
}

impl Int {
    pub(crate) fn size(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.size })
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        // FIXME: ne_bytes???
        buf.append(&mut self.data.to_ne_bytes().to_vec());
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Int }

    pub(crate) fn new(name_off: u32, size: u32, encoding: u32, offset: u32) -> Self {
        let info = (BTF_KIND_INT) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.size = size;

        let mut data = 0u32;
        data |= (encoding & 0x0f) << 24;
        data |= (offset & 0xff) << 16;
        data |= (size * 8) & 0xff;
        Int{ btf_type, data}
    }

    pub(crate) fn encoding(&self) -> u32 {
        (self.data & 0x0f000000) >> 24
    }

    pub(crate) fn offset(&self) -> u32 {
        (self.data & 0x00ff0000) >> 16
    }

    pub(crate) fn bits(&self) -> u32 {
        self.data & 0x000000ff
    }

}


#[derive(Clone, Debug)]
pub(crate) struct Enum {
    btf_type: btf_type,
    values: Vec<btf_enum>,
}

impl Enum {
    pub(crate) fn size(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.size })
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        for en in self.values {
            buf.append(&mut bytes_of::<btf_enum>(&en).to_vec());
        }
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Enum }
    pub(crate) fn new(name_off: u32, values: Vec<btf_enum>) -> Self {
        let mut info = (BTF_KIND_ENUM) << 24;
        info |= (values.len() as u32) & 0xFFFF;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.size = 4;
        Enum{btf_type, values}
    }
    
}

#[derive(Clone, Debug)]
pub(crate) struct Struct {
    btf_type: btf_type,
    members: Vec<btf_member>,
}

impl Struct {
    pub(crate) fn size(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.size })
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        for member in self.members {
            buf.append(&mut bytes_of::<btf_member>(&member).to_vec());
        }
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Struct }

    pub(crate) fn new(name_off: u32, members: Vec<btf_member>, size: u32) -> Self {
        let mut info = (BTF_KIND_STRUCT) << 24;
        info |= (members.len() as u32) & 0xFFFF;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.size = size;
       Struct { btf_type, members }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Union {
    btf_type: btf_type,
    members: Vec<btf_member>,
}

impl Union {
    pub(crate) fn size(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.size })
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        for member in self.members {
            buf.append(&mut bytes_of::<btf_member>(&member).to_vec());
        }
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Union }
}

#[derive(Clone, Debug)]
pub(crate) struct Array {
    btf_type: btf_type,
    array: btf_array,
}

impl Array {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        buf.append(&mut bytes_of::<btf_array>(&self.array).to_vec());
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn len(&self) -> u32 {
        self.array.nelems
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Array }

    pub(crate) fn new(name_off: u32, type_: u32, index_type: u32, nelems: u32) -> Self {
        let info = (BTF_KIND_ARRAY) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        let btf_array = btf_array {
            type_,
            index_type,
            nelems,
        };
        Array { btf_type, array: btf_array }
    }

}

#[derive(Clone, Debug)]
pub(crate) struct FuncProto {
    btf_type: btf_type,
    params: Vec<btf_param>,
}

impl FuncProto {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        for param in self.params {
            buf.append(&mut bytes_of::<btf_param>(&param).to_vec());
        }
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::FuncProto }

    pub(crate) fn new(params: Vec<btf_param>, return_type: u32) -> Self {
        let mut info = (BTF_KIND_FUNC_PROTO) << 24;
        info |= (params.len() as u32) & 0xFFFF;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = 0;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = return_type;
        FuncProto{btf_type, params}
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Var {
    btf_type: btf_type,
    var: btf_var,
}

impl Var {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        buf.append(&mut bytes_of::<btf_var>(&self.var).to_vec());
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::Var }

    pub(crate) fn new(name_off: u32, type_: u32, linkage: u32) -> Self {
        let info = (BTF_KIND_VAR) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = type_;
        let var = btf_var { linkage };
        Var{btf_type, var}
    }

}

#[derive(Clone, Debug)]
pub(crate) struct DataSec {
    btf_type: btf_type,
    section_info: Vec<btf_var_secinfo>,
}

impl DataSec {
    pub(crate) fn size(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.size })
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        None
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        for secinfo in self.section_info {
            buf.append(&mut bytes_of::<btf_var_secinfo>(&secinfo).to_vec());
        }
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::DataSec }

    pub(crate) fn new(
        name_off: u32,
        variables: Vec<btf_var_secinfo>,
        size: u32,
    ) -> Self {
        let mut info = (BTF_KIND_DATASEC) << 24;
        info |= (variables.len() as u32) & 0xFFFF;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.size = size;
        DataSec { btf_type, section_info: variables }
    }
}


#[derive(Clone, Debug)]
pub(crate) struct DeclTag {
    btf_type: btf_type,
    decl_tag: btf_decl_tag,
}

impl DeclTag {
    pub(crate) fn size(&self) -> Option<u32> {
        None
    }
    pub(crate) fn next_type(&self) -> Option<u32> {
        Some(unsafe { self.btf_type.__bindgen_anon_1.type_ })
    }
    pub(crate) fn to_bytes(&self) -> &[u8] {
        let mut buf = bytes_of::<btf_type>(&self.btf_type).to_vec();
        buf.append(&mut bytes_of::<btf_decl_tag>(&self.decl_tag).to_vec());
        &buf
    }
    fn name_offset(&self) -> u32 {
        self.btf_type.name_off
    }
    pub(crate) fn kind(&self) -> BtfKind { BtfKind::DeclTag }

    pub(crate) fn new(name_off: u32, type_: u32, component_idx: i32) -> Self {
        let info = (BTF_KIND_DECL_TAG) << 24;
        let mut btf_type = unsafe { std::mem::zeroed::<btf_type>() };
        btf_type.name_off = name_off;
        btf_type.info = info;
        btf_type.__bindgen_anon_1.type_ = type_;
        let decl_tag = btf_decl_tag { component_idx };
        DeclTag { btf_type, decl_tag }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub(crate) enum BtfKind {
    Unknown = BTF_KIND_UNKN,
    Int = BTF_KIND_INT,
    Float = BTF_KIND_FLOAT,
    Ptr = BTF_KIND_PTR,
    Array = BTF_KIND_ARRAY,
    Struct = BTF_KIND_STRUCT,
    Union = BTF_KIND_UNION,
    Enum = BTF_KIND_ENUM,
    Fwd = BTF_KIND_FWD,
    Typedef = BTF_KIND_TYPEDEF,
    Volatile = BTF_KIND_VOLATILE,
    Const = BTF_KIND_CONST,
    Restrict = BTF_KIND_RESTRICT,
    Func = BTF_KIND_FUNC,
    FuncProto = BTF_KIND_FUNC_PROTO,
    Var = BTF_KIND_VAR,
    DataSec = BTF_KIND_DATASEC,
    DeclTag = BTF_KIND_DECL_TAG,
    TypeTag = BTF_KIND_TYPE_TAG,
}

impl TryFrom<u32> for BtfKind {
    type Error = BtfError;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        use BtfKind::*;
        Ok(match v {
            BTF_KIND_UNKN => Unknown,
            BTF_KIND_INT => Int,
            BTF_KIND_FLOAT => Float,
            BTF_KIND_PTR => Ptr,
            BTF_KIND_ARRAY => Array,
            BTF_KIND_STRUCT => Struct,
            BTF_KIND_UNION => Union,
            BTF_KIND_ENUM => Enum,
            BTF_KIND_FWD => Fwd,
            BTF_KIND_TYPEDEF => Typedef,
            BTF_KIND_VOLATILE => Volatile,
            BTF_KIND_CONST => Const,
            BTF_KIND_RESTRICT => Restrict,
            BTF_KIND_FUNC => Func,
            BTF_KIND_FUNC_PROTO => FuncProto,
            BTF_KIND_VAR => Var,
            BTF_KIND_DATASEC => DataSec,
            BTF_KIND_DECL_TAG => DeclTag,
            BTF_KIND_TYPE_TAG => TypeTag,
            kind => return Err(BtfError::InvalidTypeKind { kind }),
        })
    }
}

impl Display for BtfKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BtfKind::Unknown => write!(f, "[UNKNOWN]"),
            BtfKind::Int => write!(f, "[INT]"),
            BtfKind::Float => write!(f, "[FLOAT]"),
            BtfKind::Ptr => write!(f, "[PTR]"),
            BtfKind::Array => write!(f, "[ARRAY]"),
            BtfKind::Struct => write!(f, "[STRUCT]"),
            BtfKind::Union => write!(f, "[UNION]"),
            BtfKind::Enum => write!(f, "[ENUM]"),
            BtfKind::Fwd => write!(f, "[FWD]"),
            BtfKind::Typedef => write!(f, "[TYPEDEF]"),
            BtfKind::Volatile => write!(f, "[VOLATILE]"),
            BtfKind::Const => write!(f, "[CONST]"),
            BtfKind::Restrict => write!(f, "[RESTRICT]"),
            BtfKind::Func => write!(f, "[FUNC]"),
            BtfKind::FuncProto => write!(f, "[FUNC_PROTO]"),
            BtfKind::Var => write!(f, "[VAR]"),
            BtfKind::DataSec => write!(f, "[DATASEC]"),
            BtfKind::DeclTag => write!(f, "[DECL_TAG]"),
            BtfKind::TypeTag => write!(f, "[TYPE_TAG]"),
        }
    }
}

impl Default for BtfKind {
    fn default() -> Self {
        BtfKind::Unknown
    }
}

unsafe fn read<T>(data: &[u8]) -> Result<T, BtfError> {
    if mem::size_of::<T>() > data.len() {
        return Err(BtfError::InvalidTypeInfo);
    }

    Ok(ptr::read_unaligned::<T>(data.as_ptr() as *const T))
}

unsafe fn read_array<T>(data: &[u8], len: usize) -> Result<Vec<T>, BtfError> {
    if mem::size_of::<T>() * len > data.len() {
        return Err(BtfError::InvalidTypeInfo);
    }

    Ok((0..len)
        .map(|i| {
            ptr::read_unaligned::<T>((data.as_ptr() as usize + i * mem::size_of::<T>()) as *const T)
        })
        .collect::<Vec<T>>())
}

impl BtfType {
    #[allow(unused_unsafe)]
    pub(crate) unsafe fn read(data: &[u8], endianness: Endianness) -> Result<BtfType, BtfError> {
        let btf_type = unsafe { read::<btf_type>(data)? };
        let data = &data[mem::size_of::<btf_type>()..];
        let vlen = type_vlen(&btf_type) as usize;
        Ok(match type_kind(&btf_type)? {
            BtfKind::Unknown => BtfType::Unknown,
            BtfKind::Fwd => BtfType::Fwd(Fwd { btf_type }),
            BtfKind::Const => BtfType::Const(Const { btf_type }),
            BtfKind::Volatile => BtfType::Volatile(Volatile { btf_type }),
            BtfKind::Restrict => BtfType::Restrict(Restrict { btf_type }),
            BtfKind::Ptr => BtfType::Ptr(Ptr { btf_type }),
            BtfKind::Typedef => BtfType::Typedef(Typedef { btf_type }),
            BtfKind::Func => BtfType::Func(Func { btf_type }),
            BtfKind::Int => {
                if mem::size_of::<u32>() > data.len() {
                    return Err(BtfError::InvalidTypeInfo);
                }
                let read_u32 = if endianness == Endianness::Little {
                    u32::from_le_bytes
                } else {
                    u32::from_be_bytes
                };
                BtfType::Int(Int {
                    btf_type,
                    data: read_u32(data[..mem::size_of::<u32>()].try_into().unwrap()),
                })
            }
            BtfKind::Float => BtfType::Float(Float { btf_type }),
            BtfKind::Enum => BtfType::Enum(Enum {
                btf_type,
                values: unsafe { read_array(data, vlen)? },
            }),
            BtfKind::Array => BtfType::Array(Array {
                btf_type,
                array: unsafe { read(data)? },
            }),
            BtfKind::Struct => BtfType::Struct(Struct {
                btf_type,
                members: unsafe { read_array(data, vlen)? },
            }),
            BtfKind::Union => BtfType::Union(Union {
                btf_type,
                members: unsafe { read_array(data, vlen)? },
            }),
            BtfKind::FuncProto => BtfType::FuncProto(FuncProto {
                btf_type,
                params: unsafe { read_array(data, vlen)? },
            }),
            BtfKind::Var => BtfType::Var(Var {
                btf_type,
                var: unsafe { read(data)? },
            }),
            BtfKind::DataSec => BtfType::DataSec(DataSec {
                btf_type,
                section_info: unsafe { read_array(data, vlen)? },
            }),
            BtfKind::DeclTag => BtfType::DeclTag(DeclTag {
                btf_type,
                decl_tag: unsafe { read(data)? },
            }),
            BtfKind::TypeTag => BtfType::TypeTag(TypeTag { btf_type }),
        })
    }

    pub(crate) fn to_bytes(&self) -> &[u8] {
        match self {
            BtfType::Unknown => &[],
            BtfType::Fwd(t) => t.to_bytes(),
            BtfType::Const(t) => t.to_bytes(),
            BtfType::Volatile(t) => t.to_bytes(),
            BtfType::Restrict(t) => t.to_bytes(),
            BtfType::Ptr(t) => t.to_bytes(),
            BtfType::Typedef(t) => t.to_bytes(),
            BtfType::Func(t) => t.to_bytes(),
            BtfType::Int(t) => t.to_bytes(),
            BtfType::Float(t) => t.to_bytes(),
            BtfType::Enum(t) => t.to_bytes(),
            BtfType::Array(t) => t.to_bytes(),
            BtfType::Struct(t) => t.to_bytes(),
            BtfType::Union(t) => t.to_bytes(),
            BtfType::FuncProto(t) => t.to_bytes(),
            BtfType::Var(t) => t.to_bytes(),
            BtfType::DataSec(t) => t.to_bytes(),
            BtfType::DeclTag(t) => t.to_bytes(),
            BtfType::TypeTag(t) => t.to_bytes(),
        }
    }

    fn size(&self) -> Option<u32> {
        match self {
            BtfType::Unknown => None,
            BtfType::Fwd(t) => t.size(),
            BtfType::Const(t) => t.size(),
            BtfType::Volatile(t) => t.size(),
            BtfType::Restrict(t) => t.size(),
            BtfType::Ptr(t) => t.size(),
            BtfType::Typedef(t) => t.size(),
            BtfType::Func(t) => t.size(),
            BtfType::Int(t) => t.size(),
            BtfType::Float(t) => t.size(),
            BtfType::Enum(t) => t.size(),
            BtfType::Array(t) => t.size(),
            BtfType::Struct(t) => t.size(),
            BtfType::Union(t) => t.size(),
            BtfType::FuncProto(t) => t.size(),
            BtfType::Var(t) => t.size(),
            BtfType::DataSec(t) => t.size(),
            BtfType::DeclTag(t) => t.size(),
            BtfType::TypeTag(t) => t.size(),
        }
    }

    fn next_type(&self) -> Option<u32> {
        match self {
            BtfType::Unknown => None,
            BtfType::Fwd(t) => t.next_type(),
            BtfType::Const(t) => t.next_type(),
            BtfType::Volatile(t) => t.next_type(),
            BtfType::Restrict(t) => t.next_type(),
            BtfType::Ptr(t) => t.next_type(),
            BtfType::Typedef(t) => t.next_type(),
            BtfType::Func(t) => t.next_type(),
            BtfType::Int(t) => t.next_type(),
            BtfType::Float(t) => t.next_type(),
            BtfType::Enum(t) => t.next_type(),
            BtfType::Array(t) => t.next_type(),
            BtfType::Struct(t) => t.next_type(),
            BtfType::Union(t) => t.next_type(),
            BtfType::FuncProto(t) => t.next_type(),
            BtfType::Var(t) => t.next_type(),
            BtfType::DataSec(t) => t.next_type(),
            BtfType::DeclTag(t) => t.next_type(),
            BtfType::TypeTag(t) => t.next_type(),
        }
    }

    pub(crate) fn type_info_size(&self) -> usize {
        match self {
            BtfType::Unknown => mem::size_of::<btf_type>(),
            BtfType::Fwd(t) => mem::size_of_val(t),
            BtfType::Const(t) => mem::size_of_val(t),
            BtfType::Volatile(t) => mem::size_of_val(t),
            BtfType::Restrict(t) => mem::size_of_val(t),
            BtfType::Ptr(t) => mem::size_of_val(t),
            BtfType::Typedef(t) => mem::size_of_val(t),
            BtfType::Func(t) => mem::size_of_val(t),
            BtfType::Int(t) => mem::size_of_val(t),
            BtfType::Float(t) => mem::size_of_val(t),
            BtfType::Enum(t) => mem::size_of_val(t),
            BtfType::Array(t) => mem::size_of_val(t),
            BtfType::Struct(t) => mem::size_of_val(t),
            BtfType::Union(t) => mem::size_of_val(t),
            BtfType::FuncProto(t) => mem::size_of_val(t),
            BtfType::Var(t) => mem::size_of_val(t),
            BtfType::DataSec(t) => mem::size_of_val(t),
            BtfType::DeclTag(t) => mem::size_of_val(t),
            BtfType::TypeTag(t) => mem::size_of_val(t),
        }
    }

    pub(crate) fn name_offset(&self) -> u32 {
        match self {
            BtfType::Unknown => 0,
            BtfType::Fwd(t) => t.name_offset(),
            BtfType::Const(t) => t.name_offset(),
            BtfType::Volatile(t) => t.name_offset(),
            BtfType::Restrict(t) => t.name_offset(),
            BtfType::Ptr(t) => t.name_offset(),
            BtfType::Typedef(t) => t.name_offset(),
            BtfType::Func(t) => t.name_offset(),
            BtfType::Int(t) => t.name_offset(),
            BtfType::Float(t) => t.name_offset(),
            BtfType::Enum(t) => t.name_offset(),
            BtfType::Array(t) => t.name_offset(),
            BtfType::Struct(t) => t.name_offset(),
            BtfType::Union(t) => t.name_offset(),
            BtfType::FuncProto(t) => t.name_offset(),
            BtfType::Var(t) => t.name_offset(),
            BtfType::DataSec(t) => t.name_offset(),
            BtfType::DeclTag(t) => t.name_offset(),
            BtfType::TypeTag(t) => t.name_offset(),
        }
    }

    pub(crate) fn kind(&self) -> BtfKind {
        match self {
            BtfType::Unknown => BtfKind::Unknown,
            BtfType::Fwd(t) => t.kind(),
            BtfType::Const(t) => t.kind(),
            BtfType::Volatile(t) => t.kind(),
            BtfType::Restrict(t) => t.kind(),
            BtfType::Ptr(t) => t.kind(),
            BtfType::Typedef(t) => t.kind(),
            BtfType::Func(t) => t.kind(),
            BtfType::Int(t) => t.kind(),
            BtfType::Float(t) => t.kind(),
            BtfType::Enum(t) => t.kind(),
            BtfType::Array(t) => t.kind(),
            BtfType::Struct(t) => t.kind(),
            BtfType::Union(t) => t.kind(),
            BtfType::FuncProto(t) => t.kind(),
            BtfType::Var(t) => t.kind(),
            BtfType::DataSec(t) => t.kind(),
            BtfType::DeclTag(t) => t.kind(),
            BtfType::TypeTag(t) => t.kind(),
        }
    }

    pub(crate) fn is_composite(&self) -> bool {
        matches!(self, BtfType::Struct(_) | BtfType::Union(_) )
    }
}

fn type_kind(ty: &btf_type) -> Result<BtfKind, BtfError> {
    ((ty.info >> 24) & 0x1F).try_into()
}

pub(crate) fn type_vlen(ty: &btf_type) -> usize {
    (ty.info & 0xFFFF) as usize
}

pub(crate) fn member_bit_offset(info: u32, member: &btf_member) -> usize {
    let k_flag = info >> 31 == 1;
    let bit_offset = if k_flag {
        member.offset & 0xFFFFFF
    } else {
        member.offset
    };

    bit_offset as usize
}

pub(crate) fn member_bit_field_size(ty: &btf_type, member: &btf_member) -> usize {
    let k_flag = (ty.info >> 31) == 1;
    let size = if k_flag { member.offset >> 24 } else { 0 };

    size as usize
}

pub(crate) fn types_are_compatible(
    local_btf: &Btf,
    root_local_id: u32,
    target_btf: &Btf,
    root_target_id: u32,
) -> Result<bool, BtfError> {
    let mut local_id = root_local_id;
    let mut target_id = root_target_id;
    let local_ty = local_btf.type_by_id(local_id)?;
    let target_ty = target_btf.type_by_id(target_id)?;

    if local_ty.kind() != target_ty.kind() {
        return Ok(false);
    }

    for _ in 0..MAX_RESOLVE_DEPTH {
        local_id = local_btf.resolve_type(local_id)?;
        target_id = target_btf.resolve_type(target_id)?;
        let local_ty = local_btf.type_by_id(local_id)?;
        let target_ty = target_btf.type_by_id(target_id)?;

        if local_ty.kind() != target_ty.kind() {
            return Ok(false);
        }

        use BtfType::*;
        match local_ty {
            Unknown | Struct(_) | Union(_) | Enum(_) | Fwd(_) | Float(_) => {
                return Ok(true)
            }
            Int(local) => {
                if let Int(target) = target_ty {
                    return Ok(local.offset() == 0 && target.offset() == 0);
                }
            }
            Ptr(local) => {
                if let Ptr(target) = target_ty {
                    local_id = local.next_type().unwrap();
                    target_id = target.next_type().unwrap();
                    continue;
                }
            }
            Array(local) => {
                if let Array(target) = target_ty {
                    local_id = local.next_type().unwrap();
                    target_id = target.next_type().unwrap();
                    continue;
                }
            }
            FuncProto(local) => {
                if let FuncProto(target) = target_ty {
                    if local.params.len() != target.params.len() {
                        return Ok(false);
                    }

                    for (l_param, t_param) in local.params.iter().zip(target.params.iter()) {
                        let local_id = local_btf.resolve_type(l_param.type_)?;
                        let target_id = target_btf.resolve_type(t_param.type_)?;
                        if !types_are_compatible(local_btf, local_id, target_btf, target_id)? {
                            return Ok(false);
                        }
                    }

                    local_id = local.next_type().unwrap();
                    target_id = target.next_type().unwrap();
                    continue;
                }
            }
            _ => panic!("this shouldn't be reached"),
        }
    }

    Err(BtfError::MaximumTypeDepthReached { type_id: local_id })
}

pub(crate) fn fields_are_compatible(
    local_btf: &Btf,
    mut local_id: u32,
    target_btf: &Btf,
    mut target_id: u32,
) -> Result<bool, BtfError> {
    for _ in 0..MAX_RESOLVE_DEPTH {
        local_id = local_btf.resolve_type(local_id)?;
        target_id = target_btf.resolve_type(target_id)?;
        let local_ty = local_btf.type_by_id(local_id)?;
        let target_ty = target_btf.type_by_id(target_id)?;

        if local_ty.is_composite() && target_ty.is_composite() {
            return Ok(true);
        }

        if local_ty.kind() != target_ty.kind() {
            return Ok(false);
        }

        use BtfType::*;
        match local_ty {
            Fwd(_) | Enum(_) => {
                let flavorless_name =
                    |name: &str| name.split_once("___").map_or(name, |x| x.0).to_string();

                let local_name = flavorless_name(&*local_btf.type_name(local_ty)?.unwrap());
                let target_name = flavorless_name(&*target_btf.type_name(target_ty)?.unwrap());

                return Ok(local_name == target_name);
            }
            Int(local) => {
                if let Int(target) = target_ty {
                    return Ok(local.offset() == 0 && target.offset() == 0);
                }
            }
            Float(_) => return Ok(true),
            Ptr(_) => return Ok(true),
            Array(local) => {
                if let Array(target) = target_ty {
                    local_id = local.next_type();
                    target_id = target.next_type();

                    continue;
                }
            }
            _ => panic!("this shouldn't be reached"),
        }
    }

    Err(BtfError::MaximumTypeDepthReached { type_id: local_id })
}

impl std::fmt::Debug for btf_type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("btf_type")
            .field("name_off", &self.name_off)
            .field("info", &self.info)
            .field("__bindgen_anon_1", &self.__bindgen_anon_1)
            .finish()
    }
}

impl std::fmt::Debug for btf_type__bindgen_ty_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Safety: union
        f.debug_struct("btf_type__bindgen_ty_1")
            .field("size", unsafe { &self.size })
            .field("type_", unsafe { &self.type_ })
            .finish()
    }
}

fn bytes_of<T>(val: &T) -> &[u8] {
    // Safety: all btf types are POD
    unsafe { crate::util::bytes_of(val) }
}
#[cfg(test)]
mod tests {
    use crate::generated::BTF_INT_SIGNED;

    use super::*;

    #[test]
    fn test_read_btf_type_int() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x00, 0x40, 0x00,
            0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Int(ty, nr_bits)) => {
                assert_eq!(ty.name_off, 1);
                assert_eq!(unsafe { ty.__bindgen_anon_1.size }, 8);
                assert_eq!(nr_bits, 64);
            }
            Ok(t) => panic!("expected int type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice());
    }

    #[test]
    fn test_write_btf_long_unsigned_int() {
        let data: &[u8] = &[
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x00, 0x40, 0x00,
            0x00, 0x00,
        ];
        let int = BtfType::new_int(1, 8, 0, 0);
        assert_eq!(int.to_bytes(), data);
    }

    #[test]
    fn test_write_btf_uchar() {
        let data: &[u8] = &[
            0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00,
            0x00, 0x00,
        ];
        let int = BtfType::new_int(0x13, 1, 0, 0);
        assert_eq!(int.to_bytes(), data);
    }

    #[test]
    fn test_write_btf_signed_short_int() {
        let data: &[u8] = &[
            0x4a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x10, 0x00,
            0x00, 0x01,
        ];
        let int = BtfType::new_int(0x4a, 2, BTF_INT_SIGNED, 0);
        assert_eq!(int.to_bytes(), data);
    }

    #[test]
    fn test_read_btf_type_ptr() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x06, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Ptr(_)) => {}
            Ok(t) => panic!("expected ptr type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_array() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Array(_, _)) => {}
            Ok(t) => panic!("expected array type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_struct() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x04, 0x04, 0x00, 0x00, 0x00, 0x47, 0x02,
            0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Struct(_, _)) => {}
            Ok(t) => panic!("expected struct type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_union() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x05, 0x04, 0x00, 0x00, 0x00, 0x0d, 0x04,
            0x00, 0x00, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Union(_, _)) => {}
            Ok(t) => panic!("expected union type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_enum() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x06, 0x04, 0x00, 0x00, 0x00, 0xc9, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Enum(_, _)) => {}
            Ok(t) => panic!("expected enum type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_fwd() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x0b, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Fwd(_)) => {}
            Ok(t) => panic!("expected fwd type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_typedef() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0b, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Typedef(_)) => {}
            Ok(t) => panic!("expected typedef type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_volatile() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x24, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Volatile(_)) => {}
            Ok(t) => panic!("expected volatile type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_const() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Const(_)) => {}
            Ok(t) => panic!("expected const type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_restrict() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x04, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Restrict(_)) => {}
            Ok(t) => panic!("expected restrict type gpt {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_func() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x17, 0x8b, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x0c, 0xf0, 0xe4, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Func(_)) => {}
            Ok(t) => panic!("expected func type gpt {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_func_proto() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::FuncProto(_, _)) => {}
            Ok(t) => panic!("expected func_proto type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_func_var() {
        let endianness = Endianness::default();
        // NOTE: There was no data in /sys/kernell/btf/vmlinux for this type
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Var(_, _)) => {}
            Ok(t) => panic!("expected var type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        };
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_func_datasec() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0xd9, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match &got {
            Ok(BtfType::DataSec(ty, info)) => {
                assert_eq!(0, unsafe { ty.__bindgen_anon_1.size } as usize);
                assert_eq!(1, type_vlen(ty) as usize);
                assert_eq!(1, info.len());
                assert_eq!(11, info[0].type_);
                assert_eq!(0, info[0].offset);
                assert_eq!(4, info[0].size);
            }
            Ok(t) => panic!("expected datasec type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_read_btf_type_float() {
        let endianness = Endianness::default();
        let data: &[u8] = &[
            0x78, 0xfd, 0x02, 0x00, 0x00, 0x00, 0x00, 0x10, 0x08, 0x00, 0x00, 0x00,
        ];
        let got = unsafe { BtfType::read(data, endianness) };
        match got {
            Ok(BtfType::Float(_)) => {}
            Ok(t) => panic!("expected float type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
        let data2 = got.unwrap().to_bytes();
        assert_eq!(data, data2.as_slice())
    }

    #[test]
    fn test_write_btf_func_proto() {
        let params = vec![
            btf_param {
                name_off: 1,
                type_: 1,
            },
            btf_param {
                name_off: 3,
                type_: 1,
            },
        ];
        let func_proto = BtfType::new_func_proto(params, 2);
        let data = func_proto.to_bytes();
        let got = unsafe { BtfType::read(&data, Endianness::default()) };
        match got {
            Ok(BtfType::FuncProto(btf_type, _params)) => {
                assert_eq!(type_vlen(&btf_type), 2);
            }
            Ok(t) => panic!("expected func proto type, got {:#?}", t),
            Err(_) => panic!("unexpected error"),
        }
    }

    #[test]
    fn test_types_are_compatible() {
        let mut btf = Btf::new();
        let name_offset = btf.add_string("u32".to_string());
        let u32t = btf.add_type(BtfType::new_int(name_offset, 4, 0, 0));
        let name_offset = btf.add_string("u64".to_string());
        let u64t = btf.add_type(BtfType::new_int(name_offset, 8, 0, 0));
        let name_offset = btf.add_string("widgets".to_string());
        let array_type = btf.add_type(BtfType::new_array(name_offset, u64t, u32t, 16));

        assert!(types_are_compatible(&btf, u32t, &btf, u32t).unwrap());
        // int types are compatible if offsets match. size and encoding aren't compared
        assert!(types_are_compatible(&btf, u32t, &btf, u64t).unwrap());
        assert!(types_are_compatible(&btf, array_type, &btf, array_type).unwrap());
    }
}
