#[cxx_qt::bridge]
mod ffi_icu {

    #[derive(Clone)]
    struct LocaleInformation {
        locale_name: String,
        locale_display_name: String,
    }

    unsafe extern "C++" {
        include!("chameleon-format-dialog/cpp/locale.h");

        type OutputFor = crate::format::OutputFor;
        fn get_locale_information(output_for: OutputFor) -> Vec<LocaleInformation>;
        fn get_default_locale_name() -> String;
        fn get_default_locale_display_name() -> String;

    }
}

pub use ffi_icu::{get_locale_information, LocaleInformation, get_default_locale_name, get_default_locale_display_name, OutputFor};


impl Default for LocaleInformation {
    fn default() -> Self {
        let locale_name = get_default_locale_name();
        let locale_display_name = get_default_locale_display_name();
        
        LocaleInformation {
            locale_name,
            locale_display_name,
        }
    }
}