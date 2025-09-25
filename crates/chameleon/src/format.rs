#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("chameleon/cpp/format.h");
        fn format_f64(fmt_str: &String, d: &f64) -> String;
        
        fn get_available_number_locales() -> Vec<String>;

    }
}
#[inline]
pub fn format_f64(fmt_str: &String, d: &f64) -> String {
    ffi::format_f64(fmt_str, d)
}

#[inline]
pub fn get_available_number_locales() -> Vec<String> {
    ffi::get_available_number_locales()
}
