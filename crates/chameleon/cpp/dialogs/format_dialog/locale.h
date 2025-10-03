#pragma once
#include "rust/cxx.h"
#include "chameleon/cpp/includes/enums.h"

struct LocaleInformation;

rust::Vec<LocaleInformation> get_locale_information(OutputFor output_for);

