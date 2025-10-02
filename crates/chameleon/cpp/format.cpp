#include "format.h"
#include "chameleon/src/locale.rs.h"

rust::String format_f64(const Locale *locale, const double &d) {
    auto l = number::NumberFormatter::with().notation(number::Notation::compactShort());
    return rust::String("");
}

