
#pragma once

#include <cstdint>

enum class OutputKind : std::int32_t{
    DisplayName = 0,
    FullName = 1,
    BaseName = 2,
};

enum class OutputFor : std::int32_t {
    AllLocales = 0,
    NumberFormatLocales = 1,
    DateFormatLocales = 2,
};