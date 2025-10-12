#pragma once
#include "rust/cxx.h"

rust::Vec<rust::String> get_unit_types();
rust::Vec<rust::String> get_available_units_for_type( rust::String unit_type );