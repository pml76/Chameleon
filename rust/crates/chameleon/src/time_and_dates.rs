
use chrono_tz::*;
use cxx::UniquePtr;
use cxx_qt_lib::{QTimeZone, QString};



pub fn tz_to_qtimezone(tz: Tz) -> UniquePtr<QTimeZone> {
    // Get the time zone name (e.g., "America/New_York")
    let timezone_name = QString::from(tz.name()).to_utf8();

    // Create a QTimeZone using the name
    QTimeZone::from_iana( &timezone_name)

}
