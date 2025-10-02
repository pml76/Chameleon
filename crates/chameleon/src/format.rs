use crate::locale;

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("chameleon/cpp/icu_includes.h");
        type Locale = crate::locale::Locale;

    }

    #[repr(i32)]
    enum OutputKind {
        DisplayName = 0,
        FullName = 1,
        BaseName = 2,
    }
    unsafe extern "C++" {
        include!("chameleon/cpp/enums.h");
        type OutputKind;
    }

    #[repr(i32)]
    enum OutputFor {
        AllLocales = 0,
        NumberFormatLocales = 1,
        DateFormatLocales = 2,
    }

    extern "C++" {
        include!("chameleon/cpp/enums.h");
        type OutputFor;

    }

    unsafe extern "C++" {
        include!("chameleon/cpp/format.h");

        unsafe fn format_f64(locale: *const Locale, d: &f64) -> String;
        

    }

}

pub use ffi::*;
