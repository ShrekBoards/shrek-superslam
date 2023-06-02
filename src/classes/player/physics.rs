use serde::{Deserialize, Serialize};

use crate::classes::{SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::PhysicsFighting` object type.
#[derive(Deserialize, Serialize)]
pub struct PhysicsFighting {
    pub m_airborn_land_dist: f32,
    pub m_airborn_no_catch_bias: f32,
    pub ground_leave_dist_walk: f32,
    pub ground_leave_dist_stairs: f32,
    pub m_jump_up_vel_influence: f32,
    pub jump_takeoff_max_time: f32,
    pub jump_delay: f32,
    pub off_ledge_vel_stop: f32,
    pub off_ledge_vel_drop: f32,
    pub off_ledge_vel_roll: f32,
    pub block_time_max: f32,
    pub block_time_recharge: f32,
    pub block_power_max: f32,
    pub block_power_recharge: f32,
    pub jump_peak_vel: f32,
    pub jump_takeoff_region_time: f32,
    pub jump_power_normal: f32,
    pub jump_power_double: f32,
    pub jump_teleport_up_dist: f32,
    pub jump_teleport_flat_dist: f32,
    pub jump_teleport_flat_up_dist: f32,
    pub jump_power_ledge: f32,
    pub jump_power_wall: f32,
    pub jump_power_wall_outward: f32,
    pub jump_power_grapple: f32,
    pub dump_up_vel: f32,
    pub dump_flat_vel: f32,
    pub accumulated_vel_air_fric: f32,
    pub accumulated_vel_ground_fric: f32,
    pub wall_bounce_low: f32,
    pub wall_bounce_medium: f32,
    pub wall_bounce_high: f32,
    pub hit_pause_delay: f32,
    pub hit_pause_light: f32,
    pub hit_pause_medium: f32,
    pub hit_pause_strong: f32,
    pub attack_speed_filter: f32,
    pub move_speed_filter: f32,
    pub min_loose_camera_radius: f32,
    pub max_loose_camera_radius: f32,
    pub tight_camera_radius: f32,
    pub camera_actual_vel_influence: f32,
    pub camera_actual_vel_max: f32,
    pub camera_vel_influence: f32,
    pub camera_vel_max: f32,
    pub deflect_time_max: f32,
    pub deflection_delay: f32,
    pub deflect_reset_time: f32,
    pub jump_teleport_time: f32,
    pub juggle_gravity: f32,
    pub knockback_fric_ground: f32,
    pub knockback_fric_air: f32,
    pub knockdown_fric: f32,
    pub m_airborn_land_dist_stairs: f32,
    pub wall_trap_hit_timer: f32,
    pub wall_trap_up_vector: f32,
    pub wall_trap_vel_multiplier: f32,
    pub wall_trap_pvp_disable_timer: f32,
    pub block_gravity: f32,
    pub dronkey_jump_foward: f32,
    pub dronkey_jump_up: f32,
    pub dronkey_jump_time: f32,
    pub move_speed_up: f32,
    pub move_speed_up_max: f32,
    pub move_speed_down: f32,
    pub move_speed_down_max: f32,
    pub def_stun_1: f32,
    pub def_kb_mult_1: f32,
    pub def_speed_mult_1: f32,
    pub def_speed_mult_2: f32,
    pub def_speed_mult_3: f32,
    pub def_speed_mult_4: f32,
    pub def_speed_mult_5: f32,
    pub def_speed_mult_6: f32,
    pub def_pow_mult_1: f32,
    pub def_pow_mult_2: f32,
    pub def_pow_mult_3: f32,
    pub item_drop_knockup: f32,
    pub item_drop_knockback: f32,
    pub throwable_drop_knockup: f32,
    pub throwable_drop_knockback: f32,
    pub walkup_angle_tolerance: f32,
}

impl SerialisedShrekSuperSlamGameObject for PhysicsFighting {
    /// Returns the hashcode for the `Game::AttackMoveType` in-game object.
    fn hash() -> u32 {
        0xADDDF1EC
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::PhysicsFighting"
    }

    /// Returns the size of a serialised `Game::PhysicsFighting` object.
    fn size() -> usize {
        0xD50
    }

    /// Return a new `PhysicsFighting` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<PhysicsFighting, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // Read numeric fields
        let m_airborn_land_dist = c.read_f32(&raw[offset + 0x0838..offset + 0x083C])?;
        let m_airborn_no_catch_bias = c.read_f32(&raw[offset + 0x083C..offset + 0x0840])?;
        let ground_leave_dist_walk = c.read_f32(&raw[offset + 0x0840..offset + 0x0844])?;
        let ground_leave_dist_stairs = c.read_f32(&raw[offset + 0x0844..offset + 0x0848])?;

        let m_jump_up_vel_influence = c.read_f32(&raw[offset + 0x084C..offset + 0x0850])?;
        let jump_takeoff_max_time = c.read_f32(&raw[offset + 0x0850..offset + 0x0854])?;
        let jump_delay = c.read_f32(&raw[offset + 0x0854..offset + 0x0858])?;
        let off_ledge_vel_stop = c.read_f32(&raw[offset + 0x0858..offset + 0x085C])?;
        let off_ledge_vel_drop = c.read_f32(&raw[offset + 0x085C..offset + 0x0860])?;
        let off_ledge_vel_roll = c.read_f32(&raw[offset + 0x0860..offset + 0x0864])?;

        let block_time_max = c.read_f32(&raw[offset + 0x0894..offset + 0x0898])?;
        let block_time_recharge = c.read_f32(&raw[offset + 0x0898..offset + 0x089C])?;

        let block_power_max = c.read_f32(&raw[offset + 0x08A0..offset + 0x08A4])?;
        let block_power_recharge = c.read_f32(&raw[offset + 0x08A4..offset + 0x08A8])?;

        let jump_peak_vel = c.read_f32(&raw[offset + 0x08AC..offset + 0x08B0])?;

        let jump_takeoff_region_time = c.read_f32(&raw[offset + 0x08C4..offset + 0x08C8])?;
        let jump_power_normal = c.read_f32(&raw[offset + 0x08C8..offset + 0x08CC])?;
        let jump_power_double = c.read_f32(&raw[offset + 0x08CC..offset + 0x08D0])?;
        let jump_teleport_up_dist = c.read_f32(&raw[offset + 0x08D0..offset + 0x08D4])?;
        let jump_teleport_flat_dist = c.read_f32(&raw[offset + 0x08D4..offset + 0x08D8])?;
        let jump_teleport_flat_up_dist = c.read_f32(&raw[offset + 0x08D8..offset + 0x08DC])?;
        let jump_power_ledge = c.read_f32(&raw[offset + 0x08DC..offset + 0x08E0])?;
        let jump_power_wall = c.read_f32(&raw[offset + 0x08E0..offset + 0x08E4])?;
        let jump_power_wall_outward = c.read_f32(&raw[offset + 0x08E4..offset + 0x08E8])?;
        let jump_power_grapple = c.read_f32(&raw[offset + 0x08E8..offset + 0x08EC])?;
        let dump_up_vel = c.read_f32(&raw[offset + 0x08EC..offset + 0x08F0])?;
        let dump_flat_vel = c.read_f32(&raw[offset + 0x08F0..offset + 0x08F4])?;

        let accumulated_vel_air_fric = c.read_f32(&raw[offset + 0x09F4..offset + 0x09F8])?;
        let accumulated_vel_ground_fric = c.read_f32(&raw[offset + 0x09F8..offset + 0x09FC])?;
        let wall_bounce_low = c.read_f32(&raw[offset + 0x09FC..offset + 0x0A00])?;
        let wall_bounce_medium = c.read_f32(&raw[offset + 0x0A00..offset + 0x0A04])?;
        let wall_bounce_high = c.read_f32(&raw[offset + 0x0A04..offset + 0x0A08])?;
        let hit_pause_delay = c.read_f32(&raw[offset + 0x0A08..offset + 0x0A0C])?;
        let hit_pause_light = c.read_f32(&raw[offset + 0x0A0C..offset + 0x0A10])?;
        let hit_pause_medium = c.read_f32(&raw[offset + 0x0A10..offset + 0x0A14])?;
        let hit_pause_strong = c.read_f32(&raw[offset + 0x0A14..offset + 0x0A18])?;

        let attack_speed_filter = c.read_f32(&raw[offset + 0x0A2C..offset + 0x0A30])?;

        let move_speed_filter = c.read_f32(&raw[offset + 0x0A38..offset + 0x0A3C])?;

        let min_loose_camera_radius = c.read_f32(&raw[offset + 0x0A60..offset + 0x0A64])?;
        let max_loose_camera_radius = c.read_f32(&raw[offset + 0x0A64..offset + 0x0A68])?;

        let tight_camera_radius = c.read_f32(&raw[offset + 0x0A6C..offset + 0x0A70])?;
        let camera_actual_vel_influence = c.read_f32(&raw[offset + 0x0A70..offset + 0x0A74])?;
        let camera_actual_vel_max = c.read_f32(&raw[offset + 0x0A74..offset + 0x0A78])?;
        let camera_vel_influence = c.read_f32(&raw[offset + 0x0A78..offset + 0x0A7C])?;
        let camera_vel_max = c.read_f32(&raw[offset + 0x0A7C..offset + 0x0A80])?;

        let deflect_time_max = c.read_f32(&raw[offset + 0x0A84..offset + 0x0A88])?;

        let deflection_delay = c.read_f32(&raw[offset + 0x0A9C..offset + 0x0AA0])?;

        let deflect_reset_time = c.read_f32(&raw[offset + 0x0AC8..offset + 0x0ACC])?;

        let jump_teleport_time = c.read_f32(&raw[offset + 0x0AE4..offset + 0x0AE8])?;
        let juggle_gravity = c.read_f32(&raw[offset + 0x0AE8..offset + 0x0AEC])?;

        let knockback_fric_ground = c.read_f32(&raw[offset + 0x0B2C..offset + 0x0B30])?;
        let knockback_fric_air = c.read_f32(&raw[offset + 0x0B34..offset + 0x0B38])?;
        let knockdown_fric = c.read_f32(&raw[offset + 0x0B38..offset + 0x0B3C])?;
        let m_airborn_land_dist_stairs = c.read_f32(&raw[offset + 0x0B3C..offset + 0x0B40])?;

        let wall_trap_hit_timer = c.read_f32(&raw[offset + 0x0B4C..offset + 0x0B50])?;
        let wall_trap_up_vector = c.read_f32(&raw[offset + 0x0B50..offset + 0x0B54])?;
        let wall_trap_vel_multiplier = c.read_f32(&raw[offset + 0x0B54..offset + 0x0B58])?;
        let wall_trap_pvp_disable_timer = c.read_f32(&raw[offset + 0x0B58..offset + 0x0B5C])?;

        let block_gravity = c.read_f32(&raw[offset + 0x0C48..offset + 0x0C4C])?;

        let dronkey_jump_foward = c.read_f32(&raw[offset + 0x0C58..offset + 0x0C5C])?;
        let dronkey_jump_up = c.read_f32(&raw[offset + 0x0C5C..offset + 0x0C60])?;
        let dronkey_jump_time = c.read_f32(&raw[offset + 0x0C60..offset + 0x0C64])?;

        let move_speed_up = c.read_f32(&raw[offset + 0x0C80..offset + 0x0C84])?;
        let move_speed_up_max = c.read_f32(&raw[offset + 0x0C84..offset + 0x0C88])?;
        let move_speed_down = c.read_f32(&raw[offset + 0x0C88..offset + 0x0C8C])?;
        let move_speed_down_max = c.read_f32(&raw[offset + 0x0C8C..offset + 0x0C90])?;

        let def_stun_1 = c.read_f32(&raw[offset + 0x0CDC..offset + 0x0CE0])?;
        let def_kb_mult_1 = c.read_f32(&raw[offset + 0x0CE0..offset + 0x0CE4])?;
        let def_speed_mult_1 = c.read_f32(&raw[offset + 0x0CE4..offset + 0x0CE8])?;
        let def_speed_mult_2 = c.read_f32(&raw[offset + 0x0CE8..offset + 0x0CEC])?;
        let def_speed_mult_3 = c.read_f32(&raw[offset + 0x0CEC..offset + 0x0CF0])?;
        let def_speed_mult_4 = c.read_f32(&raw[offset + 0x0CF0..offset + 0x0CF4])?;
        let def_speed_mult_5 = c.read_f32(&raw[offset + 0x0CF4..offset + 0x0CF8])?;
        let def_speed_mult_6 = c.read_f32(&raw[offset + 0x0CF8..offset + 0x0CFC])?;
        let def_pow_mult_1 = c.read_f32(&raw[offset + 0x0CFC..offset + 0x0D00])?;
        let def_pow_mult_2 = c.read_f32(&raw[offset + 0x0D00..offset + 0x0D04])?;
        let def_pow_mult_3 = c.read_f32(&raw[offset + 0x0D04..offset + 0x0D08])?;
        let item_drop_knockup = c.read_f32(&raw[offset + 0x0D08..offset + 0x0D0C])?;
        let item_drop_knockback = c.read_f32(&raw[offset + 0x0D0C..offset + 0x0D10])?;

        let throwable_drop_knockup = c.read_f32(&raw[offset + 0x0D14..offset + 0x0D18])?;
        let throwable_drop_knockback = c.read_f32(&raw[offset + 0x0D18..offset + 0x0D1C])?;
        let walkup_angle_tolerance = c.read_f32(&raw[offset + 0x0D1C..offset + 0x0D20])?;

        Ok(PhysicsFighting {
            m_airborn_land_dist,
            m_airborn_no_catch_bias,
            ground_leave_dist_walk,
            ground_leave_dist_stairs,
            m_jump_up_vel_influence,
            jump_takeoff_max_time,
            jump_delay,
            off_ledge_vel_stop,
            off_ledge_vel_drop,
            off_ledge_vel_roll,
            block_time_max,
            block_time_recharge,
            block_power_max,
            block_power_recharge,
            jump_peak_vel,
            jump_takeoff_region_time,
            jump_power_normal,
            jump_power_double,
            jump_teleport_up_dist,
            jump_teleport_flat_dist,
            jump_teleport_flat_up_dist,
            jump_power_ledge,
            jump_power_wall,
            jump_power_wall_outward,
            jump_power_grapple,
            dump_up_vel,
            dump_flat_vel,
            accumulated_vel_air_fric,
            accumulated_vel_ground_fric,
            wall_bounce_low,
            wall_bounce_medium,
            wall_bounce_high,
            hit_pause_delay,
            hit_pause_light,
            hit_pause_medium,
            hit_pause_strong,
            attack_speed_filter,
            move_speed_filter,
            min_loose_camera_radius,
            max_loose_camera_radius,
            tight_camera_radius,
            camera_actual_vel_influence,
            camera_actual_vel_max,
            camera_vel_influence,
            camera_vel_max,
            deflect_time_max,
            deflection_delay,
            deflect_reset_time,
            jump_teleport_time,
            juggle_gravity,
            knockback_fric_ground,
            knockback_fric_air,
            knockdown_fric,
            m_airborn_land_dist_stairs,
            wall_trap_hit_timer,
            wall_trap_up_vector,
            wall_trap_vel_multiplier,
            wall_trap_pvp_disable_timer,
            block_gravity,
            dronkey_jump_foward,
            dronkey_jump_up,
            dronkey_jump_time,
            move_speed_up,
            move_speed_up_max,
            move_speed_down,
            move_speed_down_max,
            def_stun_1,
            def_kb_mult_1,
            def_speed_mult_1,
            def_speed_mult_2,
            def_speed_mult_3,
            def_speed_mult_4,
            def_speed_mult_5,
            def_speed_mult_6,
            def_pow_mult_1,
            def_pow_mult_2,
            def_pow_mult_3,
            item_drop_knockup,
            item_drop_knockback,
            throwable_drop_knockup,
            throwable_drop_knockback,
            walkup_angle_tolerance,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for PhysicsFighting {
    /// Writes the object back to its `bin` file at the given `offset`.
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), Error> {
        // Write numeric fields.
        write_to_bin(bin, offset, 0x0838, self.m_airborn_land_dist)?;
        write_to_bin(bin, offset, 0x083C, self.m_airborn_no_catch_bias)?;
        write_to_bin(bin, offset, 0x0840, self.ground_leave_dist_walk)?;
        write_to_bin(bin, offset, 0x0844, self.ground_leave_dist_stairs)?;

        write_to_bin(bin, offset, 0x084C, self.m_jump_up_vel_influence)?;
        write_to_bin(bin, offset, 0x0850, self.jump_takeoff_max_time)?;
        write_to_bin(bin, offset, 0x0854, self.jump_delay)?;
        write_to_bin(bin, offset, 0x0858, self.off_ledge_vel_stop)?;
        write_to_bin(bin, offset, 0x085C, self.off_ledge_vel_roll)?;

        write_to_bin(bin, offset, 0x0894, self.block_time_max)?;
        write_to_bin(bin, offset, 0x0898, self.block_time_recharge)?;

        write_to_bin(bin, offset, 0x08A0, self.block_power_max)?;
        write_to_bin(bin, offset, 0x08A4, self.block_power_recharge)?;

        write_to_bin(bin, offset, 0x08AC, self.jump_peak_vel)?;

        write_to_bin(bin, offset, 0x08C4, self.jump_takeoff_region_time)?;
        write_to_bin(bin, offset, 0x08C8, self.jump_power_normal)?;
        write_to_bin(bin, offset, 0x08CC, self.jump_power_double)?;
        write_to_bin(bin, offset, 0x08D0, self.jump_teleport_up_dist)?;
        write_to_bin(bin, offset, 0x08D4, self.jump_teleport_flat_dist)?;
        write_to_bin(bin, offset, 0x08D8, self.jump_teleport_flat_up_dist)?;
        write_to_bin(bin, offset, 0x08DC, self.jump_power_ledge)?;
        write_to_bin(bin, offset, 0x08E0, self.jump_power_wall)?;
        write_to_bin(bin, offset, 0x08E4, self.jump_power_wall_outward)?;
        write_to_bin(bin, offset, 0x08E8, self.jump_power_grapple)?;
        write_to_bin(bin, offset, 0x08EC, self.dump_up_vel)?;
        write_to_bin(bin, offset, 0x08F0, self.dump_flat_vel)?;

        write_to_bin(bin, offset, 0x09F4, self.accumulated_vel_air_fric)?;
        write_to_bin(bin, offset, 0x09F8, self.accumulated_vel_ground_fric)?;
        write_to_bin(bin, offset, 0x09FC, self.wall_bounce_low)?;
        write_to_bin(bin, offset, 0x0A00, self.wall_bounce_medium)?;
        write_to_bin(bin, offset, 0x0A04, self.wall_bounce_high)?;
        write_to_bin(bin, offset, 0x0A08, self.hit_pause_delay)?;
        write_to_bin(bin, offset, 0x0A0C, self.hit_pause_light)?;
        write_to_bin(bin, offset, 0x0A10, self.hit_pause_medium)?;
        write_to_bin(bin, offset, 0x0A14, self.hit_pause_strong)?;

        write_to_bin(bin, offset, 0x0A2C, self.attack_speed_filter)?;

        write_to_bin(bin, offset, 0x0A38, self.move_speed_filter)?;

        write_to_bin(bin, offset, 0x0A60, self.min_loose_camera_radius)?;
        write_to_bin(bin, offset, 0x0A64, self.max_loose_camera_radius)?;

        write_to_bin(bin, offset, 0x0A6C, self.tight_camera_radius)?;
        write_to_bin(bin, offset, 0x0A70, self.camera_actual_vel_influence)?;
        write_to_bin(bin, offset, 0x0A74, self.camera_actual_vel_max)?;
        write_to_bin(bin, offset, 0x0A78, self.camera_vel_influence)?;
        write_to_bin(bin, offset, 0x0A7C, self.camera_vel_max)?;

        write_to_bin(bin, offset, 0x0A84, self.deflect_time_max)?;

        write_to_bin(bin, offset, 0x0A9C, self.deflection_delay)?;

        write_to_bin(bin, offset, 0x0AC8, self.deflect_reset_time)?;

        write_to_bin(bin, offset, 0x0AE4, self.jump_teleport_time)?;
        write_to_bin(bin, offset, 0x0AE8, self.juggle_gravity)?;

        write_to_bin(bin, offset, 0x0B2C, self.knockback_fric_ground)?;
        write_to_bin(bin, offset, 0x0B30, self.knockback_fric_air)?;
        write_to_bin(bin, offset, 0x0B34, self.knockdown_fric)?;
        write_to_bin(bin, offset, 0x0B38, self.m_airborn_land_dist_stairs)?;

        write_to_bin(bin, offset, 0x0B4C, self.wall_trap_hit_timer)?;
        write_to_bin(bin, offset, 0x0B50, self.wall_trap_up_vector)?;
        write_to_bin(bin, offset, 0x0B54, self.wall_trap_vel_multiplier)?;
        write_to_bin(bin, offset, 0x0B58, self.wall_trap_pvp_disable_timer)?;

        write_to_bin(bin, offset, 0x0C48, self.block_gravity)?;

        write_to_bin(bin, offset, 0x0C58, self.dronkey_jump_foward)?;
        write_to_bin(bin, offset, 0x0C5C, self.dronkey_jump_up)?;
        write_to_bin(bin, offset, 0x0C60, self.dronkey_jump_time)?;

        write_to_bin(bin, offset, 0x0C80, self.move_speed_up)?;
        write_to_bin(bin, offset, 0x0C84, self.move_speed_up_max)?;
        write_to_bin(bin, offset, 0x0C88, self.move_speed_down)?;
        write_to_bin(bin, offset, 0x0C8C, self.move_speed_down_max)?;

        write_to_bin(bin, offset, 0x0CDC, self.def_stun_1)?;
        write_to_bin(bin, offset, 0x0CE0, self.def_kb_mult_1)?;
        write_to_bin(bin, offset, 0x0CE4, self.def_speed_mult_1)?;
        write_to_bin(bin, offset, 0x0CE8, self.def_speed_mult_2)?;
        write_to_bin(bin, offset, 0x0CEC, self.def_speed_mult_3)?;
        write_to_bin(bin, offset, 0x0CF0, self.def_speed_mult_4)?;
        write_to_bin(bin, offset, 0x0CF4, self.def_speed_mult_5)?;
        write_to_bin(bin, offset, 0x0CF8, self.def_speed_mult_6)?;
        write_to_bin(bin, offset, 0x0CFC, self.def_pow_mult_1)?;
        write_to_bin(bin, offset, 0x0D00, self.def_pow_mult_2)?;
        write_to_bin(bin, offset, 0x0D04, self.def_pow_mult_3)?;
        write_to_bin(bin, offset, 0x0D08, self.item_drop_knockup)?;
        write_to_bin(bin, offset, 0x0D0C, self.item_drop_knockback)?;

        write_to_bin(bin, offset, 0x0D14, self.throwable_drop_knockup)?;
        write_to_bin(bin, offset, 0x0D18, self.throwable_drop_knockback)?;
        write_to_bin(bin, offset, 0x0D1C, self.walkup_angle_tolerance)?;

        Ok(())
    }
}

fn write_to_bin(bin: &mut Bin, original_offset: usize, offset_in_obj: usize, value: f32) -> Result<(), Error> {
    let starting_offset = original_offset + offset_in_obj;
    bin.raw.splice(starting_offset..starting_offset + 0x04, bin.console.write_f32(value)?);
    Ok(())
}