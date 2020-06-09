use crate::{Error, ErrorKind, Result};
use optee_utee_sys as raw;
use std::marker;
use std::convert::TryInto;

pub struct Parameters(pub Parameter, pub Parameter, pub Parameter, pub Parameter);

impl Parameters {
    pub fn from_raw(tee_params: &mut [raw::TEE_Param; 4], param_types: u32) -> Self {
        let (f0, f1, f2, f3) = ParamTypes::from(param_types).into_flags();
        let p0 = Parameter::from_raw(&mut tee_params[0], f0);
        let p1 = Parameter::from_raw(&mut tee_params[1], f1);
        let p2 = Parameter::from_raw(&mut tee_params[2], f2);
        let p3 = Parameter::from_raw(&mut tee_params[3], f3);

        Parameters(p0, p1, p2, p3)
    }
}

pub struct DifferentParameters(pub DifferentParameter, pub DifferentParameter, pub DifferentParameter, pub DifferentParameter);

pub struct ParamValue<'parameter> {
    raw: *mut raw::Value,
    param_type: ParamType,
    _marker: marker::PhantomData<&'parameter mut u32>,
}

impl<'parameter> ParamValue<'parameter> {
    pub fn a(&self) -> u32 {
        unsafe { (*self.raw).a }
    }

    pub fn b(&self) -> u32 {
        unsafe { (*self.raw).b }
    }

    pub fn set_a(&mut self, a: u32) {
        unsafe {
            (*self.raw).a = a;
        }
    }

    pub fn set_b(&mut self, b: u32) {
        unsafe {
            (*self.raw).b = b;
        }
    }

    pub fn param_type(&self) -> ParamType {
        self.param_type
    }
}

pub struct ParamMemref<'parameter> {
    raw: *mut raw::Memref,
    param_type: ParamType,
    _marker: marker::PhantomData<&'parameter mut [u8]>,
}

impl<'parameter> ParamMemref<'parameter> {
    pub fn buffer(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut((*self.raw).buffer as *mut u8, (*self.raw).size as usize)
        }
    }

    pub fn param_type(&self) -> ParamType {
        self.param_type
    }

    pub fn raw(&mut self) -> *mut raw::Memref {
        self.raw
    }

    pub fn set_updated_size(&mut self, size: usize) {
        unsafe { (*self.raw).size = size as u32};
    }
}

pub struct Parameter {
    pub raw: *mut raw::TEE_Param,
    pub param_type: ParamType,
}


impl Parameter {
    pub fn from_raw(ptr: *mut raw::TEE_Param, param_type: ParamType) -> Self {
        Self {
            raw: ptr,
            param_type: param_type,
        }
    }

    pub unsafe fn as_value(&mut self) -> Result<ParamValue> {
        match self.param_type {
            ParamType::ValueInput | ParamType::ValueInout | ParamType::ValueOutput => {
                Ok(ParamValue {
                    raw: &mut (*self.raw).value,
                    param_type: self.param_type,
                    _marker: marker::PhantomData,
                })
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub unsafe fn as_memref(&mut self) -> Result<ParamMemref> {
        match self.param_type {
            ParamType::MemrefInout | ParamType::MemrefInput | ParamType::MemrefOutput => {
                Ok(ParamMemref {
                    raw: &mut (*self.raw).memref,
                    param_type: self.param_type,
                    _marker: marker::PhantomData,
                })
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn raw(&self) -> *mut raw::TEE_Param {
        self.raw
    }
}

pub struct DifferentParameter {
    pub raw: raw::TEE_Param,
    pub param_type: ParamType,
}

impl DifferentParameter {
    pub fn from_vec(source: &mut Vec<u8>, param_type: ParamType) -> Result<Self> {
        let mut raw = raw::TEE_Param {
            memref: raw::Memref {
                buffer: source.as_mut_ptr() as *mut libc::c_void,
                size: source.len().try_into().map_err(|err| {
                    trace_println!("optee_utee::Parameter::from_vec try_into failed from usize to u32:{:?}", err);
                    ErrorKind::TargetDead
                })?,
            },
        };
        trace_println!("address of raw:{:p}", &raw);
        Ok(Self {
            raw: raw.clone(),
            param_type: param_type,
        })
    }

    pub fn from_values(a: u32, b: u32, param_type: ParamType) -> Self {
        let mut raw = raw::TEE_Param {
            value: raw::Value {
                a: a,
                b: b,
            },
        };
        Self {
            raw: raw.clone(),
            param_type: param_type,
        }
    }

    pub unsafe fn as_value(&mut self) -> Result<ParamValue> {
        match self.param_type {
            ParamType::ValueInput | ParamType::ValueInout | ParamType::ValueOutput => {
                Ok(ParamValue {
                    raw: &mut self.raw.value,
                    param_type: self.param_type,
                    _marker: marker::PhantomData,
                })
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub unsafe fn as_memref(&mut self) -> Result<ParamMemref> {
        match self.param_type {
            ParamType::MemrefInout | ParamType::MemrefInput | ParamType::MemrefOutput => {
                Ok(ParamMemref {
                    raw: &mut self.raw.memref,
                    param_type: self.param_type,
                    _marker: marker::PhantomData,
                })
            }
            _ => Err(Error::new(ErrorKind::BadParameters)),
        }
    }

    pub fn raw(&mut self) -> *mut raw::TEE_Param {
        &mut self.raw
    }
}

pub struct ParamTypes(u32);

impl ParamTypes {
    pub fn into_flags(&self) -> (ParamType, ParamType, ParamType, ParamType) {
        (
            (0x000fu32 & self.0).into(),
            ((0x00f0u32 & self.0) >> 4).into(),
            ((0x0f00u32 & self.0) >> 8).into(),
            ((0xf000u32 & self.0) >> 12).into(),
        )
    }
}

impl From<u32> for ParamTypes {
    fn from(value: u32) -> Self {
        ParamTypes(value)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ParamType {
    None = 0,
    ValueInput = 1,
    ValueOutput = 2,
    ValueInout = 3,
    MemrefInput = 5,
    MemrefOutput = 6,
    MemrefInout = 7,
}

impl From<u32> for ParamType {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamType::None,
            1 => ParamType::ValueInput,
            2 => ParamType::ValueOutput,
            3 => ParamType::ValueInout,
            5 => ParamType::MemrefInput,
            6 => ParamType::MemrefOutput,
            7 => ParamType::MemrefInout,
            _ => ParamType::None,
        }
    }
}
