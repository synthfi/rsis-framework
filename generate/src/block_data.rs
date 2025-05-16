use serde::Serialize;
use std::collections::HashMap;
use toml::Table;

#[derive(Default, Serialize)]
pub struct FieldData {
    name: String,
    datatype: String,
    dims: Vec<i64>,
    unit: String,
    desc: String,
    tag: String,

    is_primitive: bool,
    is_struct: bool,
    is_scalar: bool,
}

impl FieldData {
    pub fn len(&self) -> usize {
        self.dims.len()
    }
}

#[derive(Default, Serialize)]
pub struct BlockData {
    pub name: String,
    pub desc: String,
    pub fields: Vec<FieldData>,
}

#[derive(Serialize)]
pub struct TypeInfo {
    pub interface: String,
    pub structs: HashMap<String, BlockData>,
}

/// Parses the interface TOML file and returns the
/// resulting data.
///
pub fn parse_interface(data: &Table, verbose: bool) -> Result<TypeInfo, String> {
    if !data.contains_key("model") {
        return Err("[model] does not exist".to_string());
    }
    let model_data = match data["model"].as_table() {
        Some(v) => v,
        None => {
            return Err("[model] is not a table".to_string());
        }
    };
    if !model_data.contains_key("name") {
        return Err("Model table does not contain [name]".to_string());
    }
    let model_name = match model_data["name"].as_str() {
        Some(v) => v,
        None => {
            return Err("[model.name] is not a string".to_string());
        }
    };
    let model_desc = if model_data.contains_key("desc") {
        match model_data["desc"].as_str() {
            Some(v) => v,
            None => {
                return Err("[model.desc] is not a string".to_string());
            }
        }
    } else {
        ""
    };

    // construct the type database
    let mut db = TypeInfo {
        interface: model_name.to_string(),
        structs: HashMap::new(),
    };
    // parse all described types
    if !data.contains_key("types") {
        return Err("[types] does not exist".to_string());
    }
    let type_info = match data["types"].as_table() {
        Some(v) => v,
        None => {
            return Err("[types] is not a table".to_string());
        }
    };
    match parse_types(type_info, &mut db, verbose) {
        Ok(()) => {},
        Err(e) => {
            return Err(e);
        }
    }

    // TODO validate no struct definitions are missing
    Ok(db)
}

/// parse all structs from the [types] table.
/// does not validate that all referenced types exist.
fn parse_types(data: &Table, db: &mut TypeInfo, verbose: bool) -> Result<(), String> {
    for (k, v) in data {
        let vtable = match v.as_table() {
            Some(val) => val,
            None => {
                return Err(format!("{} is not a table", k));
            }
        };
        match parse_type(k, vtable, verbose) {
            Ok(bval) => {
                db.structs.insert(k.to_string(), bval);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}

fn parse_type(name: &String, data: &Table, verbose: bool) -> Result<BlockData, String> {
    let mut block = BlockData::default();
    block.name = name.to_string();
    if data.contains_key("desc") {
        match data["desc"].as_str() {
            Some(v) => {
                block.desc = v.to_string();
            },
            None => {
                return Err(format!("[{}.desc] is not a string", name));
            }
        }
    }
    if data.contains_key("fields") {
        let fields = match data["fields"].as_array() {
            Some(v) => v,
            None => {
                return Err(format!("[{}.fields] is not an array", name));
            }
        };
        for (i, field) in fields.iter().enumerate() {
            let ftable = match field.as_table() {
                Some(v) => v,
                None => {
                    return Err(format!("[{}.fields][{}] is not a table", name, i));
                }
            };

            match parse_field(ftable) {
                Ok(v) => {
                    block.fields.push(v);
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
    if verbose {
        println!("struct: {}", name);
    }
    Ok(block)
}

fn parse_field(data: &Table) -> Result<FieldData, String> {
    let mut fdata = FieldData::default();
    // required fields
    if !data.contains_key("name") {
        return Err("[name] not found in field".to_string());
    }
    fdata.name = match data["name"].as_str() {
        Some(v) => v.to_string(),
        None => {
            return Err("[name] in field is not a string".to_string());
        }
    };
    if !data.contains_key("type") {
        return Err("[type] not found in field".to_string());
    }
    fdata.datatype = match data["type"].as_str() {
        Some(v) => v.to_string(),
        None => {
            return Err("[type] in field is not a string".to_string());
        }
    };
    // optional fields
    fdata.is_scalar = true;
    if data.contains_key("dims") {
        let idims = match data["dims"].as_array() {
            Some(v) => v,
            None => {
                return Err("[dims] in field is not an array".to_string());
            }
        };
        for d in idims {
            fdata.dims.push(match d.as_integer() {
                Some(v) => v,
                None => {
                    return Err("[dims] array has non-integer values".to_string());
                }
            });
            if fdata.dims.len() > 0 {
                fdata.is_scalar = false;
            }
        }
    }
    if data.contains_key("desc") {
        fdata.desc = match data["desc"].as_str() {
            Some(v) => v.to_string(),
            None => {
                return Err("[desc] in field is not a string".to_string());
            }
        }
    }
    Ok(fdata)
}
