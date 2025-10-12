#include "chameleon-format-dialog/cpp/includes/icu_includes.h"
#include "chameleon-format-dialog/cpp/locale.h"
#include "chameleon-format-dialog/cpp/includes/enums.h"
#include "chameleon-format-dialog/src/locale.cxx.h"

rust::Vec<LocaleInformation> get_locale_information(OutputFor output_for) {
    rust::Vec<LocaleInformation> locale_info;

    int32_t count;
    const Locale* list = NULL;
    UnicodeString result;

    switch (output_for) {
        case OutputFor::AllLocales: list = icu::Locale::getAvailableLocales(count); break;
        case OutputFor::NumberFormatLocales: list = icu::NumberFormat::getAvailableLocales(count); break;
        case OutputFor::DateFormatLocales: list = icu::DateFormat::getAvailableLocales(count); break;
        default: return locale_info;
    }

    std::string tmp;
    LocaleInformation locale_info_item;
    for (int i = 0; i < count; i++) {
        locale_info_item.locale = &list[i];
        locale_info_item.locale_name = list[i].getName();

        result = ""; tmp = "";
        list[i].getDisplayName(icu::Locale::getUS(), result);
        locale_info_item.locale_display_name = result.toUTF8String(tmp);

        locale_info.push_back(locale_info_item);
    }

    return locale_info;
}