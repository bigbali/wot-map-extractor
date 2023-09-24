use std::io::SeekFrom;

use crate::{Section, SectionTableEntry, SectionTableEntryId, SectionTableV2};

pub struct BWSG {}

impl Section for BWSG {
    const ID: &'static SectionTableEntryId = b"BWSG";
    fn parse(&self, table: &SectionTableV2) {
        println!("{:?}", table.get_by_id(Self::ID));
        // println!("{:?}", table);
    }
}

pub fn bwsg(buffer: &Vec<u8>, section: &SectionTableEntry) {
    // let [_, section_start, _, length, _] = section.value;
    let (offset, length) = (section.offset, section.length);

    // println!("section_start: {}", section_start);
    // println!("section length: {}", length);

    let start = (offset) as usize;

    // for (i) in buffer[start..(start + length as usize)]
    //     .iter()
    //     .step_by(32)
    //     .enumerate()
    // {
    //     let mut a: Vec<u8> = Vec::new();
    //     for s in 0..32 {
    //         a.push(buffer[start + (i + s)])
    //     }
    //     // println!("{:?}", a);

    //     let x: String = a.iter().map(|&c| c as char).collect();
    //     println!("{:?}", x);
    // }

    for i in 0..(length as usize / 32) {
        let chunk_start = start + i * 32 + 1; // Skip the binary value at the start

        let mut a: Vec<u8> = Vec::new();
        for s in 0..32 {
            // Iterate over 31 bytes, excluding the binary value
            a.push(buffer[chunk_start + s]);
        }

        let x: String = a.iter().map(|&c| c as char).collect();
        println!("{:?}", x);
    }
}
