
extern crate xlsx_read;

use xlsx_read::{WorkBook, WorkSheet};
use xlsx_read::Value as ExcelValue;

use std::process;
use std::collections::{HashMap, BTreeMap};
use std::io::{Result, Write};
use std::fs;

#[derive(Debug)]
enum BaseType {
    Enum = 0x00,
    Sint8,
    Uint8,
    Sint16,
    Uint16,
    Sint32,
    Uint32,
    String,
    Float32,
    Float64,
    Uint8z,
    Uint16z,
    Uint32z,
    Bytes,
    Sint64,
    Uint64,
    Uint64z,
}

impl BaseType {
    fn from_name(name: &str) -> Option<BaseType> {
        match name {
            "enum" => Some(BaseType::Enum),
            "sint8" => Some(BaseType::Sint8),
            "uint8" => Some(BaseType::Uint8),
            "sint16" => Some(BaseType::Sint16),
            "uint16" => Some(BaseType::Uint16),
            "sint32" => Some(BaseType::Sint32),
            "uint32" => Some(BaseType::Uint32),
            "string" => Some(BaseType::String),
            "float32" => Some(BaseType::Float32),
            "float64" => Some(BaseType::Float64),
            "uint8z" => Some(BaseType::Uint8z),
            "uint16z" => Some(BaseType::Uint16z),
            "uint32z" => Some(BaseType::Uint32z),
            "byte" => Some(BaseType::Bytes),
            "sint64" => Some(BaseType::Sint64),
            "uint64" => Some(BaseType::Uint64),
            "uint64z" => Some(BaseType::Uint64z),
            "bool" => Some(BaseType::Uint8), /* Isn't really a base type */
            _ => None
        } 
    }

    fn to_rust_type(&self) -> &str
    {
        match *self {
            BaseType::Enum => "u8",
            BaseType::Sint8 => "i8",
            BaseType::Uint8 => "u8",
            BaseType::Sint16 => "i16",
            BaseType::Uint16 => "u16",
            BaseType::Sint32 => "i32",
            BaseType::Uint32 => "u32",
            BaseType::String => "String",
            BaseType::Float32 => "f32",
            BaseType::Float64 => "f64",
            BaseType::Uint8z => "u8",
            BaseType::Uint16z => "u16",
            BaseType::Uint32z => "u32",
            BaseType::Bytes => "Vec<u8>",
            BaseType::Sint64 => "i64",
            BaseType::Uint64 => "u64",
            BaseType::Uint64z => "u64",
        }
    }
}

fn snake_to_camel(word: &str) -> String {
    let mut result = String::with_capacity(word.len());
    let copy = word.to_string().to_lowercase();
    let parts: Vec<&str> = copy.split("_").collect();
    for part in parts {
        if !part.is_empty() {
            let mut chars = part.chars();
            result.push_str(&chars.next().unwrap().to_uppercase().collect::<String>().clone());
            for c in chars {
                result.push_str(&c.to_lowercase().collect::<String>().clone());
            }
        }
    }
    /* Some labels have a number as the first character */
    if let Some(c) = result.chars().next() {
        if c.is_numeric() {
            result.insert(0, '_');
        }
    }
    return result;
}

#[derive(Debug, Clone)]
struct FitType {
    pub name: String,
    pub value_type: String,
    pub values: HashMap<String, i64>,
}

impl FitType {
    fn generate_enum<W: Write>(&self, mut writer: W) -> Result<()>  {
        let mut type_name = snake_to_camel(&self.name);
        let mut values = BTreeMap::new();
        for (label, id) in &self.values {
            values.insert(id, snake_to_camel(label));
        }

        type_name.push_str("Field");

        writeln!(&mut writer, "")?;
        writeln!(&mut writer, "/* name: {} type: {} */", self.name, self.value_type)?;
        writeln!(&mut writer, "#[allow(dead_code)]\n#[derive(Debug)]")?;
        writeln!(&mut writer, "pub enum {} {{", type_name)?;
        for (value, label) in values.iter() {
            writeln!(&mut writer, "  {} = {},", label, value)?;
        }
        writeln!(&mut writer, "}}")?;

        match self.rust_type() {
            Some(ref rust_type) => {
                write!(&mut writer, "\nimpl From<{0}> for {1} {{\n  fn from(source: {0}) -> {1} {{\n    match source {{\n",
                    type_name, rust_type)?;
                for (value, label) in values.iter() {
                    write!(&mut writer, "      {}::{} => {},\n",
                        type_name, label, value)?;
                }
                write!(&mut writer, "    }}\n  }}\n}}\n")?;


                write!(&mut writer, "\nimpl IntoField<{1}> for {0} {{\n  fn into_field(value: {1}) -> Option<{0}> {{\n    match value {{\n",
                    type_name, rust_type)?;
                for (value, label) in values.iter() {
                    write!(&mut writer, "      {} => Some({}::{}),\n",
                        value, type_name, label)?;
                }
                write!(&mut writer, "      _ => None,\n")?;
                write!(&mut writer, "    }}\n  }}\n}}\n")?;
            },
            None => {}
        }
        Ok(())
     }

    fn rust_type(&self) -> Option<String> {
        let base_type = BaseType::from_name(&self.value_type);
        match base_type {
            Some(ref t) => { Some(t.to_rust_type().to_string()) },
            None => { None }
        }
    }
}

#[allow(dead_code)]
struct FitField {
    pub id: u8,
    pub name: String,
    pub data_type: String,
    pub fit_type: Option<FitType>,
}

struct FitMessage {
    pub id: u8,
    pub name: String,
    pub fields: Vec<FitField>,
}

impl FitMessage {
    fn generate<W: Write>(&self, mut writer: W) -> Result<()> {
        if !self.name.is_empty() {
            writeln!(&mut writer, "\n#[allow(dead_code)]\n#[derive(Debug)]")?;
            writeln!(&mut writer, "pub struct {}Message {{", snake_to_camel(&self.name))?;
            writeln!(&mut writer, "  /* id == {} */", self.id)?;
            for field in self.fields.iter() {
                let name = if field.name == "type" { "_type".to_string() } else { field.name.clone() };
                writeln!(&mut writer, "  {0}: {1}, /* {2:02x} {3:?} */", name, field.data_type, field.id, field.fit_type)?;
            }
            write!(&mut writer, "}}\n")?;
        }
        Ok(())
    }
}

fn load_fit_types(worksheet: WorkSheet) -> HashMap<String, FitType> {
    let mut fit_types: HashMap<String, FitType> = HashMap::new();
    let mut row = 0;
    let mut type_name = String::new();
    let mut value_type = String::new();
    let mut value_name = String::new();
    let mut values = HashMap::new();
    for cell in worksheet.cells {
        if cell.row <= 1 {
            continue;
        }
        if cell.row != row {
            value_name = String::new();
        }
        row = cell.row;
        if cell.column == 1 {
            if let ExcelValue::String(ref s) = cell.value {
                if !type_name.is_empty() {
                    fit_types.insert(type_name.clone(),
                                     FitType {
                                         name: type_name.clone(),
                                         value_type: value_type.clone(),
                                         values: values.clone(),
                                     });
                }
                type_name = s.clone();
                values.clear();
            }
            else {
                panic!("Expected string in cell");
            }
        }
        if cell.column == 2 {
            if let ExcelValue::String(ref s) = cell.value {
                let mut lower_type = s.to_lowercase().clone();
                if lower_type == "unit8" {
                    lower_type = "uint8".to_string();
                }
                value_type = lower_type.clone();
            }
            else {
                panic!("Expected string in cell");
            }
        }
        if cell.column == 3 {
            if let ExcelValue::String(ref s) = cell.value {
                value_name = s.clone();
            }
            else {
                panic!("Expected string in cell");
            }
        }
        if cell.column == 4 {
            let number = match cell.value {
                ExcelValue::String(ref v) => {
                    if v.starts_with("0x") {
                        Some(i64::from_str_radix(&v[2..], 16).unwrap())
                    } else {
                        Some(i64::from_str_radix(v, 10).unwrap())
                    }
                }
                ExcelValue::Integer(v) => Some(v),
                ExcelValue::Float(_) => {
                    panic!("Unexpected float value in cell");
                }
                ExcelValue::Empty => {
                    panic!("Unexpected empty cell");
                }
            };
            if let Some(num) = number {
                values.insert(value_name.clone(), num);
            }
            
        }
    }
    if !values.is_empty() {
        fit_types.insert(type_name.clone(),
                         FitType {
                             name: type_name.clone(),
                             value_type: value_type.clone(),
                             values: values.clone(),
                         });
    }
    return fit_types;
}

fn load_fit_messages(worksheet: WorkSheet,
    fit_types: &HashMap<String, FitType>) -> Vec<FitMessage>
{
    let mut fit_messages: Vec<FitMessage> = Vec::new();
    let message_numbers = fit_types["mesg_num"].values.clone();
    let mut row = 0;
    let mut message_name = String::new();
    let mut message_number = 0i64;
    let mut field_id = 0xffu8;
    let mut field_name = String::new();
    let mut field_type = String::new();
    let mut fit_type: Option<FitType> = None;
    let mut fields = Vec::new();
    for cell in worksheet.cells {
        if cell.row <= 1 {
            continue;
        }
        if cell.row > row {
            if field_id < 0xff {
                fields.push(FitField {
                    id: field_id,
                    name: field_name.clone(),
                    data_type: field_type.clone(),
                    fit_type: fit_type,
                });
            }
            field_id = 0xffu8;
            fit_type = None;
        }
        row = cell.row;
        if cell.column == 1 {
            if !message_name.is_empty() {
                fit_messages.push(FitMessage {
                    id: message_number as u8,
                    name: message_name.clone(),
                    fields: fields,
                });
            }
            fields = Vec::new();
            if let ExcelValue::String(ref s) = cell.value {
                message_name = s.clone();
                message_number = message_numbers[s];
            }
        }
        if !message_name.is_empty() {
            if cell.column == 2 {
                if let ExcelValue::Integer(ref v) = cell.value {
                    field_id = *v as u8;
                }
            } else if cell.column == 3 {
                if let ExcelValue::String(ref s) = cell.value {
                    field_name = s.clone();
                }
            } else if cell.column == 4 && field_id < 0xff {
                if let ExcelValue::String(ref s) = cell.value {
                    let lower_type = s.to_lowercase();
                    match fit_types.get(&lower_type) {
                        Some(ref r) => {
                            match r.rust_type() {
                                Some(t) => { field_type = t.clone(); }
                                None => {}
                            }
                            fit_type = Some((**r).clone());
                        }
                        None => {
                            match BaseType::from_name(&lower_type) {
                                Some(t) => { field_type = t.to_rust_type().to_string().clone(); }
                                None => {}
                            }
                        }
                    }
                }
            }
        }
    }
    return fit_messages;
}

fn write_types<W: Write>(fit_types: &HashMap<String, FitType>, mut writer: W)
    -> Result<()>
{
    for ref fit_type in fit_types.values() {
        fit_type.generate_enum(&mut writer)?;
    }

    let mut message_numbers = BTreeMap::new();
    for (label, id) in &fit_types["mesg_num"].values {
        message_numbers.insert(id, label);
    }

    write!(&mut writer, "pub fn get_message_name(id: u16) -> Option<&'static str> {{\n")?;
    write!(&mut writer, "  match id {{\n")?;
    for (id, label) in message_numbers {
        write!(&mut writer, "    {} => Some(\"{}\"),\n", id, label)?;
    }
    write!(&mut writer, "    _ => None,\n")?;
    write!(&mut writer, "  }}\n")?;
    write!(&mut writer, "}}\n\n")?;

    Ok(())
}

#[allow(dead_code)]
fn write_messages<W: Write>(fit_messages: &Vec<FitMessage>, mut writer: W)
    -> Result<()>
{
    for ref message in fit_messages {
        message.generate(&mut writer)?;
    }

    writeln!(&mut writer, "pub fn get_type_name(message_id: u16, field_id: u8) -> Option<&'static str> {{")?;
    writeln!(&mut writer, "  match message_id {{")?;
    for ref message in fit_messages {
        if message.name.is_empty() {
            continue;
        }
        writeln!(&mut writer, "    {} => {{", message.id)?;
        writeln!(&mut writer, "      match field_id {{")?;
        for ref field in &message.fields {
            writeln!(&mut writer, "        {} => Some(\"{}\"),", field.id, field.name)?;
        }
        writeln!(&mut writer, "        _ => None,")?;
        writeln!(&mut writer, "      }}")?;
        writeln!(&mut writer, "    }},")?;
    }
    writeln!(&mut writer, "    _ => None,")?;
    writeln!(&mut writer, "  }}")?;
    writeln!(&mut writer, "}}\n")?;

    Ok(())
}

fn main() {
    use std::path::Path;
    let profile_path = Path::new("fit-sdk/Profile.xlsx");
    let output_path = Path::new("src/fit_types.rs");
    if !output_path.is_file() {
        if !profile_path.is_file() {
            println!("Failed to find profile, {}", profile_path.display());
        }
        let mut workbook = WorkBook::open(profile_path).unwrap();
        workbook.load().unwrap();
        
        let mut types_file = fs::File::create(output_path).unwrap();
        write!(types_file, "/* This file was auto-generated */\n", ).unwrap();
        types_file.write(include_bytes!("fit_types.in")).unwrap();
        {
            let worksheet = workbook.load_worksheet("Types").unwrap();
            let fit_types = load_fit_types(worksheet);
            write_types(&fit_types, &mut types_file).unwrap();
            let worksheet = workbook.load_worksheet("Messages").unwrap();
            let fit_messages = load_fit_messages(worksheet, &fit_types);
            write_messages(&fit_messages, &mut types_file).unwrap()
        }
    }
    process::exit(0);
}
