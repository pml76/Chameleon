#[cxx_qt::bridge]
mod qobject {
    #[namespace = "Qt"]
    #[repr(i32)]
    #[derive(Debug, Eq)]
    enum Orientation {
        Horizontal = 1,
        Vertical = 2,
    }

    /// Each item in the model has a set of data elements associated with it, each with its own role.
    /// The roles are used by the view to indicate to the model which type of data it needs.
    /// Custom models should return data in these types.
    #[namespace = "Qt"]
    #[repr(i32)]
    #[derive(Debug, Eq)]
    enum ItemDataRole {
        /// The general purpose roles (and the associated types) are:
        DisplayRole = 0,
        DecorationRole = 1,
        EditRole = 2,
        ToolTipRole = 3,
        StatusTipRole = 4,
        WhatsThisRole = 5,
        SizeHintRole = 13,

        /// Roles describing appearance and meta data (with associated types):
        FontRole = 6,
        TextAlignmentRole = 7,
        BackgroundRole = 8,
        ForegroundRole = 9,
        CheckStateRole = 10,
        InitialSortOrderRole = 14,

        /// Accessibility roles (with associated types):
        AccessibleTextRole = 11,
        AccessibleDescriptionRole = 12,

        /// User roles:
        /// 
        /// For user roles, it is up to the developer to decide which types to use and ensure 
        /// that components use the correct types when accessing and setting data.
        UserRole = 0x100,
    }

    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!(<Qt>);
        type Orientation;

        type ItemDataRole;
    }
    
}

pub use qobject::*;