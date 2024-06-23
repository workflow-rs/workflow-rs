//! buffer slicing and other utilities

/// Takes a `&[u8]` buffer slice and returns a slice `&[T]`
/// with a given number of elements of type `T`
pub fn buffer_as_slice<'data, T: 'data>(
    data: &'data [u8],
    byte_offset: usize,
    elements: usize,
) -> &'data [T] {
    unsafe {
        std::slice::from_raw_parts::<T>(
            std::mem::transmute::<*const u8, *const T>(data.as_ptr().add(byte_offset)),
            elements,
        )
    }
}

/// Takes a mutable `&[u8]` buffer slice and returns a
/// mutable slice `&[T]` with a given number of elements
/// of type `T`
pub fn buffer_as_slice_mut<'data, T: 'data>(
    data: &'data mut [u8],
    byte_offset: usize,
    elements: usize,
) -> &mut [T] {
    unsafe {
        std::slice::from_raw_parts_mut::<T>(
            std::mem::transmute::<*mut u8, *mut T>(data.as_mut_ptr().add(byte_offset)),
            elements,
        )
    }
}

/// Takes a reference to a struct of type `T` and returns
/// a raw `&[u8]` slice with byte length of the type `T`
pub fn struct_as_slice_u8<'data, T: 'data>(data: &T) -> &'data [u8] {
    unsafe {
        std::slice::from_raw_parts::<u8>(data as *const T as *const u8, std::mem::size_of::<T>())
    }
}

/// Extract a substring starting at 0 and truncating it
/// to `min(length,str.len())`.
pub fn substring(str: &str, start: usize, length: usize) -> String {
    str[start..length.min(str.len())].to_string()
}

/// Truncate a string, optionally appending another string
/// or appending `"..."` if the `append` string is `None`
pub fn substr(str: &str, start: usize, length: usize, append: Option<&str>) -> String {
    let len = str.len();
    let str = str[start..length.min(len)].to_string();
    if len > length {
        str + append.unwrap_or("...")
    } else {
        str
    }
}
