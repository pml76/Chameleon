#pragma once
#include "rust/cxx.h"
#include "chameleon/cpp/icu_includes.h"

rust::String format_f64(const Locale *locale, const double &d);
