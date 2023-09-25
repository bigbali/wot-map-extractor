use std::{env, fs::File, io::Cursor, io::Read};

mod section;

use crate::section::{bwsg::BWSG, Section, SectionTable};

// each map sector (A0, A1, ..., B9, ...) maps to .cdata_processed
// entry: [?, offset, ?, length, ?]

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = File::open(path).unwrap();
    let metadata = file.metadata().unwrap();

    let mut buffer = Vec::with_capacity(metadata.len() as usize);

    file.read_to_end(&mut buffer)
        .expect("Failed to read file into in-memory buffer");

    let mut in_memory_buffer = Cursor::new(buffer);

    let section_table =
        SectionTable::init(&mut in_memory_buffer).expect("Failed to initialize section table.");

    println!("{:#?}", section_table);

    let bwsg = BWSG::parse(&section_table, &mut in_memory_buffer);

    println!("{:#?}", bwsg);
}
