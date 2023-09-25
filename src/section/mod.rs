use std::{
    fmt::{self, Formatter},
    io::{self, Cursor, Read, Seek},
};

pub mod bwsg;

const SECTIONS: usize = 29;
const SECTION_DATA_BYTES: usize = 20;

#[derive(Debug)]
pub struct SectionTable {
    pub length: usize,
    pub sections: [SectionMetadata; 29],
}

pub type SectionId = [u8; 4];

pub struct SectionMetadata {
    id: SectionId,
    offset: i32,
    length: i32,
}

// Print 'id' as string
impl fmt::Debug for SectionMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionMetadata")
            .field(
                "id",
                &self.id.iter().map(|&c| c as char).collect::<String>(),
            )
            .field("offset", &self.offset)
            .field("length", &self.length)
            .finish()
    }
}

pub trait Section {
    const ID: &'static SectionId;

    fn parse(table: &SectionTable, buffer: &mut Cursor<Vec<u8>>) -> Self;
}

pub trait SeekSection {
    fn get_id(&mut self) -> Result<[u8; 4], io::Error>;
    fn get_i32(&mut self) -> Result<i32, io::Error>;
    fn get_u32(&mut self) -> Result<u32, io::Error>;
    fn skip(&mut self, bytes: usize) {}
}

impl<T> SeekSection for T
where
    T: Read + Seek,
{
    fn get_id(&mut self) -> Result<[u8; 4], io::Error> {
        let mut buffer = [0; 4];

        match self.read_exact(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        }
    }

    fn get_i32(&mut self) -> Result<i32, io::Error> {
        let mut buffer = [0; 4];

        match self.read_exact(&mut buffer) {
            Ok(_) => Ok(i32::from_le_bytes(buffer)),
            Err(e) => Err(e),
        }
    }

    fn get_u32(&mut self) -> Result<u32, io::Error> {
        let mut buffer = [0; 4];

        match self.read_exact(&mut buffer) {
            Ok(_) => Ok(u32::from_le_bytes(buffer)),
            Err(e) => Err(e),
        }
    }

    fn skip(&mut self, bytes: usize) {}
}

impl SectionTable {
    pub fn get_by_id(&self, id: &SectionId) -> Option<&SectionMetadata> {
        self.sections
            .iter()
            .find(|section| return section.id == *id)
    }

    pub fn init(buffer: &mut Cursor<Vec<u8>>) -> Result<Self, io::Error> {
        let section_table_size = SECTIONS * SECTION_DATA_BYTES;

        let sections: [SectionMetadata; SECTIONS] = (0..SECTIONS)
            .map(|_| {
                let id = buffer.get_id().expect("ID");
                let _ = buffer.get_i32().unwrap();
                let offset = buffer.get_i32().expect("offset");
                let _ = buffer.get_i32().unwrap();
                let length = buffer.get_i32().expect("len");
                let _ = buffer.get_i32().unwrap();

                SectionMetadata { id, offset, length }
            })
            .collect::<Vec<SectionMetadata>>()
            .try_into()
            .unwrap();

        Ok(SectionTable {
            length: section_table_size,
            sections,
        })
    }
}
