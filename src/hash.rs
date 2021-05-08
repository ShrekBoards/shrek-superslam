/// Hashing algorithm used throughout the game. Creates a 4-byte code from the
/// given input `name`.
pub fn hash(name: &str) -> u32 {
    let mut a: u32 = 0;

    for c in name.chars() {
        let mut b = a;
        a >>= 0x1B;
        b <<= 0x05;
        a |= b;
        a ^= c.to_ascii_lowercase() as u32;
    }

    a
}

#[cfg(test)]
mod test {
    use super::*;

    // An assortment of strings found in the game files and their corresponding
    // hashes, which the hash function should be able to recreate.

    #[test]
    fn hash_1() {
        assert_eq!(hash(&"1"), 0x00000031);
    }

    #[test]
    fn hash_bkcape() {
        assert_eq!(hash(&"bk_cape"), 0x53C00A7D);
    }

    #[test]
    fn hash_shrekpuppet_shirtfrontr() {
        assert_eq!(hash(&"shrekpuppet_shirtfrontr"), 0x873DD7A1);
    }
}
