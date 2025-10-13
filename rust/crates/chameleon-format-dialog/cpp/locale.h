#pragma once
#include <unicode/locid.h>

#include "rust/cxx.h"
#include "chameleon-format-dialog/cpp/includes/enums.h"

struct LocaleInformation;

class icu_77::Locale;
using icu_77::Locale;

rust::Vec<LocaleInformation> get_locale_information(OutputFor output_for);
const Locale *get_default_locale();
rust::String get_locale_name(const Locale *locale);
rust::String get_locale_display_name(const Locale *locale);

