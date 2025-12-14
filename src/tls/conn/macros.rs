/// Macro to set boolean flags on the SSL connector
macro_rules! set_bool {
    ($cfg:expr, $field:ident, $conn:expr, $setter:ident) => {
        if $cfg.$field {
            $conn.$setter();
        }
    };
    ($cfg:expr, !$field:ident, $conn:expr, $setter:ident, $arg:expr) => {
        if !$cfg.$field {
            $conn.$setter($arg);
        }
    };
}

/// Macro to set optional values on the SSL connector
macro_rules! set_option {
    ($cfg:expr, $field:ident, $conn:expr, $setter:ident) => {
        if let Some(val) = $cfg.$field {
            $conn.$setter(val);
        }
    };
}

/// Macro to set optional reference values on the SSL connector (with error handling)
macro_rules! set_option_ref_try {
    ($cfg:expr, $field:ident, $conn:expr, $setter:ident) => {
        if let Some(val) = $cfg.$field.as_deref() {
            $conn.$setter(val)?;
        }
    };
}

pub(super) use {set_bool, set_option, set_option_ref_try};
