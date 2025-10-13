#[cxx_qt::bridge]
mod ffi_icu {
    unsafe extern "C++" {
        include!("chameleon-format-dialog/cpp/includes/icu_includes.h");

        type Locale;

    }


    #[derive(Clone)]
    struct LocaleInformation {
        locale: *const Locale,
        locale_name: String,
        locale_display_name: String,
    }

    unsafe extern "C++" {
        include!("chameleon-format-dialog/cpp/locale.h");

        type OutputFor = crate::format::OutputFor;
        fn get_locale_information(output_for: OutputFor) -> Vec<LocaleInformation>;
        unsafe fn get_default_locale()->*const Locale;

        unsafe fn get_locale_name(locale: *const Locale) -> String;
        unsafe fn get_locale_display_name(locale: *const Locale) -> String;
    }
}

pub use ffi_icu::{get_locale_information, LocaleInformation, Locale};

pub fn get_default_locale() -> *const Locale {
    unsafe { ffi_icu::get_default_locale() }
}

pub fn get_locale_name(locale: *const Locale) -> String {
    unsafe { ffi_icu::get_locale_name(locale) }
}

fn get_locale_display_name(locale: *const Locale) -> String {
    unsafe { ffi_icu::get_locale_display_name(locale) }
}
impl Default for LocaleInformation {
    fn default() -> Self {
        let locale = get_default_locale();
        let locale_name = get_locale_name(locale);
        let locale_display_name = get_locale_display_name(locale);
        LocaleInformation {
            locale,
            locale_name,
            locale_display_name,
        }
    }
}