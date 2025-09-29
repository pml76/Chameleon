#[cxx::bridge]
mod ffi_icu {
    unsafe extern "C++" {
        include!("chameleon/cpp/icu_includes.h");

        type Locale;

    }

    #[derive(Copy)]
    struct LocaleInformation {
        locale: *const Locale,
        locale_name: String,
        locale_display_name: String,
    }

    unsafe extern "C++" {
        include!("chameleon/cpp/locale.h");

        type OutputFor = crate::format::OutputFor;
        fn get_locale_information(output_for: OutputFor) -> Vec<LocaleInformation>;
    }
}

pub use ffi_icu::*;

