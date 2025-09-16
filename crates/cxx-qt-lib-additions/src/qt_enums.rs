#[cxx_qt::bridge]
mod qobject {
    #[namespace = "Qt"]
    enum Orientation {
        Horizontal,
        Vertical,
    }

    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!(<Qt>);
        type Orientation;
    }
    
}

use qobject::*;