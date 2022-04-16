/// Structure that comes after the dependencies in the .bin file, don't yet
/// know what it does.
pub(super) struct BinOffset4Struct {}

impl BinOffset4Struct {
    /// Get the size in bytes of a single entry.
    pub(super) const fn size() -> usize {
        0x40
    }
}