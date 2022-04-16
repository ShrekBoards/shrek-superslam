/// Structure describing the dependency of a .bin file to another .bin file.
pub(super) struct BinDependency {}

impl BinDependency {
    /// Get the size in bytes of a single dependency descriptor.
    pub(super) const fn size() -> usize {
        0x80
    }
}