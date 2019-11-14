#![no_std]

pub type JSValue = f64;

pub type JSType = i32;

pub const UNDEFINED: JSValue = 0.0;
pub const FALSE: JSValue = 0.0;
pub const TRUE: JSValue = 1.0;
pub const NULL: JSValue = 1.0;
pub const CONSOLE: JSValue = 2.0;
pub const WINDOW: JSValue = 3.0;
pub const DOCUMENT: JSValue = 4.0;

pub const TYPE_NOTHING: JSType = 0;
pub const TYPE_NUM: JSType = 1;
pub const TYPE_STRING: JSType = 2;
pub const TYPE_BOOL: JSType = 3;
pub const TYPE_FUNCTION: JSType = 4;
pub const TYPE_OBJECT: JSType = 5;
pub const TYPE_UINT8_ARRAY: JSType = 6;
pub const TYPE_INT8_ARRAY: JSType = 7;
pub const TYPE_UINT8CLAMPED_ARRAY: JSType = 8;
pub const TYPE_INT16_ARRAY: JSType = 9;
pub const TYPE_UINT16_ARRAY: JSType = 10;
pub const TYPE_INT32_ARRAY: JSType = 11;
pub const TYPE_UINT32_ARRAY: JSType = 12;
pub const TYPE_F32_ARRAY: JSType = 13;
pub const TYPE_F64_ARRAY: JSType = 14;
pub const TYPE_BI64_ARRAY: JSType = 15;
pub const TYPE_BUI64_ARRAY: JSType = 16;
pub const TYPE_MEMORY: JSType = 17;

pub trait ToJSValue {
    #[inline(Always)]
    fn to_js_value(&self) -> JSValue;
}

impl ToJSValue for JSValue {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }
}
