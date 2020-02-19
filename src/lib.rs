extern crate byteorder;
extern crate chrono;

use std::io::{Read, Seek, SeekFrom, Error, ErrorKind};
use std::collections::HashMap;
use std::fmt;

use byteorder::{LittleEndian, BigEndian, ReadBytesExt};
use chrono::{Utc, TimeZone, DateTime};

pub use crc::crc_16;
mod crc;

pub mod fit_types;
pub use fit_types::*;

/*
I really should implement better errors

Errors,
 - io::Error
 - fmt::Error
 - CRC Error
 - Magic Token Error
 - Invalid Size Error
*/

const RECORD_COMPRESSED_TIME: u8 = 0b_1000_0000;
const RECORD_DEFINITION: u8      = 0b_0100_0000;
const RECORD_DEVELOPER_DATA: u8  = 0b_0010_0000;

#[derive(Debug, Clone)]
pub enum FieldType {
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

impl FieldType {

    fn from_code(code: u8) -> Option<FieldType> {
        match code {
            0x00 => Some(FieldType::Enum),
            0x01 => Some(FieldType::Sint8),
            0x02 => Some(FieldType::Uint8),
            0x03 => Some(FieldType::Sint16),
            0x04 => Some(FieldType::Uint16),
            0x05 => Some(FieldType::Sint32),
            0x06 => Some(FieldType::Uint32),
            0x07 => Some(FieldType::String),
            0x08 => Some(FieldType::Float32),
            0x09 => Some(FieldType::Float64),
            0x0A => Some(FieldType::Uint8z),
            0x0B => Some(FieldType::Uint16z),
            0x0C => Some(FieldType::Uint32z),
            0x0D => Some(FieldType::Bytes),
            0x0E => Some(FieldType::Sint64),
            0x0F => Some(FieldType::Uint64),
            0x10 => Some(FieldType::Uint64z),
            _ => None
        } 
    }

    fn size(&self) -> u8 {
        match *self {
            FieldType::Enum => 1,
            FieldType::Sint8 | FieldType::Uint8 | FieldType::Uint8z => 1,
            FieldType::Sint16 | FieldType::Uint16 | FieldType::Uint16z => 2,
            FieldType::Sint32 | FieldType::Uint32 | FieldType::Uint32z => 4,
            FieldType::Sint64 | FieldType::Uint64 | FieldType::Uint64z => 8,
            FieldType::Float32 => 4,
            FieldType::Float64 => 8,
            FieldType::Bytes => 1,
            FieldType::String => 1,
        } 
    }
}

#[derive(Debug, Clone)]
struct Header {
    header_size: u8,
    protocol_version: u8,
    profile_version: u16,
    size: u32
}

impl Header {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Header, Error> {
        let header_size = reader.read_u8()?;
        if header_size >= 12 {
            let protocol_version = reader.read_u8()?;
            let profile_version = reader.read_u16::<LittleEndian>()?;
            let data_size = reader.read_u32::<LittleEndian>()?;
            let data_type = reader.read_u32::<LittleEndian>()?;
            if data_type != 0x5449462E {
                return Err(Error::new(ErrorKind::Other, "Invalid magic"));
            } 
            let mut crc = 0u16;
            if header_size >= 14 {
                crc = reader.read_u16::<LittleEndian>()?;
            }
            if crc > 0 {
                reader.seek(SeekFrom::Start(0))?;
                let mut buffer = [0; 12];
                reader.read_exact(&mut buffer)?;
                let mut calculated_crc = 0u16;
                for byte in buffer.iter() {
                    calculated_crc = crc_16(calculated_crc, byte);
                }
                if crc != calculated_crc {
                    return Err(Error::new(ErrorKind::Other, "Invalid CRC"));
                }
            }
            reader.seek(SeekFrom::Start(header_size as u64))?;
            return Ok(Header{
                header_size: header_size,
                protocol_version: protocol_version,
                profile_version: profile_version,
                size: data_size});
        }
        return Err(Error::new(ErrorKind::Other, "Invalid size"));
    }
}

#[derive(Debug, Clone)]
pub struct DefinitionField {
    pub index: u8,
    pub size: u8,
    pub count: u8,
    pub kind: FieldType,
}

impl DefinitionField {
    fn read_field<R: Read + Seek>(&self, reader: &mut R, big_endian: bool)
        -> Result<FitType, Error>
    {
        match self.kind {
            FieldType::Enum => {
                let value = reader.read_u8()?;
                Ok(FitType::Enum(value))
            }
            FieldType::Sint8 => {
                let value = reader.read_i8()?;
                Ok(FitType::Sint8(value))
            }
            FieldType::Uint8 => {
                let value = reader.read_u8()?;
                Ok(FitType::Uint8(value))
            }
            FieldType::Sint16 => {
                let value = if big_endian {
                        reader.read_i16::<BigEndian>()?
                    } else {
                        reader.read_i16::<LittleEndian>()?
                    };
                Ok(FitType::Sint16(value))
            }
            FieldType::Uint16 => {
                let value = if big_endian {
                        reader.read_u16::<BigEndian>()?
                    } else {
                        reader.read_u16::<LittleEndian>()?
                    };
                Ok(FitType::Uint16(value))
            }
            FieldType::Sint32 => {
                let value = if big_endian {
                        reader.read_i32::<BigEndian>()?
                    } else {
                        reader.read_i32::<LittleEndian>()?
                    };
                Ok(FitType::Sint32(value))
            }
            FieldType::Uint32 => {
                let value = if big_endian {
                        reader.read_u32::<BigEndian>()?
                    } else {
                        reader.read_u32::<LittleEndian>()?
                    };
                Ok(FitType::Uint32(value))
            }
            FieldType::String => {
                let mut value: Vec<u8> = vec![0; self.size as usize];
                reader.read_exact(&mut value)?;
                let value = match String::from_utf8(value) {
                    Ok(s) => s,
                    Err(_) => String::new(),
                };
                Ok(FitType::String(value))
            }
            FieldType::Float32 => {
                let value = if big_endian {
                        reader.read_f32::<BigEndian>()?
                    } else {
                        reader.read_f32::<LittleEndian>()?
                    };
                Ok(FitType::Float32(value))
            }
            FieldType::Float64 => {
                let value = if big_endian {
                        reader.read_f64::<BigEndian>()?
                    } else {
                        reader.read_f64::<LittleEndian>()?
                    };
                Ok(FitType::Float64(value))
            }
            FieldType::Uint8z => {
                let value = reader.read_u8()?;
                Ok(FitType::Uint8z(value))
            }
            FieldType::Uint16z => {
                let value = if big_endian {
                        reader.read_u16::<BigEndian>()?
                    } else {
                        reader.read_u16::<LittleEndian>()?
                    };
                Ok(FitType::Uint16z(value))
            }
            FieldType::Uint32z => {
                let value = if big_endian {
                        reader.read_u32::<BigEndian>()?
                    } else {
                        reader.read_u32::<LittleEndian>()?
                    };
                Ok(FitType::Uint32z(value))
            }
            FieldType::Bytes => {
                let mut value: Vec<u8> = Vec::with_capacity(self.size as usize);
                reader.read_exact(&mut value)?;
                Ok(FitType::Bytes(value))
            }
            FieldType::Sint64 => {
                let value = if big_endian {
                        reader.read_i64::<BigEndian>()?
                    } else {
                        reader.read_i64::<LittleEndian>()?
                    };
                Ok(FitType::Sint64(value))
            }
            FieldType::Uint64 => {
                let value = if big_endian {
                        reader.read_u64::<BigEndian>()?
                    } else {
                        reader.read_u64::<LittleEndian>()?
                    };
                Ok(FitType::Uint64(value))
            }
            FieldType::Uint64z => {
                let value = if big_endian {
                        reader.read_u64::<BigEndian>()?
                    } else {
                        reader.read_u64::<LittleEndian>()?
                    };
                Ok(FitType::Uint64z(value))
            }
        }
    }
}

#[derive(Debug)]
pub struct CompressedTimeRecord {
    pub local_index: u8,
    pub global_index: u16,
    pub time_offset: u8,
    pub fields: Vec<(u8, u8, FitType)>,
}

impl CompressedTimeRecord {
    fn read<R: Read + Seek>(header: u8, reader: &mut R, definitions: &HashMap<u8, DefinitionRecord>)
        -> Result<CompressedTimeRecord, Error> {
        let local_index = (header & 0x60) >> 5;
        let time_offset = header & 0x1F;
        let definition = definitions.get(&local_index);
        let definition = match definition {
            Some(def) => {
                def
            },
            None => {
                return Err(Error::new(ErrorKind::Other, "Unknown type"));
            }
        };

        let mut fields = Vec::with_capacity(definition.fields.len());

        let global_index = definition.global_index;

        for ref def in definition.fields.iter() {
            for n in 0..def.count {
                let field = def.read_field(reader, definition.big_endian)?;
                fields.push((def.index, n, field));
            }
        }

        return Ok(CompressedTimeRecord{
            local_index: local_index,
            global_index: global_index,
            time_offset: time_offset,
            fields: fields
            });
    }

    pub fn message_type(&self) -> Option<MesgNumField>
    {
        MesgNumField::into_field(self.global_index)
    }
}

impl fmt::Display for CompressedTimeRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg_name = match fit_types::get_message_name(self.global_index) {
            Some(ref s) => s,
            None => "",
        };
        write!(f, "CompressedTime:\n  local: {}\n  global: {}\n  offset: {}\n  fields: {}\n  name: {}\n",
            self.local_index, self.global_index, self.time_offset, self.fields.len(), msg_name)?;
        for field in self.fields.iter() {
            let field_name = match fit_types::get_type_name(self.global_index, field.0) {
                Some(ref s) => s,
                None => "",
            };
            write!(f, "    {0} ({1:02x}) {2} {3}\n", field_name, field.0, field.1, field.2)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DefinitionRecord {
    pub local_index: u8,
    pub global_index: u16,
    pub big_endian: bool,
    pub fields: Vec<DefinitionField>,
}

impl DefinitionRecord {
    fn read<R: Read + Seek>(header: u8, reader: &mut R) -> Result<DefinitionRecord, Error> {
        let local_index = header & 0x0f;
        let developer = (header & RECORD_DEVELOPER_DATA)
            == RECORD_DEVELOPER_DATA;
        reader.read_u8()?;
        let architecture = reader.read_u8()?;
        let big_endian = (architecture & 0x01) == 0x01;
        let global_index = if big_endian {
                reader.read_u16::<BigEndian>()?
            }
            else {
                reader.read_u16::<LittleEndian>()?
            };
        let num_fields = reader.read_u8()?;
        let mut fields = Vec::with_capacity(num_fields as usize); 
        for _ in 0..num_fields {
            let f_index = reader.read_u8()?;
            let f_size = reader.read_u8()?;
            let f_kind = reader.read_u8()?;
            let f_kind = f_kind & 0x1f;
            let f_fbt = match FieldType::from_code(f_kind) {
                Some(s) => s,
                None => { return Err(Error::new(ErrorKind::Other, "Unknown type")); }
            };
            let fbt_size = f_fbt.size();
            let type_ok = match f_fbt {
                FieldType::Bytes | FieldType::String => {
                    true
                }
                _ => {
                    f_size % fbt_size == 0
                }
            };
            if !type_ok {
                return Err(Error::new(ErrorKind::Other, "Type size mismatch"));
            }
            let count = match f_fbt {
                FieldType::Bytes | FieldType::String => {
                    1
                }
                _ => {
                    f_size / fbt_size
                }
            };
            fields.push(
                DefinitionField {index: f_index, size: f_size, count: count, kind: f_fbt});
        }
        if developer {
            let dev_num_fields = reader.read_u8()?;
            println!("!! Skipping developer fields!");
            reader.seek(SeekFrom::Current((dev_num_fields as i64) * 3))?;
        }
        return Ok(DefinitionRecord{local_index: local_index,
            global_index: global_index, big_endian: big_endian,
            fields: fields});
    }
}

impl fmt::Display for DefinitionRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg_name = fit_types::get_message_name(self.global_index);
        if msg_name.is_none() {
            write!(f, "\x1B[31m")?;
        }
        let msg_name = match msg_name { Some(m) => { m }, None => { "" } };
        write!(f, "Definition: local: {:3} global: {:5} fields: {:3} name: {}\n"
            , self.local_index, self.global_index, self.fields.len(), msg_name)?;
        for field in self.fields.iter() {
            let field_name = match fit_types::get_type_name(self.global_index, field.index) {
                Some(ref s) => s, None => "",
            };
            if field_name.is_empty() {
                write!(f, "\x1B[31m")?;
            }
            write!(f, "  {:3} {:?} {}", field.index, field.kind, field_name)?;
            if field_name.is_empty() {
                writeln!(f, "\x1B[0m")?;
            }
            else {
                writeln!(f, "")?;
            }
        }
        write!(f, "\x1B[0m\n")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct NormalRecord {
    pub local_index: u8,
    pub global_index: u16,
    pub fields: Vec<(u8, u8, FitType)>,
}

impl NormalRecord {
    fn read<R: Read + Seek>(header: u8, reader: &mut R, definitions: &HashMap<u8, DefinitionRecord>)
        -> Result<NormalRecord, Error> {
        let local_index = header & 0x0F;
        let definition = definitions.get(&local_index);
        let definition = match definition {
            Some(def) => {
                def
            },
            None => {
                return Err(Error::new(ErrorKind::Other, "Unknown type"));
            }
        };

        let mut fields = Vec::with_capacity(definition.fields.len()); 

        for ref def in definition.fields.iter() {
            for n in 0..def.count {
                let field = def.read_field(reader, definition.big_endian)?;
                fields.push((def.index, n, field));
            }
        }

        return Ok(NormalRecord{
            local_index: local_index,
            global_index: definition.global_index,
            fields: fields,
            });
    }

    pub fn message_type(&self) -> Option<MesgNumField>
    {
        MesgNumField::into_field(self.global_index)
    }
}

impl fmt::Display for NormalRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg_name = match fit_types::get_message_name(self.global_index) {
            Some(ref s) => s.to_string(),
            None => format!("{:5}", self.global_index),
        };
        write!(f, "Normal: {}\n", msg_name)?;
        for field in self.fields.iter() {
            let name = match fit_types::get_type_name(self.global_index, field.0) {
                Some(ref s) => format!("{} ({})", s, field.0),
                None => format!("{:3}", field.0),
            };
            write!(f, "  {:3} {:32} {}\n", field.1, name, field.2)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Record {
    CompressedTime(CompressedTimeRecord),
    Definition(DefinitionRecord),
    Normal(NormalRecord)
}

impl Record {
    fn read<R: Read + Seek>(reader: &mut R, definitions: &HashMap<u8, DefinitionRecord>) -> Result<Record, Error> {
        let record_flag = reader.read_u8()?;

        if record_flag & 0b_0001_0000 == 0b_0001_0000 {
            return Err(Error::new(ErrorKind::Other,
                format!("Invalid record header, {:02x}", record_flag)));
        }
        
        if (record_flag & RECORD_COMPRESSED_TIME) == RECORD_COMPRESSED_TIME {
            let record = CompressedTimeRecord::read(record_flag, reader, definitions)?;
            return Ok(Record::CompressedTime(record));
        }
        else {
            if (record_flag & RECORD_DEFINITION) == RECORD_DEFINITION {
                let record = DefinitionRecord::read(record_flag, reader)?;
                return Ok(Record::Definition(record));
            }
            else {
                let record = NormalRecord::read(record_flag, reader, definitions)?;
                return Ok(Record::Normal(record));
            }
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Record::Normal(ref rec) => {
                write!(f, "{}", rec)
            },
            Record::CompressedTime(ref rec) => {
                write!(f, "{}", rec)
            },
            Record::Definition(ref rec) => {
                write!(f, "{}", rec)
            }
        }
    }
}

pub fn read_fit(reader: &mut std::fs::File) -> Result<Vec<Record>, Error> {
    let header = Header::decode(reader)?;
    let mut records = Vec::new();
    let mut definitions = HashMap::new();
    let file_stop =  header.size as u64 + header.header_size as u64;
    loop {
        let position = reader.seek(SeekFrom::Current(0i64))?;
        if position >= file_stop {
            break;
        }
        let record = Record::read(reader, &definitions)?;
        match record {
            Record::Definition(ref rec) => {
                definitions.insert(rec.local_index, rec.clone());
                }
            Record::Normal(_) => {}
            Record::CompressedTime(_) => {}
        }
        records.push(record);
    }
    Ok(records)
}

pub fn timestamp_as_datetime(timestamp: u32) -> DateTime<Utc>
{
    if timestamp < 0x10000000 {
        // seconds from device power on
        return Utc.timestamp(i64::from(timestamp), 0);
    }
    else {
        return Utc.timestamp(i64::from(timestamp) + FIT_EPOCH, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_files() {
        use std::fs;
        for entry in fs::read_dir("fit-files").unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                let file = &mut fs::File::open(path).unwrap();
                let _ = read_fit(file).unwrap();
            }
        }
    }

    #[test]
    fn display_records() {
        use std::fs;

        for entry in fs::read_dir("fit-files").unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                let file = &mut fs::File::open(path).unwrap();
                let records = read_fit(file).unwrap();
                for record in records {
                    println!("{}", record);
                }
            }
        }
    }

    #[test]
    fn datetime_timestamp() {
        assert_eq!(
            timestamp_as_datetime(0x0fffffff),
            Utc.ymd(1978, 07, 04).and_hms(21, 24, 15)
            );
        assert_eq!(
            timestamp_as_datetime(0x10000000),
            Utc.ymd(1998, 07, 03).and_hms(21, 24, 16)
            );
        assert_eq!(
            timestamp_as_datetime(u32::min_value()),
            Utc.ymd(1970, 01, 01).and_hms(00, 00, 00)
            );
        assert_eq!(
            timestamp_as_datetime(u32::max_value()),
            Utc.ymd(2126, 02, 06).and_hms(06, 28, 15)
            );
    }
}
