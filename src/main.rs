use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Read,
    io::{Error, ErrorKind},
    mem,
    ops::{Deref, DerefMut},
};

const HEADER_ENTRIES: usize = 29;

type EntryValue = [i32; 5];

#[derive(Default, Clone, Debug)]
struct Entry {
    name: String,
    description: Option<String>,
    value: EntryValue,
}

impl Entry {
    fn set_value(&mut self, value: EntryValue) -> &Self {
        let _ = mem::replace(&mut self.value, value);
        self
    }
}

#[derive(Default, Debug)]
struct SectionTable(HashMap<String, Entry>);

fn section(key: &str, description: &str) -> (String, Entry) {
    let name = key.to_string();

    (
        name.clone(),
        Entry {
            name,
            description: Some(description.to_string()),
            ..Default::default()
        },
    )
}

impl SectionTable {
    fn from_hashmap(map: HashMap<String, Entry>) -> Self {
        Self(map)
    }

    fn new() -> Self {
        Self::from_hashmap(HashMap::from([
            section("BWTB", "unknown"),
            section("BWST", "unknown"),
            section("BWAL", "unknown"),
            section("BWCS", "unknown"),
            section("BWSG", "unknown"),
            section("BSGD", "unknown"),
            section("BWS2", "unknown"),
            section("BSG2", "unknown"),
            section("BWT2", "unknown"),
            section("BSMI", "unknown"),
            section("BSMO", "unknown"),
            section("BSMA", "unknown"),
            section("SpTr", "unknown"),
            section("WGSD", "unknown"),
            section("WTCP", "unknown"),
            section("BWWa", "unknown"),
            section("BWEP", "unknown"),
            section("WGCO", "unknown"),
            section("BWPs", "unknown"),
            section("CENT", "Content"),
            section("UDOS", "unknown"),
            section("WGDE", "unknown"),
            section("BWLC", "unknown"),
            section("BWVL", "unknown"),
            section("WTau", "unknown"),
            section("WTbl", "unknown"),
            section("WGSH", "unknown"),
            section("WGMM", "unknown"),
            section("GOBJ", "unknown"),
        ]))
    }
}

impl Deref for SectionTable {
    type Target = HashMap<String, Entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SectionTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

    parse_binary(buffer, path);
}

fn parse_binary(buffer: Vec<u8>, path: &String) {
    let table = parse_header(buffer)
        .unwrap_or_else(|e| panic!("{}", error(e, "Failed to parse header", Some(path))));

    for (k, v) in table.clone().into_iter() {
        println!("{:?} {:?}", k, v);
    }
}

fn parse_header(buffer: Vec<u8>) -> Result<SectionTable, Error> {
    let mut table = SectionTable::new();
    let entry_count = HEADER_ENTRIES * 24;

    let header = &buffer[0..entry_count];

    for (index, chunk_start) in (0..entry_count).step_by(24).enumerate() {
        let chunk = &header[chunk_start..chunk_start + 24];

        // Each entry must be exactly 24 bytes
        if chunk.len() == 24 {
            let section_name: String = [chunk[0], chunk[1], chunk[2], chunk[3]]
                .iter()
                .map(|&c| c as char)
                .collect();

            table
                .get_mut(&section_name)
                .unwrap_or_else(|| panic!("Error: section header {section_name} doesn't exist.",))
                .set_value([
                    i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
                    i32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]),
                    i32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]),
                    i32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]),
                    i32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]),
                ]);

            // println!("{index}, {:?}, {:?}", descriptor_string, entry.body);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Header entry at index {index} is invalid"),
            ));
        }
    }

    return Ok(table);
}
