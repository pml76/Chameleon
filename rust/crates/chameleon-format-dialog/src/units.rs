#[cxx_qt::bridge]
mod ffi_icu {

    unsafe extern "C++" {
        include!("chameleon-format-dialog/cpp/units.h");

        fn get_unit_types() -> Vec<String>;
        fn get_available_units_for_type( unit_type: &String  ) -> Vec<String>;
        
    }
}

pub use ffi_icu::*;
