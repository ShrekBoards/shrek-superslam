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

    #[test]
    fn hash_game_attackmovetype() {
        assert_eq!(hash(&"Game::AttackMoveType"), 0xEBF07BB5);
    }

    #[test]
    fn hash_game_attackmoveregion() {
        assert_eq!(hash(&"Game::AttackMoveRegion"), 0x8811292E);
    }

    #[test]
    fn hash_game_attackmoveprojectile() {
        assert_eq!(hash(&"Game::AttackMoveProjectile"), 0xF2CFE08D);
    }
}
