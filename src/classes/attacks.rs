use crate::classes::ShrekSuperSlamGameObject;

pub mod AttackMoveType {

}

impl ShrekSuperSlamGameObject for AttackMoveType {
    const fn hash() -> u32 {
        0xEBF07BB5
    }

    const fn name() -> &'static str {
        "Game::AttackMoveType"
    }
}