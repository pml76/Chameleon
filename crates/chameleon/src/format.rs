#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("chameleon/cpp/format.h");
        fn format_f64(fmt_str: &String, d: &f64) -> String;

        fn format_f32(fmt_str: &String, f: &f32) -> String;

        fn format_bool(fmt_str: &String, b: &bool) -> String;

        fn format_i8(fmt_str: &String, i: &i8) -> String;
        fn format_i16(fmt_str: &String, i: &i16) -> String;
        fn format_i32(fmt_str: &String, i: &i32) -> String;
        fn format_i64(fmt_str: &String, i: &i64) -> String;

        fn format_u8(fmt_str: &String, i: &u8) -> String;
        fn format_u16(fmt_str: &String, i: &u16) -> String;
        fn format_u32(fmt_str: &String, i: &u32) -> String;
        fn format_u64(fmt_str: &String, i: &u64) -> String;

        fn format_str(fmt_str: &String, s: &String) -> String;
    }
}
#[inline]
pub(crate) fn format_f64(fmt_str: &String, d: &f64) -> String {
    ffi::format_f64(fmt_str, d)
}
#[inline]
pub(crate) fn format_f32(fmt_str: &String, f: &f32) -> String {
    ffi::format_f32(fmt_str, f)
}
#[inline]
pub(crate) fn format_i8(fmt_str: &String, i: &i8) -> String {
    ffi::format_i8(fmt_str, i)
}
#[inline]
pub fn format_i16(fmt_str: &String, i: &i16) -> String {
    ffi::format_i16(fmt_str, i)
}
#[inline]
pub fn format_i32(fmt_str: &String, i: &i32) -> String {
    ffi::format_i32(fmt_str, i)
}
#[inline]
pub fn format_i64(fmt_str: &String, i: &i64) -> String {
    ffi::format_i64(fmt_str, i)
}
#[inline]
pub fn format_u8(fmt_str: &String, i: &u8) -> String {
    ffi::format_u8(fmt_str, i)
}
#[inline]
pub fn format_u16(fmt_str: &String, i: &u16) -> String {
    ffi::format_u16(fmt_str, i)
}
#[inline]
pub fn format_u32(fmt_str: &String, i: &u32) -> String {
    ffi::format_u32(fmt_str, i)
}
#[inline]
pub fn format_u64(fmt_str: &String, i: &u64) -> String {
    ffi::format_u64(fmt_str, i)
}
#[inline]
pub fn format_str(fmt_str: &String, s: &String) -> String {
    ffi::format_str(fmt_str, s)
}
#[inline]
pub fn format_bool(fmt_str: &String, b: &bool) -> String {
    ffi::format_bool(fmt_str, b)
}
