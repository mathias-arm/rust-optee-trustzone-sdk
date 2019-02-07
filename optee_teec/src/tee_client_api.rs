use libc::*;

pub const TEEC_CONFIG_PAYLOAD_REF_COUNT: uint32_t = 4;

pub const TEEC_CONFIG_SHAREDMEM_MAX_SIZE: c_ulong = -1 as c_long as c_ulong;

pub const TEEC_NONE: uint32_t                  = 0x00000000;
pub const TEEC_VALUE_INPUT: uint32_t           = 0x00000001;
pub const TEEC_VALUE_OUTPUT: uint32_t          = 0x00000002;
pub const TEEC_VALUE_INOUT: uint32_t           = 0x00000003;
pub const TEEC_MEMREF_TEMP_INPUT: uint32_t     = 0x00000005;
pub const TEEC_MEMREF_TEMP_OUTPUT: uint32_t    = 0x00000006;
pub const TEEC_MEMREF_TEMP_INOUT: uint32_t     = 0x00000007;
pub const TEEC_MEMREF_WHOLE: uint32_t          = 0x0000000C;
pub const TEEC_MEMREF_PARTIAL_INPUT: uint32_t  = 0x0000000D;
pub const TEEC_MEMREF_PARTIAL_OUTPUT: uint32_t = 0x0000000E;
pub const TEEC_MEMREF_PARTIAL_INOUT: uint32_t  = 0x0000000F;

pub const TEEC_MEM_INPUT: uint32_t  = 0x00000001;
pub const TEEC_MEM_OUTPUT: uint32_t = 0x00000002;

pub const TEEC_SUCCESS: uint32_t               = 0x00000000;
pub const TEEC_ERROR_GENERIC: uint32_t         = 0xFFFF0000;
pub const TEEC_ERROR_ACCESS_DENIED: uint32_t   = 0xFFFF0001;
pub const TEEC_ERROR_CANCEL: uint32_t          = 0xFFFF0002;
pub const TEEC_ERROR_ACCESS_CONFLICT: uint32_t = 0xFFFF0003;
pub const TEEC_ERROR_EXCESS_DATA: uint32_t     = 0xFFFF0004;
pub const TEEC_ERROR_BAD_FORMAT: uint32_t      = 0xFFFF0005;
pub const TEEC_ERROR_BAD_PARAMETERS: uint32_t  = 0xFFFF0006;
pub const TEEC_ERROR_BAD_STATE: uint32_t       = 0xFFFF0007;
pub const TEEC_ERROR_ITEM_NOT_FOUND: uint32_t  = 0xFFFF0008;
pub const TEEC_ERROR_NOT_IMPLEMENTED: uint32_t = 0xFFFF0009;
pub const TEEC_ERROR_NOT_SUPPORTED: uint32_t   = 0xFFFF000A;
pub const TEEC_ERROR_NO_DATA: uint32_t         = 0xFFFF000B;
pub const TEEC_ERROR_OUT_OF_MEMORY: uint32_t   = 0xFFFF000C;
pub const TEEC_ERROR_BUSY: uint32_t            = 0xFFFF000D;
pub const TEEC_ERROR_COMMUNICATION: uint32_t   = 0xFFFF000E;
pub const TEEC_ERROR_SECURITY: uint32_t        = 0xFFFF000F;
pub const TEEC_ERROR_SHORT_BUFFER: uint32_t    = 0xFFFF0010;
pub const TEEC_ERROR_EXTERNAL_CANCEL: uint32_t = 0xFFFF0011;
pub const TEEC_ERROR_TARGET_DEAD: uint32_t     = 0xFFFF3024;

pub const TEEC_ORIGIN_API: uint32_t         = 0x00000001;
pub const TEEC_ORIGIN_COMMS: uint32_t       = 0x00000002;
pub const TEEC_ORIGIN_TEE: uint32_t         = 0x00000003;
pub const TEEC_ORIGIN_TRUSTED_APP: uint32_t = 0x00000004;

pub const TEEC_LOGIN_PUBLIC: uint32_t            = 0x00000000;
pub const TEEC_LOGIN_USER: uint32_t              = 0x00000001;
pub const TEEC_LOGIN_GROUP: uint32_t             = 0x00000002;
pub const TEEC_LOGIN_APPLICATION: uint32_t       = 0x00000004;
pub const TEEC_LOGIN_USER_APPLICATION: uint32_t  = 0x00000005;
pub const TEEC_LOGIN_GROUP_APPLICATION: uint32_t = 0x00000006;

pub type TEEC_Result = uint32_t;

#[repr(C)]
pub struct TEEC_Context {
    fd: c_int,
    reg_mem: bool,
}

#[repr(C)]
pub struct TEEC_UUID {
    time_low: uint32_t,
    time_mid: uint16_t,
    time_hi_and_version: uint16_t,
    clock_seq_and_node: [uint8_t; 8],
}

#[repr(C)]
pub struct TEEC_Session {
    ctx: *mut TEEC_Context,
    session_id: uint32_t,
}

#[repr(C)]
pub struct TEEC_SharedMemory {
    buffer: *mut c_void,
    size: size_t,
    flags: uint32_t,
    id: c_int,
    alloced_size: size_t,
    shadow_buffer: *mut c_void,
    registered_fd: c_int,
    buffer_allocated: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEEC_TempMemoryReference {
    buffer: *mut c_void,
    size: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEEC_RegisteredMemoryReference {
    parent: *mut TEEC_SharedMemory,
    size: size_t,
    offset: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEEC_Value {
    a: uint32_t,
    b: uint32_t,
}

#[repr(C)]
pub union TEEC_Parameter {
    tmpref: TEEC_TempMemoryReference,
    memref: TEEC_RegisteredMemoryReference,
    value: TEEC_Value,
}

#[repr(C)]
pub struct TEEC_Operation {
    started: uint32_t,
    param_types: uint32_t,
    params: [TEEC_Parameter; TEEC_CONFIG_PAYLOAD_REF_COUNT as usize],
    session: *mut TEEC_Session,
}

extern "C" {
    pub fn TEEC_InitializeContext(name: *mut c_char, context: *mut TEEC_Context) -> TEEC_Result;
    pub fn TEEC_FinalizeContext(context: *mut TEEC_Context);
    pub fn TEEC_OpenSession(context: *mut TEEC_Context,
                            session: *mut TEEC_Session,
                            destination: *const TEEC_UUID,
                            connectionMethod: uint32_t,
                            connectionData: *const c_void,
                            operation: *mut TEEC_Operation,
                            returnOrigin: *mut uint32_t) -> TEEC_Result;
    pub fn TEEC_CloseSession(session: *mut TEEC_Session);
    pub fn TEEC_InvokeCommand(session: *mut TEEC_Session,
                              commandID: uint32_t,
                              operation: *mut TEEC_Operation,
                              returnOrigin: *mut uint32_t) -> TEEC_Result;
    pub fn TEEC_RegisterSharedMemory(context: *mut TEEC_Context,
                                     sharedMem: *mut TEEC_SharedMemory) -> TEEC_Result;
    pub fn TEEC_AllocateSharedMemory(context: *mut TEEC_Context,
                                     sharedMem: *mut TEEC_SharedMemory) -> TEEC_Result;
    pub fn TEEC_ReleaseSharedMemory(sharedMemory: *mut TEEC_SharedMemory);
    pub fn TEEC_RequestCancellation(operation: *mut TEEC_Operation);
}
