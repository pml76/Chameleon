use crate::notion_selector_model::NotionSelectorModelRust;
use crate::notion_selector_model::qobject::ENotion;
use crate::number_sign_display_selector_model::NumberSignDisplaySelectorModelRust;
use crate::number_sign_display_selector_model::qobject::ENumberSignDisplay;
use crate::locale_selector_model::LocaleSelectorModelRust;
use cxx_qt::CxxQtType;
use std::pin::Pin;
use crate::locale::LocaleInformation;
use crate::unit_selector_model::UnitTypeSelectorModelRust;

#[cxx_qt::bridge]
mod qobject {

    unsafe extern "C++" {
        include!("chameleon-format-dialog/src/notion_selector_model.cxxqt.h");
        type ENotion = crate::notion_selector_model::ENotion;

        include!("chameleon-format-dialog/src/number_sign_display_selector_model.cxxqt.h");
        type ENumberSignDisplay = crate::number_sign_display_selector_model::ENumberSignDisplay;


        include!("chameleon-format-dialog/src/locale.cxxqt.h");
        type LocaleInformation = crate::locale_selector_model::LocaleInformation;
    }

    pub struct FormatDialogSettings {
        pub notion: ENotion,
        pub number_sign_display: ENumberSignDisplay,
        pub locale: LocaleInformation,
        pub unit_type: String,
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type FormatDialogModel = super::FormatDialogModelRust;

        #[rust_name = "set_notion_index"]
        #[qinvokable]
        fn setNotionIndex(self: Pin<&mut Self>, index: i32);

        #[rust_name = "get_notion_index"]
        #[qinvokable]
        fn getNotionIndex(self: Pin<&mut Self>) -> i32;

        #[rust_name = "get_number_sign_display_index"]
        #[qinvokable]
        fn getNumberSignDisplayIndex(self: Pin<&mut Self>) -> i32;

        #[rust_name = "set_number_sign_display_index"]
        #[qinvokable]
        fn setNumberSignDisplayIndex(self: Pin<&mut Self>, index: i32);

        #[rust_name = "get_locale_index"]
        #[qinvokable]
        fn getLocaleIndex(self: Pin<&mut Self>) -> i32;

        #[rust_name = "set_locale_index"]
        #[qinvokable]
        fn setLocaleIndex(self: Pin<&mut Self>, index: i32);
    }
}

pub struct FormatDialogModelRust {
    notion: ENotion,
    notion_selector_model_rust: NotionSelectorModelRust,

    number_sign_display: ENumberSignDisplay,
    number_sign_display_selector_model_rust: NumberSignDisplaySelectorModelRust,

    locale: LocaleInformation,
    locale_selector_model_rust: LocaleSelectorModelRust,

    unit_type: String,
    unit_type_selector_model_rust: UnitTypeSelectorModelRust,
}

impl Default for FormatDialogModelRust {
    fn default() -> Self {
        let mut ret = Self {
            notion: ENotion::Simple,
            notion_selector_model_rust: NotionSelectorModelRust::default(),

            number_sign_display: ENumberSignDisplay::Auto,
            number_sign_display_selector_model_rust: NumberSignDisplaySelectorModelRust::default(),

            locale: LocaleInformation::default(),
            locale_selector_model_rust: LocaleSelectorModelRust::default(),

            unit_type: "".to_string(),
            unit_type_selector_model_rust: UnitTypeSelectorModelRust::default(),
        };

        ret.locale = ret.locale_selector_model_rust.find_locale_information(ret.locale_selector_model_rust.default_locale_index().unwrap()).unwrap().clone();
        ret.unit_type = ret.unit_type_selector_model_rust.default_unit_type();
        ret
    }
}

impl qobject::FormatDialogModel {
    fn get_locale_index(self: Pin<&mut Self>) -> i32 {
        println!("get locale index");
        if let Some(s) = self.rust().locale_selector_model_rust.find_locale_index(&self.rust().locale.locale_name) {
            return s;
        }
        0
    }

    fn set_locale_index(mut self: Pin<&mut Self>, index: i32) {
        if let Some(s) = self.rust().locale_selector_model_rust.find_locale_information(index) {
            println!("set locale to: {:?}", s.locale_display_name);
            self.as_mut().rust_mut().locale = s.clone();
        }
    }

    fn set_notion_index(mut self: Pin<&mut Self>, index: i32) {
        let notions = &self.as_mut().rust_mut();
        let notions = notions.notion_selector_model_rust.get_notions();

        if index > notions.len() as i32 || index < 0 {
            return;
        }
        self.as_mut().rust_mut().notion = notions[index as usize];

        println!("set notion to: {:?}", self.as_mut().rust_mut().notion);
    }

    fn get_notion_index(self: Pin<&mut Self>) -> i32 {
        println!("get notion index");
        let notions = self.rust();
        let notions = notions.notion_selector_model_rust.get_notions();

        for (index, notion) in notions.iter().enumerate() {
            if *notion == self.rust().notion {
                return index as i32;
            }
        }
        0
    }

    fn get_number_sign_display_index(self: Pin<&mut Self>) -> i32 {
        println!("get number sign display index");
        let number_sign_display_selector_model = self.rust();
        let number_sign_display_selector_model = number_sign_display_selector_model
            .number_sign_display_selector_model_rust
            .get_number_sign_displays();

        for (index, number_sign_display) in number_sign_display_selector_model.iter().enumerate() {
            if *number_sign_display == self.rust().number_sign_display {
                return index as i32;
            }
        }
        0
    }

    fn set_number_sign_display_index(mut self: Pin<&mut Self>, index: i32) {
        let number_sign_display_selector_model = self.as_mut().rust_mut();
        let number_sign_display_selector_model = number_sign_display_selector_model
            .number_sign_display_selector_model_rust
            .get_number_sign_displays();

        if index
            > number_sign_display_selector_model
            .len() as i32
            || index < 0
        {
            return;
        }
        self.as_mut().rust_mut().number_sign_display =
            number_sign_display_selector_model[index as usize];
        println!(
            "set number sign display to: {:?}",
            self.as_mut().rust_mut().number_sign_display
        );
    }
}
