#pragma once
#include "rust/cxx.h"
#include "chameleon-format-dialog/cpp/includes/icu_includes.h"

rust::String format_f64(const rust::Str locale_name);


/// internal interface class to hold an instance of number::LocalizedNumberFormatter
class LocalizedNumberFormatterData {
    friend class LocalizedNumberFormatter;
public:
    explicit LocalizedNumberFormatterData(const std::string& locale_name) :
        formatter(number::NumberFormatter::withLocale(Locale::createCanonical(locale_name.c_str()))) {}

private:
    number::LocalizedNumberFormatter formatter;

};

/// interface class for rust implementing the builder pattern for a LocalizedNumberFormatter.
class LocalizedNumberFormatter {
public:
    explicit LocalizedNumberFormatter(const rust::Str& locale_name) :
        data(std::make_unique<LocalizedNumberFormatterData>(std::string(locale_name))) {}

    /// format a f64 into a string
    [[nodiscard]] rust::String format_f64(double value) const
    {
        UErrorCode errorCode = U_ZERO_ERROR;
        auto formatted_value = data->formatter.formatDouble(value, errorCode);
        if (U_FAILURE(errorCode))
        {
            throw std::runtime_error("Error formatting number in LocalizedNumberFormatter::format_f64");
        }

        errorCode = U_ZERO_ERROR;
        auto result = formatted_value.toString(errorCode);
        if (U_FAILURE(errorCode))
        {
            throw std::runtime_error("Error converting to UnicodeString in LocalizedNumberFormatter::format_f64");
        }

        std::string tmp;
        return result.toUTF8String(tmp);
    }

    /// format an i64 into a string
    [[nodiscard]] rust::String format_i64(int64_t value) const {
        UErrorCode errorCode = U_ZERO_ERROR;
        auto formatted_value = data->formatter.formatInt(value, errorCode);
        if (U_FAILURE(errorCode))
        {
            throw std::runtime_error("Error formatting number in LocalizedNumberFormatter::format_f64");
        }

        errorCode = U_ZERO_ERROR;
        auto result = formatted_value.toString(errorCode);
        if (U_FAILURE(errorCode))
        {
            throw std::runtime_error("Error converting to UnicodeString in LocalizedNumberFormatter::format_f64");
        }

        std::string tmp;
        return result.toUTF8String(tmp);
    }

private:
    std::unique_ptr<LocalizedNumberFormatterData> data;
};

/// create a new LocalizedNumberFormatter instance
[[nodiscard]]
std::unique_ptr<LocalizedNumberFormatter> new_localized_number_formatter(rust::Str locale_name);