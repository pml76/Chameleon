#include "format.h"
#include <unicode/numberformatter.h>
#include "chameleon-format-dialog/src/locale.cxx.h"

rust::String format_f64(const Locale *locale, const double &d) {
    auto l = icu_77::number::NumberFormatter::with();
    l.notation(icu_77::number::Notation::compactShort());

    return {""};
}

