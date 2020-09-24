use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::files::Bin;

pub struct LocalizedString {
    /// The contents of the string
    pub string: String,

    /// Unknown value that resides at +04, seems to be 0 if the string is empty
    unknown: u32,
}

impl SerialisedShrekSuperSlamGameObject for LocalizedString {
    fn hash() -> u32 {
        0xBFC7788D
    }

    fn name() -> &'static str {
        "gf::LocalizedString"
    }

    fn size() -> usize {
        0x0C
    }

    fn new(bin: &Bin, offset: usize) -> LocalizedString {
        let x = bin.console.read_u32(&bin.raw[offset + 0x04..offset + 0x08]);
        let str_offset = bin.console.read_u32(&bin.raw[offset + 0x08..offset + 0x0C]);
        LocalizedString {
            string: bin.get_str_from_offset(str_offset).unwrap(),
            unknown: x,
        }
    }
}

impl LocalizedString {
    pub fn is_empty(&self) -> bool {
        self.unknown == 0
    }
}