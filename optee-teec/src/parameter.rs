use optee_teec_sys as raw;
use std::mem;

pub trait Param {
    fn into_raw(&mut self) -> raw::TEEC_Parameter;
    fn param_type(&self) -> ParamType;
    fn from_raw(raw: raw::TEEC_Parameter, param_type: ParamType) -> Self;
}

pub struct ParamValue {
    raw: raw::TEEC_Value,
    param_type: ParamType,
}

impl ParamValue {
    pub fn new(a: u32, b: u32, param_type: ParamType) -> Self {
        let raw = raw::TEEC_Value { a, b };
        Self { raw, param_type }
    }
    pub fn a(&self) -> u32 {
        self.raw.a
    }
    pub fn b(&self) -> u32 {
        self.raw.b
    }
}

impl Param for ParamValue {
    fn into_raw(&mut self) -> raw::TEEC_Parameter {
        raw::TEEC_Parameter {
            value: self.raw
        }
    }

    fn from_raw(raw: raw::TEEC_Parameter, param_type: ParamType) -> Self {
        Self { raw: unsafe { raw.value }, param_type: param_type }
    }

    fn param_type(&self) -> ParamType {
        self.param_type
    }
}

pub struct ParamNone;

impl Param for ParamNone {
    fn into_raw(&mut self) -> raw::TEEC_Parameter {
        let raw: raw::TEEC_Parameter = unsafe { mem::zeroed() };
        raw
    }

    fn param_type(&self) -> ParamType {
        ParamType::None
    }

    fn from_raw(_raw: raw::TEEC_Parameter, _param_type: ParamType) -> Self {
        Self
    }
}

pub struct ParamTmpRef<'a> {
    raw: raw::TEEC_TempMemoryReference,
    buffer: &'a mut [u8],
    param_type: ParamType,
}

impl<'a> ParamTmpRef<'a> {
    pub fn new(buffer: &'a mut [u8], param_type: ParamType) -> Self {
        let raw = raw::TEEC_TempMemoryReference {
            buffer: buffer.as_ptr() as _,
            size: buffer.len(),
        };
        Self { raw, buffer, param_type }
    }

    pub fn buffer(&self) -> &[u8] {
        self.buffer
    }
}

impl<'a> Param for ParamTmpRef<'a> {
    fn into_raw(&mut self) -> raw::TEEC_Parameter {
        raw::TEEC_Parameter {
            tmpref: self.raw
        }
    }

    fn param_type(&self) -> ParamType {
        self.param_type
    }

    fn from_raw(raw: raw::TEEC_Parameter, param_type: ParamType) -> Self {
        let buffer: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(raw.tmpref.buffer as *mut u8, raw.tmpref.size)
        };
        Self {
            raw: unsafe { raw.tmpref },
            buffer: buffer,
            param_type: param_type,
        }
    }
}

/// These are used to indicate the type of Parameter encoded inside the
/// operation structure.
#[derive(Copy, Clone)]
pub enum ParamType {
    /// The Parameter is not used.
    None = 0,
    /// The Parameter is a TEEC_Value tagged as input.
    ValueInput = 1,
    /// The Parameter is a TEEC_Value tagged as output.
    ValueOutput = 2,
    /// The Parameter is a TEEC_Value tagged as both as input and output, i.e.,
    /// for which both the behaviors of ValueInput and ValueOutput apply.
    ValueInout = 3,
    /// The Parameter is a TEEC_TempMemoryReference describing a region of
    /// memory which needs to be temporarily registered for the duration of the
    /// Operation and is tagged as input.
    MemrefTempInput = 5,
    /// Same as MemrefTempInput, but the Memory Reference is tagged as
    /// output. The Implementation may update the size field to reflect the
    /// required output size in some use cases.
    MemrefTempOutput = 6,
    /// A Temporary Memory Reference tagged as both input and output, i.e., for
    /// which both the behaviors of MemrefTempInput and MemrefTempOutput apply.
    MemrefTempInout = 7,
    /// The Parameter is a Registered Memory Reference that refers to the
    /// entirety of its parent Shared Memory block. The parameter structure is a
    /// TEEC_MemoryReference. In this structure, the Implementation MUST read
    /// only the parent field and MAY update the size field when the operation
    /// completes.
    MemrefWhole = 0xC,
    /// A Registered Memory Reference structure that refers to a partial region
    /// of its parent Shared Memory block and is tagged as input.
    MemrefPartialInput = 0xD,
    /// A Registered Memory Reference structure that refers to a partial region
    /// of its parent Shared Memory block and is tagged as output.
    MemrefPartialOutput = 0xE,
    /// The Registered Memory Reference structure that refers to a partial
    /// region of its parent Shared Memory block and is tagged as both input and
    /// output, i.e., for which both the behaviors of MemrefPartialInput and
    /// MemrefPartialOutput apply.
    MemrefPartialInout = 0xF,
}

impl From<u32> for ParamType {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamType::None,
            1 => ParamType::ValueInput,
            2 => ParamType::ValueOutput,
            3 => ParamType::ValueInout,
            5 => ParamType::MemrefTempInput,
            6 => ParamType::MemrefTempOutput,
            7 => ParamType::MemrefTempInout,
            0xC => ParamType::MemrefWhole,
            0xD => ParamType::MemrefPartialInput,
            0xE => ParamType::MemrefPartialOutput,
            0xF => ParamType::MemrefPartialInout,
            _ => ParamType::None,
        }
    }
}

pub struct ParamTypes(u32);

impl ParamTypes {
    pub fn new(p0: ParamType, p1: ParamType, p2: ParamType, p3: ParamType) -> Self {
        ParamTypes((p0 as u32) | (p1 as u32) << 4 | (p2 as u32) << 8 | (p3 as u32) << 12)
    }

    pub fn into_flags(&self) -> (ParamType, ParamType, ParamType, ParamType) {
        (
            (0x000fu32 & self.0).into(),
            (0x00f0u32 & self.0).into(),
            (0x0f00u32 & self.0).into(),
            (0xf000u32 & self.0).into(),
        )
    }
}

impl From<u32> for ParamTypes {
    fn from(value: u32) -> Self {
        ParamTypes(value)
    }
}

impl From<[u32; 4]> for ParamTypes {
    fn from(param_types: [u32; 4]) -> Self {
        ParamTypes(
            param_types[0] | param_types[1] << 4 | param_types[2] << 8 | param_types[3] << 12,
        )
    }
}

impl From<ParamTypes> for u32 {
    fn from(a: ParamTypes) -> u32 {
        a.0
    }
}
