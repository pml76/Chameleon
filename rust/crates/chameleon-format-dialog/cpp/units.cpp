#include "chameleon-format-dialog/cpp/includes/icu_includes.h"
#include "chameleon-format-dialog/cpp/units.h"
#include <cstdint>
#include <iostream>
#include <ostream>
#include <vector>


rust::Vec<rust::String> get_unit_types() {
    rust::Vec<rust::String> unit_types;

    UErrorCode errorCode = U_ZERO_ERROR;
    std::string result;
    StringEnumeration* types = MeasureUnit::getAvailableTypes(errorCode);

    if (U_SUCCESS(errorCode))
    {
        errorCode = U_ZERO_ERROR;
        for (int32_t i = 0; i < types->count(errorCode); i++) {
            errorCode = U_ZERO_ERROR;
            result = "";
            std::string tmp = types->snext(errorCode)->toUTF8String(result);
            unit_types.push_back(tmp);
        }
    } else
    {
        std::cerr << "Error-code returned from MeasureUnit::getAvailableTypes(): " << errorCode << std::endl;
    }

    delete types;

    return unit_types;
}

rust::Vec<rust::String> get_available_units_for_type( const rust::String &unit_type ) {
    rust::Vec<rust::String> units;
    std::vector<icu_77::MeasureUnit> measure_units;
    int32_t size = 500;
    UErrorCode errorCode = U_ZERO_ERROR;
    int32_t count;
    rust::String type = unit_type;


    do {
        size *= 2;
        measure_units.resize(size);
        errorCode = U_ZERO_ERROR;
        count = MeasureUnit::getAvailable(type.c_str(), measure_units.data(), size, errorCode);
    } while (errorCode == U_BUFFER_OVERFLOW_ERROR );

    if (U_SUCCESS(errorCode))
    {
        for (int32_t i = 0; i < count; i++)
        {
            const MeasureUnit& unit = measure_units[i];
            std::string tmp = unit.getIdentifier();
            units.push_back(tmp);
        }
    }

    return units;
}