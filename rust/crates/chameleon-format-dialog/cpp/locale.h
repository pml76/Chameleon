#pragma once
#include "rust/cxx.h"
#include "chameleon-format-dialog/cpp/includes/enums.h"

struct LocaleInformation;

rust::Vec<LocaleInformation> get_locale_information(OutputFor output_for);

