use serde::Serialize;
use std::collections::HashMap;
use toml::Table;

#[derive(Default, Serialize)]
pub struct FieldData {
    pub name: String,
    pub datatype: String,
    pub collection: Vec<String>,
    pub generics: Vec<String>,
    pub dims: Vec<i64>,
    pub unit: String,
    pub desc: String,
    pub tag: String,

    pub is_primitive: bool,
    pub is_struct: bool,
    pub is_scalar: bool,
    pub is_collection: bool,
    pub is_generic: bool,
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

    pub gen_types: Vec<String>,
    pub is_generic: bool,
}

#[derive(Serialize)]
pub struct TypeInfo {
    pub interface: String,
    pub structs: HashMap<String, BlockData>,
    pub is_template: bool,
}

/// Parses the interface TOML file and returns the
/// resulting data.
///
pub fn parse_interface<F>(data: &Table, verbose: bool, cb_inc: &mut F) -> Result<TypeInfo, String>
    where F: FnMut(&FieldData)
{
    let mut block_desc = "".to_string();
    let mut block_type : &str = "";
    if data.contains_key("model") {
        block_type = "model";
    } else if data.contains_key("template") {
        block_type = "template";
    }
    let block_data = match data[block_type].as_table() {
        Some(v) => v,
        None => return Err(format!("[{}] is not a table", block_type)),
    };
    if !block_data.contains_key("name") {
        return Err(format!("{} table does not contain [name]", block_type));
    }
    let block_name = match block_data["name"].as_str() {
        Some(v) => v.to_string(),
        None => return Err(format!("[{}.name] is not a string", block_type)),
    };
    if block_data.contains_key("desc") {
        block_desc = match block_data["desc"].as_str() {
            Some(v) => v.to_string(),
            None => return Err(format!("[{}.desc] is not a string", block_type)),
        }
    }

    // construct the type database
    let mut db = TypeInfo {
        interface: block_name,
        structs: HashMap::new(),
        is_template: data.contains_key("template"),
    };
    // parse all described types
    if !data.contains_key("types") {
        return Err("[types] does not exist".to_string());
    }
    let type_info = match data["types"].as_table() {
        Some(v) => v,
        None => return Err("[types] is not a table".to_string()),
    };
    match parse_types(type_info, &mut db, cb_inc, verbose) {
        Ok(()) => {},
        Err(e) => return Err(e),
    }

    // TODO validate no struct definitions are missing
    Ok(db)
}

/// parse all structs from the [types] table.
/// does not validate that all referenced types exist.
fn parse_types<F>(data: &Table, db: &mut TypeInfo, cb_inc: &mut F, verbose: bool) -> Result<(), String>
    where F: FnMut(&FieldData)
{
    for (k, v) in data {
        let vtable = match v.as_table() {
            Some(val) => val,
            None => return Err(format!("{} is not a table", k)),
        };
        match parse_type(k, vtable, cb_inc, verbose) {
            Ok(bval) => {
                db.structs.insert(k.to_string(), bval);
            },
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn parse_type<F>(name: &String, data: &Table, cb_inc: &mut F, verbose: bool) -> Result<BlockData, String>
    where F: FnMut(&FieldData)
{
    let mut block = BlockData::default();
    block.name = name.to_string();
    if data.contains_key("desc") {
        match data["desc"].as_str() {
            Some(v) => {
                block.desc = v.to_string();
            },
            None => return Err(format!("[{}.desc] is not a string", name)),
        }
    }
    // template data
    if data.contains_key("generic") {
        let args = match data["generic"].as_table() {
            Some(v) => v,
            None => return Err(format!("[{}.generic] is not a table", name)),
        };
        for (k,v) in args.iter() {
            let tdata = match v.as_table() {
                Some(v) => v,
                None => return Err(format!("[{}.generic.{}] is not a table", name, k)),
            };
            if !tdata.contains_key("options") {
                return Err(format!("[{}.generic.{}] is missing `options` field", name, k));
            }
            match tdata["options"].as_array() {
                Some(v) => {},
                None => return Err(format!("[{}.generic.{}.options] is not an array", name, k)),
            }
            block.gen_types.push(k.to_string());
        }
        block.is_generic = true;
    }
    // field parsing
    if data.contains_key("fields") {
        let fields = match data["fields"].as_array() {
            Some(v) => v,
            None => return Err(format!("[{}.fields] is not an array", name)),
        };
        for (i, field) in fields.iter().enumerate() {
            let ftable = match field.as_table() {
                Some(v) => v,
                None => return Err(format!("[{}.fields][{}] is not a table", name, i)),
            };

            match parse_field(ftable) {
                Ok(v) => {
                    cb_inc(&v);
                    block.fields.push(v);
                },
                Err(e) => return Err(e),
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
        None => return Err("[name] in field is not a string".to_string()),
    };
    if !data.contains_key("type") {
        return Err("[type] not found in field".to_string());
    }
    fdata.datatype = match data["type"].as_str() {
        Some(v) => v.to_string(),
        None => return Err("[type] in field is not a string".to_string()),
    };
    // optional fields
    fdata.is_scalar = true;
    if data.contains_key("dims") {
        let idims = match data["dims"].as_array() {
            Some(v) => v,
            None => return Err("[dims] in field is not an array".to_string()),
        };
        for d in idims {
            fdata.dims.push(match d.as_integer() {
                Some(v) => v,
                None => return Err("[dims] array has non-integer values".to_string()),
            });
            if fdata.dims.len() > 0 {
                fdata.is_scalar = false;
            }
        }
    }
    if data.contains_key("desc") {
        fdata.desc = match data["desc"].as_str() {
            Some(v) => v.to_string(),
            None => return Err("[desc] in field is not a string".to_string()),
        }
    }
    if data.contains_key("generic") {
        let gdata = match data["generic"].as_array() {
            Some(v) => v,
            None => return Err(format!("[{}][generic] is not an array", fdata.name)),
        };
        for val in gdata {
            match val.as_str() {
                Some(v) => fdata.generics.push(v.to_string()),
                None => return Err(format!("[{}][generic] has non string elements", fdata.name)),
            }
        }
        fdata.is_generic = true;
    }
    Ok(fdata)
}
