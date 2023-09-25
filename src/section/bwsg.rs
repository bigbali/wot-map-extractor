use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use super::{Section, SectionId, SectionTable};

#[derive(Debug)]
pub struct BWSG {
    pub strings: HashMap<u32, String>,
}

impl Section for BWSG {
    const ID: &'static SectionId = b"BWSG";

    fn parse(table: &SectionTable, buffer: &mut Cursor<Vec<u8>>) -> Self {
        println!("{:?}", table.get_by_id(Self::ID));

        BWSG {
            strings: HashMap::new(),
        }
    }
}
