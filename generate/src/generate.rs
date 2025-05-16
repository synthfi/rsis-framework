
use crate::block_data::*;
use minijinja::{context, Environment};
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;

pub fn generate_interface(outfile: &Path, db: &TypeInfo, target: String) -> Result<(), String> {
    match target.as_ref() {
        "rust" => {
            println!("generating rust");
            rs_interface(outfile, db);
        },
        "cpp" => {
            return Err("cpp unimplemented".to_string());
        },
        "fortran" => {
            return Err("fortran unimplemented".to_string());
        }
        _ => {
            return Err("Unknown language".to_string());
        },
    }
    Ok(())
}

fn rs_interface(file: &Path, db: &TypeInfo) -> Result<(), String> {
    let file_resolved = file.with_extension("rs");

    // check for SVector, SMatrix
    let mut includes: Vec<String> = vec![];
    let mut svec = false;
    let mut smat = false;
    for (_key, val) in &db.structs {
        for field in &val.fields {
            if !svec && field.len() == 1 {
                svec = true;
                includes.push("use nalgebra::SVector;".to_string());
            }
            else if !smat && field.len() == 2 {
                smat = true;
                includes.push("use nalgebra::SMatrix;".to_string())
            }
        }
    }

    // minijinja rendering
    let mut env = Environment::new();
    env.add_template("block.rs", include_str!("../templates/block.rs"))
        .unwrap();
    let tmpl = env.get_template("block.rs").unwrap();
    let rendered = tmpl.render(
        context!(db => db, includes => includes)
    ).unwrap();
    let mut f = match File::create(&file_resolved) {
        Ok(v) => v,
        Err(..) => {
            return Err("Failed to create output file".to_string());
        }
    };
    f.write_all(rendered.as_bytes());
    println!("Generated: {}", file_resolved.display());
    Ok(())
}

fn cpp_interface(file: &Path, db: &TypeInfo) -> Result<(), String> {
    let typemap = HashMap::from([
        ("usize", "std::size_t"),
        ("isize", "std::ptrdiff_t"),
        ("i8", "int8_t"),
        ("i16", "int16_t"),
        ("i32", "int32_t"),
        ("i64", "int64_t"),
        ("u8", "uint8_t"),
        ("u16", "uint16_t"),
        ("u32", "uint32_t"),
        ("u64", "uint64_t"),
        ("f32", "float"),
        ("f64", "double"),
        ("bool", "bool"),
        ("char", "char"),
        ("String", "std::string"),
    ]);
    let containermap = HashMap::from([
        ("HashMap", "std::unordered_map"),
        ("BTreeMap", "std::map"),
        ("HashSet", "std::unordered_set"),
        ("BTreeSet", "std::set"),
    ]);
    Ok(())
}
