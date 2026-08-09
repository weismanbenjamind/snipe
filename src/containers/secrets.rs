use serde::{Deserialize, Serialize};
use toml::Value as TomlValue;
use toml::map::Map as TomlMap;

const REDACTED_DELIMITER: &str = "*****";

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct SecretString(String);

impl SecretString {
    pub(crate) fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for SecretString {
    fn from(value: String) -> Self {
        SecretString(value)
    }
}

impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", REDACTED_DELIMITER)
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", REDACTED_DELIMITER)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct SecretTomlValue(TomlValue);

impl SecretTomlValue {
    fn fmt_toml_val(val: &TomlValue, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match val {
            TomlValue::Boolean(_)
            | TomlValue::Float(_)
            | TomlValue::Integer(_)
            | TomlValue::Datetime(_) => {
                write!(f, "{val}")
            }
            TomlValue::String(_) => write!(f, "{}", REDACTED_DELIMITER),
            TomlValue::Array(a) => Self::fmt_array(a, f),
            TomlValue::Table(t) => Self::fmt_table(t, f),
        }
    }

    fn fmt_array(a: &[TomlValue], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut arr_iter = a.iter();
        if let Some(val) = arr_iter.next() {
            Self::fmt_toml_val(val, f)?;
        }
        arr_iter.try_for_each(|val| {
            write!(f, ", ")?;
            Self::fmt_toml_val(val, f)
        })?;
        write!(f, "]")
    }

    fn fmt_table(
        t: &TomlMap<String, TomlValue>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut table_iter = t.iter();
        if let Some((k, v)) = table_iter.next() {
            write!(f, "{k}: ")?;
            Self::fmt_toml_val(v, f)?;
        }
        table_iter.try_for_each(|(k, v)| {
            write!(f, ", {k}: ")?;
            Self::fmt_toml_val(v, f)
        })?;
        write!(f, "}}")
    }

    fn debug_toml_val(val: &TomlValue, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match val {
            TomlValue::Boolean(_)
            | TomlValue::Float(_)
            | TomlValue::Integer(_)
            | TomlValue::Datetime(_) => {
                write!(f, "{val:?}")
            }
            TomlValue::String(_) => write!(f, "String({})", REDACTED_DELIMITER),
            TomlValue::Array(a) => Self::debug_array(a, f),
            TomlValue::Table(t) => Self::debug_table(t, f),
        }
    }

    fn debug_array(a: &[TomlValue], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Array([")?;
        let mut arr_iter = a.iter();
        if let Some(val) = arr_iter.next() {
            Self::debug_toml_val(val, f)?;
        }
        arr_iter.try_for_each(|val| {
            write!(f, ", ")?;
            Self::debug_toml_val(val, f)
        })?;
        write!(f, "])")
    }

    fn debug_table(
        t: &TomlMap<String, TomlValue>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Table({{")?;
        let mut table_iter = t.iter();
        if let Some((k, v)) = table_iter.next() {
            write!(f, "{k:?}: ")?;
            Self::debug_toml_val(v, f)?;
        }
        table_iter.try_for_each(|(k, v)| {
            write!(f, ", {k:?}: ")?;
            Self::debug_toml_val(v, f)
        })?;
        write!(f, "}})")
    }
}

impl std::fmt::Display for SecretTomlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::fmt_toml_val(&self.0, f)
    }
}

impl std::fmt::Debug for SecretTomlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::debug_toml_val(&self.0, f)
    }
}

impl From<SecretTomlValue> for TomlValue {
    fn from(value: SecretTomlValue) -> Self {
        value.0
    }
}

impl From<TomlValue> for SecretTomlValue {
    fn from(value: TomlValue) -> Self {
        Self(value)
    }
}
