use simd_json::{base::ValueAsObject, derived::ValueObjectAccess};
use std::{fs, path::Path};

pub fn parse_package_json_scripts<P: AsRef<Path>>(path: &P) -> Result<Vec<String>, std::io::Error> {
    let mut raw = fs::read(path)?;
    let mut package_json_scripts = Vec::new();

    if let Ok(object) = simd_json::to_borrowed_value(&mut raw) {
        if let Some(scripts) = object.get("scripts").as_object() {
            for (key, _) in scripts.iter() {
                package_json_scripts.push(key.to_string());
            }
        }
    }

    Ok(package_json_scripts)
}
