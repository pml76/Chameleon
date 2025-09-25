#pragma once
#include "rust/cxx.h"

rust::Vec<rust::String> get_available_number_locales();

rust::String format_f64(const rust::String &fmt_str, const double &d);
