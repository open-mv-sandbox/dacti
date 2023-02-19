use bytemuck::{Pod, TransparentWrapper, Zeroable};
use wrapmuck::Wrapmuck;

/// Group of indices with specific encoding, starting at a given offset.
///
/// Groups always contain 255 entries of space, but `length` says how many are actually valid.
#[derive(TransparentWrapper, Wrapmuck, Debug, Clone)]
#[repr(transparent)]
pub struct IndexGroupHeader(IndexGroupHeaderRaw);

impl IndexGroupHeader {
    pub fn offset(&self) -> u64 {
        u64::from_le(self.0.offset)
    }

    pub fn set_offset(&mut self, value: u64) {
        self.0.offset = value.to_le();
    }

    pub fn encoding(&self) -> IndexGroupEncoding {
        IndexGroupEncoding::from_bytes(self.0.encoding)
    }

    pub fn set_encoding(&mut self, value: IndexGroupEncoding) {
        self.0.encoding = value.to_bytes();
    }

    pub fn length(&self) -> u8 {
        self.0.length
    }

    pub fn set_length(&mut self, value: u8) {
        self.0.length = value;
    }
}

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
struct IndexGroupHeaderRaw {
    offset: u64,
    encoding: [u8; 4],
    length: u8,
    _reserved: [u8; 3],
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum IndexGroupEncoding {
    None,
    Brotli,
    Unknown([u8; 4]),
}

impl IndexGroupEncoding {
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        let value = match extract_str(&bytes) {
            Ok(value) => value,
            Err(_) => return Self::Unknown(bytes),
        };

        match value {
            "none" => Self::None,
            "brot" => Self::Brotli,
            _ => Self::Unknown(bytes),
        }
    }

    pub fn to_bytes(self) -> [u8; 4] {
        match self {
            Self::None => *b"none",
            Self::Brotli => *b"brot",
            Self::Unknown(bytes) => bytes,
        }
    }
}

fn extract_str(bytes: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let mut length = 4;
    for (i, c) in bytes.iter().enumerate() {
        if *c == 0 {
            length = i;
            break;
        }
    }

    std::str::from_utf8(&bytes[0..length])
}
