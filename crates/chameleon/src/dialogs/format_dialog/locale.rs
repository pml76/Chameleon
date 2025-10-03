#[cxx_qt::bridge]
mod ffi_icu {
    unsafe extern "C++" {
        include!("chameleon/cpp/includes/icu_includes.h");

        type Locale;

    }


    #[derive(Clone)]
    struct LocaleInformation {
        locale: *const Locale,
        locale_name: String,
        locale_display_name: String,
    }

    unsafe extern "C++" {
        include!("chameleon/cpp/dialogs/format_dialog/locale.h");

        type OutputFor = crate::dialogs::format_dialog::format::OutputFor;
        fn get_locale_information(output_for: OutputFor) -> Vec<LocaleInformation>;
    }
}

pub use ffi_icu::*;

