use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Read,
    io::{Error, ErrorKind},
};

const HEADER_ENTRIES: usize = 29;

#[derive(Default, Copy, Clone, Debug)]
struct Entry {
    descriptor: [u8; 4],
    body: [i32; 5],
}

#[derive(Default, Debug)]
struct DataTable {
    raw_entries: [Entry; HEADER_ENTRIES],
    table: HashMap<String, [i32; 5]>,
}

impl DataTable {
    fn new() -> DataTable {
        DataTable {
            raw_entries: Default::default(),
            table: HashMap::with_capacity(HEADER_ENTRIES),
        }
    }
}

fn error(e: Error, m: &str, path: Option<&String>) -> String {
    return match path {
        Some(path) => format!("{m}: [{}] {}", path, e.to_string()),
        None => format!("{m}: [unknown] {}", e.to_string()),
    };
}

fn main() {
    // set_hook(Box::new(error));
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

    for (k, v) in table.table.into_iter() {
        println!("{:?}{:?}", k, v);
    }
}

fn parse_header(buffer: Vec<u8>) -> Result<DataTable, Error> {
    let mut table = DataTable::new();
    let entry_count = HEADER_ENTRIES * 24;

    let header = &buffer[0..entry_count];

    for (index, chunk_start) in (0..entry_count).step_by(24).enumerate() {
        let chunk = &header[chunk_start..chunk_start + 24];

        // Each entry must be exactly 24 bytes
        if chunk.len() == 24 {
            let entry = Entry {
                descriptor: [chunk[0], chunk[1], chunk[2], chunk[3]],
                body: [
                    i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
                    i32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]),
                    i32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]),
                    i32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]),
                    i32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]),
                ],
            };

            let descriptor_string: String = entry.descriptor.iter().map(|&c| c as char).collect();

            // println!("{index}, {:?}, {:?}", descriptor_string, entry.body);

            table.raw_entries[index] = entry;
            table.table.insert(descriptor_string, entry.body);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Header entry at index {index} is invalid"),
            ));
        }
    }

    return Ok(table);
}
