use crate::locale;

#[cxx::bridge]
mod ffi {

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

        //type Locale = crate::locale::Locale;

        fn format_f64(fmt_str: &String, d: &f64) -> String;
        

    }

}

pub use ffi::*;
