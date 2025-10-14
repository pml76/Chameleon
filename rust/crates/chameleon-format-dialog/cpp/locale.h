#pragma once
#include <unicode/locid.h>

#include "rust/cxx.h"
#include "chameleon-format-dialog/cpp/includes/enums.h"

struct LocaleInformation;


rust::Vec<LocaleInformation> get_locale_information(OutputFor output_for);
rust::String get_default_locale_name();
rust::String get_default_locale_display_name();

