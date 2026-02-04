use crate::err::RulesError;
use std::collections::HashMap;

pub fn write(
    file_name: &str,
    obj_type: String,
    obj: HashMap<String, Vec<String>>,
) -> Result<(), RulesError> {
    write_with_base_dir(file_name, obj_type, obj, "config")
}

#[cfg(test)]
pub(crate) fn write_with_base_dir(
    _file_name: &str,
    _obj_type: String,
    _obj: HashMap<String, Vec<String>>,
    _base_dir: &str,
) -> Result<(), RulesError> {
    // TODO: if object already exists, add non-existent attributes
    Ok(())
}

#[cfg(not(test))]
pub(crate) fn write_with_base_dir(
    _file_name: &str,
    _obj_type: String,
    _obj: HashMap<String, Vec<String>>,
    _base_dir: &str,
) -> Result<(), RulesError> {
    // TODO: if object already exists, add non-existent attributes
    Ok(())
}
