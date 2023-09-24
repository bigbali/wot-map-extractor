use core::fmt;
use std::{
    collections::HashMap,
    env,
    fmt::{Display, Formatter},
    fs::File,
    io::{Cursor, Read},
    io::{Error, ErrorKind},
    mem::{self, size_of},
    ops::{Deref, DerefMut},
};

mod bwsg;

use bwsg::bwsg;

use crate::bwsg::BWSG;

// each map sector (A0, A1, ..., B9, ...) maps to .cdata_processed
// entry: [?, offset, ?, length, ?]

const HEADER_ENTRIES: usize = 29;

pub type SectionTableEntryValue = [i32; 5];

pub type SectionTableEntryId = [u8; 4];

#[derive(Default, Clone, Debug)]
pub struct SectionTableEntry {
    name: String,
    offset: i32,
    length: i32,
    value: SectionTableEntryValue,
    description: String,
}

#[derive(Default)]
pub struct SectionTableEntryV2 {
    id: SectionTableEntryId,
    offset: i32,
    length: i32,
}

// Print 'id' as string
impl fmt::Debug for SectionTableEntryV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionTableEntry")
            .field(
                "id",
                &self.id.iter().map(|&c| c as char).collect::<String>(),
            )
            .field("offset", &self.offset)
            .field("lenght", &self.length)
            .finish()
    }
}

// impl SectionTableEntry {
//     fn set_value(&mut self, value: SectionTableEntryValue) -> &mut Self {
//         let _ = mem::replace(&mut self.value, value);
//         self
//     }
//     fn set_offset(&mut self, value: i32) -> &mut Self {
//         let _ = mem::replace(&mut self.offset, value);
//         self
//     }
//     fn set_length(&mut self, value: i32) -> &mut Self {
//         let _ = mem::replace(&mut self.length, value);
//         self
//     }
// }

// #[derive(Default, Debug)]
// struct SectionTable(HashMap<String, SectionTableEntry>);

#[derive(Default, Debug)]
struct SectionTableV2 {
    length: usize,
    sections: [SectionTableEntryV2; 29],
}

impl SectionTableV2 {
    fn get_by_id(&self, id: &SectionTableEntryId) -> Option<&SectionTableEntryV2> {
        self.sections
            .iter()
            .find(|section| return section.id == *id)
    }

    fn init(buffer: &Cursor<u8>) -> Self {
        let section_table_size = HEADER_ENTRIES * size_of::<SectionTableEntryValue>();

        // SEEK

        // let header = &buffer[0..section_table_size];

        // (0..section_table_size).step_by(24).map(f) {
        //     let chunk = &header[chunk_start..chunk_start + 24];

        //     // Each entry must be exactly 24 bytes
        //     if chunk.len() == 24 {
        //         let section_name: String = [chunk[0], chunk[1], chunk[2], chunk[3]]
        //             .iter()
        //             .map(|&c| c as char)
        //             .collect();

        //         table
        //             .get_mut(&section_name)
        //             .unwrap_or_else(|| panic!("Error: section header {section_name} doesn't exist.",))
        //             .set_value([
        //                 i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
        //                 i32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]),
        //                 i32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]),
        //                 i32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]),
        //                 i32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]),
        //             ])
        //             .set_offset(i32::from_le_bytes([
        //                 chunk[8], chunk[9], chunk[10], chunk[11],
        //             ]))
        //             .set_length(i32::from_le_bytes([
        //                 chunk[16], chunk[17], chunk[18], chunk[19],
        //             ]));

        //         // println!("{index}, {:?}, {:?}", descriptor_string, entry.body);
        //             }
        //         }

        let mut x = SectionTableV2 {
            length: 29,
            sections: Default::default(),
        };

        x.sections[5] = SectionTableEntryV2 {
            id: b"BWSG".to_owned(),
            length: 88,
            offset: 99,
        };

        x
    }
}

trait Section {
    const ID: &'static SectionTableEntryId;

    fn parse(&self, table: &SectionTableV2) {}
}

// fn section(key: &str, description: &str) -> (String, SectionTableEntry) {
//     let name = key.to_string();

//     (
//         name.clone(),
//         SectionTableEntry {
//             name,
//             description: description.to_string(),
//             ..Default::default()
//         },
//     )
// }

// impl SectionTable {
//     fn from_hashmap(map: HashMap<String, SectionTableEntry>) -> Self {
//         Self(map)
//     }

//     fn new() -> Self {
//         Self::from_hashmap(HashMap::from([
//             section("BWTB", "unknown"),
//             section("BWST", "Section Table"),
//             section("BWAL", "Asset List"),
//             section("BWCS", "Compiled Space Settings"),
//             section("BWSG", "Static Geometry"),
//             section("BSGD", "Static Geometry Data"),
//             section("BWS2", "unknown"),
//             section("BSG2", "unknown"),
//             section("BWT2", "Terrain 2"),
//             section("BSMI", "unknown"),
//             section("BSMO", "BigWorld Static Model Manger"),
//             section("BSMA", "unknown"),
//             section("SpTr", "unknown"),
//             section("WGSD", "unknown"),
//             section("WTCP", "unknown"),
//             section("BWWa", "unknown"),
//             section("BWEP", "unknown"),
//             section("WGCO", "unknown"),
//             section("BWPs", "unknown"),
//             section("CENT", "Content"),
//             section("UDOS", "User Data Object Section"),
//             section("WGDE", "unknown"),
//             section("BWLC", "unknown"),
//             section("BWVL", "unknown"),
//             section("WTau", "Static Scene Audio Handler"),
//             section("WTbl", "unknown"),
//             section("WGSH", "unknown"),
//             section("WGMM", "unknown"),
//             section("GOBJ", "unknown"),
//         ]))
//     }
// }

// impl Deref for SectionTable {
//     type Target = HashMap<String, SectionTableEntry>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for SectionTable {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

fn error(e: Error, m: &str, path: Option<&String>) -> String {
    return match path {
        Some(path) => format!("{m}: [{}] {}", path, e.to_string()),
        None => format!("{m}: [unknown] {}", e.to_string()),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = File::open(path)
        .unwrap_or_else(|e| panic!("{}", error(e, "Failed to open file", Some(path))));
    let metadata = file
        .metadata()
        .unwrap_or_else(|e| panic!("{}", error(e, "Failed to read metadata", Some(path))));

    let mut buffer = Vec::with_capacity(metadata.len() as usize);

    file.read_to_end(&mut buffer)
        .unwrap_or_else(|e| panic!("{}", error(e, "Failed to read file contents", Some(path))));

    // parse_binary(&buffer, path);

    let x = SectionTableV2::init(&buffer);

    let bwsg = BWSG {};
    bwsg.parse(&x);

    // println!("{:?}", x)
}

// fn parse_binary(buffer: &Vec<u8>, path: &String) {
//     let table = parse_header(&buffer)
//         .unwrap_or_else(|e| panic!("{}", error(e, "Failed to parse header", Some(path))));

//     bwsg(
//         &buffer,
//         table
//             .get("BWSG")
//             .unwrap_or_else(|| panic!("Failed to get BWSG")),
//     );

//     let mut values: Vec<SectionTableEntry> = table.clone().values().cloned().collect();
//     values.sort_by(|a, b| a.offset.cmp(&b.offset));

//     for v in values {
//         println!("{:?}", v);
//     }

//     println!(
//         "Sum of all section lengths: {} (bytes)",
//         table.iter().map(|(_, entry)| entry.value[3]).sum::<i32>()
//     );
// }

// fn parse_header(buffer: &Vec<u8>) -> Result<SectionTable, Error> {
//     let mut table = SectionTable::new();
//     let entry_count = HEADER_ENTRIES * 24;

//     let header = &buffer[0..entry_count];

//     for (index, chunk_start) in (0..entry_count).step_by(24).enumerate() {
//         let chunk = &header[chunk_start..chunk_start + 24];

//         // Each entry must be exactly 24 bytes
//         if chunk.len() == 24 {
//             let section_name: String = [chunk[0], chunk[1], chunk[2], chunk[3]]
//                 .iter()
//                 .map(|&c| c as char)
//                 .collect();

//             table
//                 .get_mut(&section_name)
//                 .unwrap_or_else(|| panic!("Error: section header {section_name} doesn't exist.",))
//                 .set_value([
//                     i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
//                     i32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]),
//                     i32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]),
//                     i32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]),
//                     i32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]),
//                 ])
//                 .set_offset(i32::from_le_bytes([
//                     chunk[8], chunk[9], chunk[10], chunk[11],
//                 ]))
//                 .set_length(i32::from_le_bytes([
//                     chunk[16], chunk[17], chunk[18], chunk[19],
//                 ]));

//             // println!("{index}, {:?}, {:?}", descriptor_string, entry.body);
//         } else {
//             return Err(Error::new(
//                 ErrorKind::InvalidData,
//                 format!("Header entry at index {index} is invalid"),
//             ));
//         }
//     }

//     return Ok(table);
// }

// fn parse_header(buffer: &Vec<u8>) -> Result<SectionTableV2, Error> {
//     // let mut table = SectionTable::new();
//     let section_table_size = HEADER_ENTRIES * size_of::<SectionTableEntryValue>();

//     let header = &buffer[0..section_table_size];

//     (0..section_table_size).step_by(24).map(f) {
//         let chunk = &header[chunk_start..chunk_start + 24];

//         // Each entry must be exactly 24 bytes
//         if chunk.len() == 24 {
//             let section_name: String = [chunk[0], chunk[1], chunk[2], chunk[3]]
//                 .iter()
//                 .map(|&c| c as char)
//                 .collect();

//             table
//                 .get_mut(&section_name)
//                 .unwrap_or_else(|| panic!("Error: section header {section_name} doesn't exist.",))
//                 .set_value([
//                     i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
//                     i32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]),
//                     i32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]),
//                     i32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]),
//                     i32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]),
//                 ])
//                 .set_offset(i32::from_le_bytes([
//                     chunk[8], chunk[9], chunk[10], chunk[11],
//                 ]))
//                 .set_length(i32::from_le_bytes([
//                     chunk[16], chunk[17], chunk[18], chunk[19],
//                 ]));

//             // println!("{index}, {:?}, {:?}", descriptor_string, entry.body);
//         } else {
//             return Err(Error::new(
//                 ErrorKind::InvalidData,
//                 format!("Header entry at index {index} is invalid"),
//             ));
//         }
//     }

//     return Ok(table);
// }
