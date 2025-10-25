#[cxx::bridge]
mod qobject {

    #[repr(i32)]
    enum OutputKind {
        DisplayName = 0,
        FullName = 1,
        BaseName = 2,
    }
    unsafe extern "C++" {
        include!("chameleon-format-dialog/cpp/includes/enums.h");
        type OutputKind;
    }

    #[repr(i32)]
    enum OutputFor {
        AllLocales = 0,
        NumberFormatLocales = 1,
        DateFormatLocales = 2,
    }

    extern "C++" {
        include!("chameleon-format-dialog/cpp/includes/enums.h");
        type OutputFor;

    }

    unsafe extern "C++" {
        include!("chameleon-format-dialog/cpp/format.h");

        type LocalizedNumberFormatter;

        fn format_f64(self: &LocalizedNumberFormatter, value: f64) ->Result<String>;
        fn format_i64(self: &LocalizedNumberFormatter, value: i64) -> Result<String>;

        fn new_localized_number_formatter(locale_name: &str) -> UniquePtr<LocalizedNumberFormatter>;

        unsafe fn format_f64(locale_name: &str) -> String;
        

    }

}

use cxx::UniquePtr;
pub use qobject::*;

impl LocalizedNumberFormatter {
    pub fn new(locale_name: &str) -> UniquePtr<LocalizedNumberFormatter> {
        new_localized_number_formatter(locale_name)
    }
}
