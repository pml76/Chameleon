#include "format.h"
#include <unicode/numberformatter.h>
#include "chameleon-format-dialog/src/locale.cxx.h"
#include "chameleon-format-dialog/src/model.cxxqt.h"
#include "chameleon-format-dialog/src/number_sign_display_selector_model.cxxqt.h"


number::LocalizedNumberFormatter new_number_formater_with_locale(const LocaleInformation &locale)
{
    return number::NumberFormatter::withLocale(*locale.locale);
}

number::ScientificNotation scientific_notation(const bool is_engineering, const ENumberSignDisplay number_sign_display, const int32_t min_exponent_digits) {

    const auto engineering = is_engineering ? number::Notation::engineering() : number::Notation::scientific();
    switch (number_sign_display)
    {
        case ENumberSignDisplay::Always: return engineering.withExponentSignDisplay(UNUM_SIGN_ALWAYS).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::Auto: return engineering.withExponentSignDisplay(UNUM_SIGN_AUTO).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::Never: return engineering.withExponentSignDisplay(UNUM_SIGN_NEVER).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::ExceptZero: return engineering.withExponentSignDisplay(UNUM_SIGN_EXCEPT_ZERO).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::Accounting: return engineering.withExponentSignDisplay(UNUM_SIGN_ACCOUNTING).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::AccountingNegative: return engineering.withExponentSignDisplay(UNUM_SIGN_EXCEPT_ZERO).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::AccountingAlways: return engineering.withExponentSignDisplay(UNUM_SIGN_ACCOUNTING_ALWAYS).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::SignNegative: return engineering.withExponentSignDisplay(UNUM_SIGN_NEGATIVE).withMinExponentDigits(min_exponent_digits);

        case ENumberSignDisplay::AccountingExceptZero : return engineering.withExponentSignDisplay(UNUM_SIGN_ACCOUNTING_EXCEPT_ZERO).withMinExponentDigits(min_exponent_digits);



        default: return engineering.withExponentSignDisplay(UNUM_SIGN_AUTO).withMinExponentDigits(min_exponent_digits);
    }

}

number::LocalizedNumberFormatter notation (const number::LocalizedNumberFormatter& f, const ENotion notation, const ENumberSignDisplay exponent_sign_display, const int32_t min_exponent_digits) {

    switch (notation)
    {
        case ENotion::CompactLong:  return f.notation(number::Notation::compactLong());

        case ENotion::CompactShort: return f.notation(number::Notation::compactShort());

        case ENotion::Engineering:
            {
                const auto engineering = scientific_notation(true, exponent_sign_display, min_exponent_digits);
                return f.notation(engineering);
            }

        case ENotion::Scientific:
            {
                const auto scientific = scientific_notation(false, exponent_sign_display, min_exponent_digits);
                return f.notation(scientific);
            }

        case ENotion::Simple:       return f.notation(number::Notation::simple());



        default: return f.notation(number::Notation::simple());

    }

}


rust::String format_f64(const LocaleInformation &locale, const double &d) {
    auto l = number::NumberFormatter::with();
    l.notation(number::Notation::compactShort());

    return {""};
}

