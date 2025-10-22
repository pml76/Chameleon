pub mod locale;
pub mod format;
pub mod locale_selector_model;
pub mod notion_selector_model;
mod number_sign_display_selector_model;
mod model;
mod unit_selector_model;
mod units;



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
