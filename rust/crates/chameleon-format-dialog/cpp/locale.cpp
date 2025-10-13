#include "chameleon-format-dialog/cpp/includes/icu_includes.h"
#include "chameleon-format-dialog/cpp/locale.h"
#include "chameleon-format-dialog/cpp/includes/enums.h"
#include "chameleon-format-dialog/src/locale.cxx.h"

rust::Vec<LocaleInformation> get_locale_information(OutputFor output_for) {
    rust::Vec<LocaleInformation> locale_info;

    int32_t count;
    const Locale* list = NULL;

    switch (output_for) {
        case OutputFor::AllLocales: list = icu::Locale::getAvailableLocales(count); break;
        case OutputFor::NumberFormatLocales: list = icu::NumberFormat::getAvailableLocales(count); break;
        case OutputFor::DateFormatLocales: list = icu::DateFormat::getAvailableLocales(count); break;
        default: return locale_info;
    }

    LocaleInformation locale_info_item;
    for (int i = 0; i < count; i++) {
        locale_info_item.locale = &list[i];
        locale_info_item.locale_name = list[i].getName();

        UnicodeString result = ""; std::string tmp;
        list[i].getDisplayName(Locale::getRoot(), result);
        locale_info_item.locale_display_name = result.toUTF8String(tmp);

        locale_info.push_back(locale_info_item);
    }

    return locale_info;
}


const Locale *get_default_locale() {
    return &Locale::getRoot();
}

rust::String get_locale_name(const Locale *locale)
{
    return locale->getName();
}

rust::String get_locale_display_name(const Locale *locale)
{
    UnicodeString result = "";
    std::string tmp;
    locale->getDisplayName(Locale::getRoot(), result);
    return result.toUTF8String(tmp);
}