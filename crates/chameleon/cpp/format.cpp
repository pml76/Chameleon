#include <fmt/format.h>
#include "format.h"

rust::String format_f64(const rust::String &fmt_str, const double &d) {
    return fmt::format(std::string(fmt_str), d);
}

rust::String format_f32(const rust::String &fmt_str, const float &f) {
    return fmt::format(std::string(fmt_str), f);
}

rust::string format_bool(const rust::string &fmt_str, const bool &b) {
    return fmt::format(std::string(fmt_str), b);
}

rust::String format_i8(const rust::String &fmt_str, const int8_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_i16(const rust::String &fmt_str, const int16_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_i32(const rust::String &fmt_str, const int32_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_i64(const rust::String &fmt_str, const int64_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_u8(const rust::String &fmt_str, const uint8_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_u16(const rust::String &fmt_str, const uint16_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_u32(const rust::String &fmt_str, const uint32_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_u64(const rust::String &fmt_str, const uint64_t &i) {
    return fmt::format(std::string(fmt_str), i);
}

rust::String format_str(const rust::String &fmt_str, const rust::String &s) {
    return fmt::format(std::string(fmt_str), std::string(s));
}