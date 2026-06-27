use js_sys::{ArrayBuffer, Iterator, JsString, Object, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Uint8Array)]
    #[derive(Clone, Debug)]
    pub type Buffer;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Buffer)]
    pub fn alloc(size: f64, fill: Option<&Buffer>, encoding: Option<&JsString>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "allocUnsafe")]
    pub fn alloc_unsafe(size: f64) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "byteLength")]
    pub fn byte_length(string: &JsString, encoding: Option<&JsString>) -> f64;

    // FIXME: compiler complains about
    #[wasm_bindgen(static_method_of = Buffer, js_name = "compare")]
    pub fn compare_(buf1: &Buffer, buf2: &Buffer) -> i32;

    #[wasm_bindgen(static_method_of = Buffer)]
    pub fn concat(list: Box<[JsValue]>, total_length: Option<f64>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "isBuffer")]
    pub fn is_buffer(value: &JsValue) -> bool;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "isEncoding")]
    pub fn is_encoding(encoding: &JsString) -> bool;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array(array: Box<[JsValue]>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array_buffer(
        buffer: &ArrayBuffer,
        byte_offset: Option<f64>,
        length: Option<f64>,
    ) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array_uint8(array: &Uint8Array) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_string(string: &JsString, encoding: Option<&JsString>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, getter, js_name = "poolSize")]
    pub fn pool_size() -> f64;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn compare(
        this: &Buffer,
        target: &Buffer,
        target_start: Option<f64>,
        target_end: Option<f64>,
        source_start: Option<f64>,
        source_end: Option<f64>,
    ) -> i32;

    #[wasm_bindgen(method)]
    pub fn copy(
        this: &Buffer,
        target: &Buffer,
        target_start: Option<f64>,
        source_start: Option<f64>,
        source_end: Option<f64>,
    ) -> f64;

    #[wasm_bindgen(method)]
    pub fn entries(this: &Buffer) -> Iterator;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Buffer, that: &Buffer) -> bool;

    #[wasm_bindgen(method)]
    pub fn fill(
        this: &Buffer,
        value: &JsValue,
        offset: Option<f64>,
        end: Option<f64>,
        encoding: Option<JsString>,
    ) -> Buffer;

    #[wasm_bindgen(method)]
    pub fn includes(
        this: &Buffer,
        value: &JsValue,
        offset: Option<f64>,
        encoding: Option<&JsString>,
    ) -> bool;

    #[wasm_bindgen(method, js_name = "indexOf")]
    pub fn index_of(
        this: &Buffer,
        value: &JsValue,
        offset: Option<f64>,
        encoding: Option<&JsString>,
    ) -> f64;

    #[wasm_bindgen(method)]
    pub fn keys(this: &Buffer) -> Iterator;

    #[wasm_bindgen(method, js_name = "lastIndexOf")]
    pub fn last_index_of(
        this: &Buffer,
        value: &JsValue,
        offset: Option<f64>,
        encoding: Option<&JsString>,
    ) -> f64;

    #[wasm_bindgen(method, js_name = "readBigInt64BE")]
    pub fn read_big_int64_be(this: &Buffer, offset: Option<f64>) -> i64;

    #[wasm_bindgen(method, js_name = "readBigInt64LE")]
    pub fn read_big_int64_le(this: &Buffer, offset: Option<f64>) -> i64;

    #[wasm_bindgen(method, js_name = "readBigUInt64BE")]
    pub fn read_big_uint64_be(this: &Buffer, offset: Option<f64>) -> u64;

    #[wasm_bindgen(method, js_name = "readBigUInt64LE")]
    pub fn read_big_uint64_le(this: &Buffer, offset: Option<f64>) -> u64;

    #[wasm_bindgen(method, js_name = "readDoubleBE")]
    pub fn read_double_be(this: &Buffer, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "readDoubleLE")]
    pub fn read_double_le(this: &Buffer, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "readFloatBE")]
    pub fn read_float_be(this: &Buffer, offset: Option<f64>) -> f32;

    #[wasm_bindgen(method, js_name = "readFloatLE")]
    pub fn read_float_le(this: &Buffer, offset: Option<f64>) -> f32;

    #[wasm_bindgen(method, js_name = "readIntBE")]
    pub fn read_int_be(this: &Buffer, offset: f64, length: u8) -> f64;

    #[wasm_bindgen(method, js_name = "readIntLE")]
    pub fn read_int_le(this: &Buffer, offset: f64, length: u8) -> f64;

    #[wasm_bindgen(method, js_name = "readInt8")]
    pub fn read_int8(this: &Buffer, offset: Option<f64>) -> i8;

    #[wasm_bindgen(method, js_name = "readInt16BE")]
    pub fn read_int16_be(this: &Buffer, offset: Option<f64>) -> i16;

    #[wasm_bindgen(method, js_name = "readInt16LE")]
    pub fn read_int16_le(this: &Buffer, offset: Option<f64>) -> i16;

    #[wasm_bindgen(method, js_name = "readInt32BE")]
    pub fn read_int32_be(this: &Buffer, offset: Option<f64>) -> i32;

    #[wasm_bindgen(method, js_name = "readInt32LE")]
    pub fn read_int32_le(this: &Buffer, offset: Option<f64>) -> i32;

    #[wasm_bindgen(method, js_name = "readUIntBE")]
    pub fn read_uint_be(this: &Buffer, offset: f64, length: u8) -> f64;

    #[wasm_bindgen(method, js_name = "readUIntLE")]
    pub fn read_uint_le(this: &Buffer, offset: f64, length: u8) -> f64;

    #[wasm_bindgen(method, js_name = "readUInt8")]
    pub fn read_uint8(this: &Buffer, offset: Option<f64>) -> u8;

    #[wasm_bindgen(method, js_name = "readUInt16BE")]
    pub fn read_uint16_be(this: &Buffer, offset: Option<f64>) -> u16;

    #[wasm_bindgen(method, js_name = "readUInt16LE")]
    pub fn read_uint16_le(this: &Buffer, offset: Option<f64>) -> u16;

    #[wasm_bindgen(method, js_name = "readUInt32BE")]
    pub fn read_uint32_be(this: &Buffer, offset: Option<f64>) -> u32;

    #[wasm_bindgen(method, js_name = "readUInt32LE")]
    pub fn read_uint32_le(this: &Buffer, offset: Option<f64>) -> u32;

    #[wasm_bindgen(method)]
    pub fn slice(this: &Buffer, start: Option<f64>, end: Option<f64>) -> Buffer;

    #[wasm_bindgen(method)]
    pub fn subarray(this: &Buffer, start: Option<f64>, end: Option<f64>) -> Buffer;

    #[wasm_bindgen(method)]
    pub fn swap16(this: &Buffer) -> Buffer;

    #[wasm_bindgen(method)]
    pub fn swap32(this: &Buffer) -> Buffer;

    #[wasm_bindgen(method)]
    pub fn swap64(this: &Buffer) -> Buffer;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Buffer) -> Object;

    #[wasm_bindgen(method, js_name = "toString")]
    pub fn to_string(
        this: &Buffer,
        encoding: Option<JsString>,
        start: Option<f64>,
        end: Option<f64>,
    ) -> JsString;

    #[wasm_bindgen(method)]
    pub fn values(this: &Buffer) -> Iterator;

    #[wasm_bindgen(method)]
    pub fn write(
        this: &Buffer,
        string: &JsString,
        offset: Option<f64>,
        length: Option<f64>,
        encoding: Option<&JsString>,
    ) -> f64;

    #[wasm_bindgen(method, js_name = "writeBigInt64BE")]
    pub fn write_big_int64_be(this: &Buffer, value: i64, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeBigInt64LE")]
    pub fn write_big_int64_le(this: &Buffer, value: i64, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeBigUInt64BE")]
    pub fn write_big_uint64_be(this: &Buffer, value: u64, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeBigUInt64LE")]
    pub fn write_big_uint64_le(this: &Buffer, value: u64, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeDoubleBE")]
    pub fn write_double_be(this: &Buffer, value: f64, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeDoubleLE")]
    pub fn write_double_le(this: &Buffer, value: f64, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeFloatBE")]
    pub fn write_float_be(this: &Buffer, value: f32, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeFloatLE")]
    pub fn write_float_le(this: &Buffer, value: f32, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeInt8")]
    pub fn write_int8(this: &Buffer, value: i8, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeInt16BE")]
    pub fn write_int16_be(this: &Buffer, value: i16, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeInt16LE")]
    pub fn write_int16_le(this: &Buffer, value: i16, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeInt32BE")]
    pub fn write_int32_be(this: &Buffer, value: i32, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeInt32LE")]
    pub fn write_int32_le(this: &Buffer, value: i32, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeUInt8")]
    pub fn write_uint8(this: &Buffer, value: u8, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeUInt16BE")]
    pub fn write_uint16_be(this: &Buffer, value: u16, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeUInt16LE")]
    pub fn write_uint16_le(this: &Buffer, value: u16, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeUInt32BE")]
    pub fn write_uint32_be(this: &Buffer, value: u32, offset: Option<f64>) -> f64;

    #[wasm_bindgen(method, js_name = "writeUInt32LE")]
    pub fn write_uint32_le(this: &Buffer, value: u32, offset: Option<f64>) -> f64;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn length(this: &Buffer) -> f64;
}
