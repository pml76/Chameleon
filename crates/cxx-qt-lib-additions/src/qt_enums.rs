#[cxx_qt::bridge]
mod qobject {
    #[namespace = "Qt"]
    #[repr(i32)]
    enum Orientation {
        Horizontal = 1,
        Vertical = 2,
    }

    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!(<Qt>);
        type Orientation;
    }
    
}

pub use qobject::*;