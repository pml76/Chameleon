#pragma once
#include <format>
#include "rust/cxx.h"

rust::String format_f64(const rust::String &fmt_str, const double &d);
rust::String format_f32(const rust::String &fmt_str, const float &f);

rust::String format_bool(const rust::String &fmt_str, const bool &b);

rust::String format_i8(const rust::String &fmt_str, const int8_t &i);
rust::String format_i16(const rust::String &fmt_str, const int16_t &i);
rust::String format_i32(const rust::String &fmt_str, const int32_t &i);
rust::String format_i64(const rust::String &fmt_str, const int64_t &i);

rust::String format_u8(const rust::String &fmt_str, const uint8_t &i);
rust::String format_u16(const rust::String &fmt_str, const uint16_t &i);
rust::String format_u32(const rust::String &fmt_str, const uint32_t &i);
rust::String format_u64(const rust::String &fmt_str, const uint64_t &i);

rust::String format_str(const rust::String &fmt_str, const rust::String &str);