#include "format.h"
#include <unicode/locid.h>
#include <unicode/unistr.h>
#include <unicode/uloc.h>

rust::Vec<rust::String> get_available_number_locales() {

    rust::Vec<rust::String> res;

    int32_t count;
    const icu::Locale* list = NULL;
    icu::UnicodeString result;
    list = icu::Locale::getAvailableLocales(count);

    std::string tmp;
    for (int i = 0; i < count; i++) {
        list[i].getDisplayName(icu::Locale::getUS(), result);

        tmp = "";
        res.push_back(result.toUTF8String(tmp));
    }

    return res;
}

rust::String format_f64(const rust::String &fmt_str, const double &d) {
    return rust::String("");
}

