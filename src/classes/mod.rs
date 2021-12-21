//! Module containing structures representing the various Shrek SuperSlam
//! classes.
//!
//! The .db.bin files seen throughout the extracted MASTER.DAT are essentially
//! a collection of serialised game objects. The serialisation is very basic
//! and is nearly the entire object as it would exist in memory dumped into
//! the file.
//!
//! Each object begins with a unique hash identifier. This is created using the
//! game's hashing method on the class name. At runtime, these hashes are
//! converted to the vtable for the class, which is then used to call class
//! methods. Many objects also contain offsets to other objects. These are
//! stored as offsets within the file, excluding the header. Therefore all
//! offsets need 0x40 added to find the true file offset. At runtime, these are
//! converted to pointers to the real memory addresses.
//!
//! For the purpose of this library, we can define the data fields of a given
//! class, then given the type we want to deserialise and an offset within the
//! .db.bin file to find the object, flesh out a Rust type with the data fields
//! found in the file. This module contains the structure definitions for the
//! serialised classes, and the method [`Bin::get_object_from_offset`] constructs
//! these types from a given offset in a .db.bin file.
//!
//! Of particular note is the [`db::GfDb`] class, which begins each .db.bin
//! file and acts as a table of contents for the rest of the file, by
//! containing an entry for each top-level class within the file, as well as a
//! name for the object. This can then be used to determine other types within
//! the file. An example of extracting this class is given here:
//!
//! ```no_run
//! use std::path::Path;
//! use shrek_superslam::{Console, MasterDat, MasterDir};
//! use shrek_superslam::classes::GfDb;
//! use shrek_superslam::files::Bin;
//!
//! // Extract the .db.bin file from the MASTER.DAT
//! let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
//! let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
//! let my_file_bytes = master_dat.decompressed_file("data\\players\\shrek\\player.db.bin").unwrap();
//!
//! // Parse the file and get the gf::DB object
//! let bin = Bin::new(my_file_bytes, Console::PC).unwrap();
//! let gf_db = bin.get_object_from_offset::<GfDb>(0x00).unwrap();
//! ```
mod db;
mod error;
mod item;
mod level;
mod object_initializer;
mod player;
mod strings;
mod util;

pub use db::*;
pub use error::Error;
pub use item::*;
pub use level::*;
pub use object_initializer::*;
pub use player::*;
pub use strings::*;

use crate::errors;
use crate::files::Bin;

/// Trait for structures representing serialised Shrek SuperSlam game objects
/// that appear in the game's .bin files
pub trait SerialisedShrekSuperSlamGameObject: Sized {
    /// Returns the hash value identifying the class.
    fn hash() -> u32;

    /// Returns the name of the class.
    fn name() -> &'static str;

    /// Returns the size in bytes the serialised object takes up in a .bin file.
    fn size() -> usize;

    /// Returns an instance of the object from the given `offset` in the given
    /// `Bin` structure.
    ///
    /// # Remarks
    ///
    /// Do not call directly, instead use [`Bin::get_object_from_offset`] to
    /// get objects from a .bin file.
    fn new(bin: &Bin, offset: usize) -> Result<Self, errors::Error>;
}

/// Trait for structures representing Shrek SuperSlam game objects that can be
/// written back to a .bin file to overwrite the original.
pub trait WriteableShrekSuperSlamGameObject {
    /// Overwrites the game object at the given `offset` within the given `Bin`
    /// file.
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), errors::Error>;
}

/// Enumeration of the different class types.
///
/// This allows parsing a .db.bin file to a collection of the types contained
/// within it.
pub enum ShrekSuperSlamObject {
    AttackMoveRegion(AttackMoveRegion),
    AttackMoveType(AttackMoveType),
    BehaviorAiFighting(BehaviorAiFighting),
    BehaviorFightingControlShrek(BehaviorFightingControlShrek),
    DynamicThrowable(DynamicThrowable),
    EffectStringReference(EffectStringReference),
    Entity(Entity),
    EventSequence(EventSequence),
    GameWorld(GameWorld),
    GfDb(GfDb),
    ItemSpawner(ItemSpawner),
    LocalizedString(LocalizedString),
    LookAtData(LookAtData),
    ObjectInitializer(ObjectInitializer),
    PhysicsFighting(PhysicsFighting),
    PotionType(PotionType),
    PowerupType(PowerupType),
    ProjectileType(ProjectileType),
    RenderSpawn(RenderSpawn),
    ScriptDb(ScriptDb),
    Spitter(Spitter),
    SpitterKeyframe(SpitterKeyframe),
    WeaponType(WeaponType),
}

/// Lookup a hash value and retrieve the name of the class corresponding to the hash
///
/// # Parameters
///
/// - `hash`: The hash to lookup
///
/// # Returns
///
/// A Some(&'static str) of the matching class name, if any. None if the hash
/// given does not match any class names.
pub(crate) fn hash_lookup(hash: u32) -> Option<&'static str> {
    match hash {
        0xB974E53B => Some("Game::GameWorld"),
        0xC239E1AB => Some("Game::RotationObject"),
        0xE7DE386D => Some("Game::FrozenObject"),
        0xC81858A7 => Some("Game::HitPointObject"),
        0xD94AF267 => Some("Game::DangerObject"),
        0x90B7F7D9 => Some("Game::LoopingCombinationObject"),
        0xEF9A1498 => Some("Game::EffectObject"),
        0x9848AD18 => Some("Game::SoundObject"),
        0x9386D5F8 => Some("Game::SoundBox"),
        0x850B9B2F => Some("Game::ForceBox"),
        0xCE55537F => Some("Game::DestructibleObject"),
        0x87C01D61 => Some("Game::ReplacementItemSpawner"),
        0xCD47AA2B => Some("Game::ItemSpawner"),
        0xA1773A21 => Some("Game::PlayerEffect"),
        0xC320FF18 => Some("Game::EffectStarter"),
        0xDBFB4A35 => Some("Game::ObjectInitializer"),
        0x9ACD0AD1 => Some("Game::PadTrigger"),
        0x9414A807 => Some("Game::HintTrigger"),
        0xEEFC4B3C => Some("Game::PlayerFlagTrigger"),
        0xD508DEF3 => Some("Game::HoldingItemTrigger"),
        0xC1CB398A => Some("Game::ItemTrigger"),
        0xB2569A8D => Some("Game::TriggerTeleport"),
        0xB576B7A0 => Some("Game::TriggerMode"),
        0xC0458C1A => Some("Game::TriggerEventSequence"),
        0xBB8828E8 => Some("Game::Trigger"),
        0x90D8FCD6 => Some("Game::Spitter"),
        0x84AD7E70 => Some("Game::SpitterKeyframe"),
        0x9EDD2BDB => Some("Game::ShellController"),
        0xCB4A46CD => Some("Game::PadStatus"),
        0xA995C17E => Some("Game::LoseOnTime"),
        0xBB17DA40 => Some("Game::WinThreshold"),
        0x96067A2C => Some("Game::LoseOnNoLives"),
        0xD32DB93B => Some("Game::LoseIfTeamMemberLoses"),
        0xAC50D20D => Some("Game::WinOnCount"),
        0xBFCC890D => Some("Game::WinOnPoints"),
        0xD19EA218 => Some("Game::AlwaysHavePowerup"),
        0xC496BEF7 => Some("Game::AlwaysHaveWeapon"),
        0xB9B84D11 => Some("Game::RespawnAtTimes"),
        0x9C321742 => Some("Game::RespawnOnPlayerSlammed"),
        0xEDF8FB25 => Some("Game::RespawnIfPlayerDead"),
        0x984D9BFA => Some("Game::RespawnAfterDeath"),
        0xFAE4941A => Some("Game::AIDifficulty"),
        0x9356083A => Some("Game::AIRampDifficultyByRound"),
        0xE0529118 => Some("Game::AIRampDifficultyOnLives"),
        0x9D6A5031 => Some("Game::AIHateForBeingHit"),
        0xF3BFBC75 => Some("Game::AIHatePlayer"),
        0xFBB7890D => Some("Game::AIAllSettings"),
        0xB1063F4E => Some("Game::AISettings"),
        0xE6317725 => Some("Game::AIHateRule"),
        0xEF94268E => Some("Game::DieOnTrigger"),
        0xBBE4569E => Some("Game::DieOnSlam"),
        0xA9FD2CAB => Some("Game::RaceScoring"),
        0xFD9397A9 => Some("Game::ScoreUniqueSpitters"),
        0xB848AE44 => Some("Game::CountScoreThreshold"),
        0xCF7DB63E => Some("Game::PointsForStat"),
        0xA2F712DC => Some("Game::PointsForMove"),
        0x838B238E => Some("Game::PointsForHolding"),
        0xC86C019B => Some("Game::PointsForTrigger"),
        0xC23DDB26 => Some("Game::PointsForPickup"),
        0xCDBF71ED => Some("Game::PointsForSlammingPlayer"),
        0xF0DCB924 => Some("Game::PointsForMultislam"),
        0xD61C51D6 => Some("Game::ReverseSlamScoring"),
        0xB0D0C463 => Some("Game::StandardSlamScoring"),
        0xA30864D1 => Some("Game::HitLoseMass"),
        0xEE8D88D0 => Some("Game::SlamDropCandy"),
        0x8773A684 => Some("Game::HitDropCandy"),
        0xAE976571 => Some("Game::MoveFilter"),
        0xB9E2855B => Some("Game::HitGainOverTime"),
        0xE362FD5C => Some("Game::HitSlamMeterFull"),
        0xCADF8B33 => Some("Game::HitDrainPower"),
        0xB999E74C => Some("Game::HitGainPower"),
        0xB71B8368 => Some("Game::StandardHitCalculations"),
        0xA735B439 => Some("Game::WinRule"),
        0xAC357B07 => Some("Game::ScoreRule"),
        0xB13062EB => Some("Game::Ruleset"),
        0xA6FC81A0 => Some("Game::RenderSpawn"),
        0x898CE5F0 => Some("Game::CollisionBone"),
        0x9E086676 => Some("Game::RenderBase"),
        0xFACA7B72 => Some("Game::CompoundLock"),
        0xC66F1B0F => Some("Game::DebugAlwaysLock"),
        0xB327837F => Some("Game::AllTrophyLock"),
        0x877C2680 => Some("Game::AllBadgesLock"),
        0xF37E0B44 => Some("Game::CharacterPlayedLock"),
        0xF94ACEFD => Some("Game::StatLock"),
        0xC2210CDD => Some("Game::TimeLock"),
        0x99404BEF => Some("Game::SlamCountLock"),
        0xA3C52BCB => Some("Game::LadderClearLock"),
        0xE7E47AC7 => Some("Game::PointsClearedLock"),
        0xA34A56E0 => Some("Game::ClusterClearedLock"),
        0xDB95C924 => Some("Game::ModeUnLockedLock"),
        0xFD1FDE7E => Some("Game::LevelClearedLock"),
        0xEF18743E => Some("Game::Lock"),
        0x86ED912E => Some("Game::PrescribedMovement"),
        0xE277740F => Some("Game::ApplauseMovementType"),
        0x8046BF58 => Some("Game::PanicRunMovementType"),
        0x9EE64EBA => Some("Game::MagnetToPointMovementType"),
        0x9A6A14A8 => Some("Game::MagnetMovementType"),
        0xFB09DBBB => Some("Game::FrozenMovementType"),
        0xC147C572 => Some("Game::PinataMovementType"),
        0xA859DFEE => Some("Game::AttachToHavokObjMovementType"),
        0xF24D9D30 => Some("Game::GravityWandMovementType"),
        0xCF617C7E => Some("Game::PrescribedMovementType"),
        0xADDDF1EC => Some("Game::PhysicsFighting"),
        0x894E3AE9 => Some("Game::ComboSpec"),
        0x90695169 => Some("Game::BufferedMove"),
        0x9616A4A0 => Some("Game::AttackMove"),
        0xEBF07BB5 => Some("Game::AttackMoveType"),
        0xF2CFE08D => Some("Game::AttackMoveRegion"),
        0xB44FD060 => Some("Game::PhysicsModelSimplePed"),
        0xDB80AE8C => Some("Game::ContactStateModelConstrained"),
        0xD886BF1B => Some("Game::ContactStateModelAirborn"),
        0xE428884C => Some("Game::ContactStateModelGround"),
        0xAD861E63 => Some("Game::ContactStateModelBase"),
        0xEA99DF81 => Some("Game::PhysicsBase"),
        0xE51FC5B0 => Some("Game::perlin_noise_changing"),
        0xB7AA610C => Some("Game::perlin_noise3d"),
        0xE438E321 => Some("Game::perlin_noise"),
        0xB234F35A => Some("Game::perlin_noise_params"),
        0xF7B763F1 => Some("Game::LadderSetup"),
        0xA0C4CC2F => Some("Game::CinematicMode"),
        0xEC441540 => Some("Game::Mode"),
        0xAD9A2CB2 => Some("Game::PreloadData"),
        0x95172616 => Some("Game::Level"),
        0xE87813BA => Some("Game::LevelMesh"),
        0x93AE869C => Some("Game::NodeSetUp3D"),
        0xDC27C781 => Some("Game::NodeSetUp2DWithTackOn"),
        0xC1E2134A => Some("Game::NodeSetUp2D"),
        0x8340CF0F => Some("Game::NodeSetUp1D"),
        0x925430CB => Some("Game::NodeSetUp"),
        0x81FB7394 => Some("Game::SetValuePairString"),
        0x92BAC9D4 => Some("Game::SetValuePairObject"),
        0xCB08B9C2 => Some("Game::SetValuePair"),
        0xB4817F16 => Some("Game::RefObjNavNode"),
        0x9814426F => Some("Game::HUDAIOptionsNavNode"),
        0xFEF082C0 => Some("Game::SetupOptionsNavNode"),
        0x867C38FB => Some("Game::MeleeOptionsNavNode"),
        0xE052070C => Some("Game::FunctionalityNavNode"),
        0xB1FEA000 => Some("Game::InterfaceNavNode"),
        0xEB23097E => Some("Game::InterfaceNavNodeStub"),
        0xB1F1A848 => Some("Game::NodeAffectGroup"),
        0x989F638F => Some("Game::InterfaceLayout"),
        0x8D8D20AF => Some("Game::HudModeOptions"),
        0x80F28026 => Some("Game::ShellCharacterSelectMenu"),
        0x8E80ED27 => Some("Game::HUDSummaryControl"),
        0xB340E18E => Some("Game::ModelStatusIndicator"),
        0x8C22A912 => Some("Game::LockedListenerDisplay"),
        0xBE43D77C => Some("Game::InWorldMenuWrap"),
        0x925039C8 => Some("Game::ModeCompletionDisplay"),
        0xE67EDC5D => Some("Game::NavigationLock"),
        0xFA62DC5F => Some("Game::BonusDisplayShellMenu"),
        0xD3D31BCA => Some("Game::ModeClusterShellMenu"),
        0xB54AECE0 => Some("Game::NavNodeTree"),
        0xFC3D7B19 => Some("Game::InterfaceScrollMenu"),
        0x86FD461A => Some("Game::InterfaceMenu"),
        0xDE65F66B => Some("Game::SimpleKeyMapMenu"),
        0xCDC3DA2A => Some("Game::InterfaceMenuStub"),
        0xEF7ED5F8 => Some("Game::ShellUnlockableDisplay"),
        0xA030FE2E => Some("Game::LadderDisplay"),
        0xA33B4949 => Some("Game::TeamFlagDisplay"),
        0xAA7A773A => Some("Game::ShellCharacterModelDisplay"),
        0xC5C695F3 => Some("Game::ModelDisplay"),
        0xD6E5D622 => Some("Game::InWorldModelDisplay"),
        0xD1DAE777 => Some("Game::InterfaceModel"),
        0xEB380818 => Some("Game::HudModeTimer"),
        0xC68C22F6 => Some("Game::HudWinPointDisplay"),
        0xE69C6711 => Some("Game::HudSimpleModeInfoDisplay"),
        0x8658551B => Some("Game::ShellModeInfoDisplay"),
        0xA41A1CBF => Some("Game::HudSimplePreGameDisplay"),
        0xE1477CAA => Some("Game::HudWinningSoundPlayer"),
        0xCE81A051 => Some("Game::StringFlasher"),
        0xB0DDCD53 => Some("Game::HudPregameDisplay"),
        0xB89D4E88 => Some("Game::HudTeamInfoDisplay"),
        0xFB0D4BAD => Some("Game::HudCharInfoDisplay"),
        0x9A673874 => Some("Game::HoldKeyDisplay"),
        0x9B2226C1 => Some("Game::HudMedalDisplay"),
        0xEC78B39E => Some("Game::ItemScroll"),
        0xE1628175 => Some("Game::GameFunctionalInfoItem"),
        0xFD228C46 => Some("Game::GameProgressInfoItem"),
        0xB01B337A => Some("Game::HudCharInfoItem"),
        0xC86C7F68 => Some("Game::PadUnplugMessage"),
        0x9C3CE0FD => Some("Game::HudSlamUpdater"),
        0xCF4F942A => Some("Game::HudCharacterDisplay"),
        0xC23153CF => Some("Game::InterfaceDisplayObject"),
        0x86D0ABC8 => Some("Game::HudCinematic"),
        0xFCA7D96F => Some("Game::HudMelee"),
        0xA2CED9CB => Some("Game::Hud"),
        0xF32EBBA9 => Some("Game::LoadingScreen"),
        0x8871A11D => Some("Game::ModeCluster"),
        0xBBFB0554 => Some("Game::ModeLoader"),
        0xE2BB19C3 => Some("Game::PlayerEntity"),
        0xBB37EB43 => Some("Game::LevelMaxCamera"),
        0xC9B834D4 => Some("Game::LevelCamera"),
        0x80557E97 => Some("Game::GlobalMachine"),
        0xD2DD0436 => Some("Game::EventPlayEventSequence"),
        0xBD6065B6 => Some("Game::EventItemSetState"),
        0x9D678770 => Some("Game::EventPropSetPositionOrientation"),
        0xEEDE33E3 => Some("Game::EventMultiEffect"),
        0xF5773F48 => Some("Game::EventEffectOnManyObjects"),
        0xFE97C48F => Some("Game::EventEffect"),
        0xAECA0CAF => Some("Game::EventCameraFov"),
        0x90D38045 => Some("Game::EventCameraLookAtPoint"),
        0x958EB61D => Some("Game::EventCameraLookAt"),
        0xD3A9A9EC => Some("Game::EventCameraPositionPoint"),
        0xBF0B9630 => Some("Game::EventCameraPosition"),
        0x97CFF398 => Some("Game::EventCameraFinish"),
        0xBA5E750F => Some("Game::EventCameraMaxCamTarget"),
        0x8BFF848A => Some("Game::EventCameraMaxCamPosition"),
        0xD3BE5E08 => Some("Game::EventCameraMaxCam"),
        0x89D80CDE => Some("Game::EventEntityQuickSetPositionOrient"),
        0x8692ADA7 => Some("Game::EventEntityTeleport"),
        0xF45CC1B2 => Some("Game::EventEntitySetOrient"),
        0xB33BB7A2 => Some("Game::EventEntitySetPosition"),
        0xA807E1CF => Some("Game::EventEntityClearState"),
        0xEF5EDD84 => Some("Game::EventEntitySetState"),
        0x985426D7 => Some("Game::EventEntityPropDrop"),
        0xF566CFD5 => Some("Game::EventEntityPropGet"),
        0xAB2F2611 => Some("Game::EventChangeDestructibleObjectReset"),
        0x9C624EE2 => Some("Game::EventHavokObjectsReset"),
        0xDB677D68 => Some("Game::EventHavokObjectsUnhide"),
        0xEC786CEC => Some("Game::EventHavokObjectsHide"),
        0xFD5EEBD3 => Some("Game::EventEnableItemTriggers"),
        0x9251F413 => Some("Game::EventDisableItemTriggers"),
        0xFB2FDAAE => Some("Game::EventEnableTriggers"),
        0xEA393FDD => Some("Game::EventDisableTriggers"),
        0xBF77C223 => Some("Game::EventEnableForceBoxes"),
        0xAF35D0B8 => Some("Game::EventDisableForceBoxes"),
        0x8C19C6DD => Some("Game::EventObjectsAnimateUnpause"),
        0xB3C1DD97 => Some("Game::EventObjectsAnimateStop"),
        0x814BA01B => Some("Game::EventObjectsAnimatePause"),
        0xD7BF7C1A => Some("Game::EventObjectsAnimate"),
        0xE079C55E => Some("Game::EventObjectsUnhide"),
        0xF554CA7A => Some("Game::EventObjectsHide"),
        0xCF336940 => Some("Game::EventObjectsAll"),
        0x85A7CD91 => Some("Game::EventPlayWinSound"),
        0xBBAF5F9D => Some("Game::EventPlayNameSound"),
        0x88285AEF => Some("Game::EventStopSound"),
        0xD04786EE => Some("Game::EventPlaySound"),
        0xC73A0BB0 => Some("Game::EventPrintOnScreen"),
        0xD19046F4 => Some("Game::EventResetHitPointObject"),
        0xB1CDBFF2 => Some("Game::EventDisableEventSpawnItems"),
        0xC23A0700 => Some("Game::EventSetDeflectionIncrease"),
        0xFFE78054 => Some("Game::EventChangeTargetType"),
        0xDEA43CED => Some("Game::EventAddTargetObject"),
        0xBA5D2FE7 => Some("Game::EventPrintDebug"),
        0xFCBD44E9 => Some("Game::EventPlayerControl"),
        0xF0777087 => Some("Game::EventLight"),
        0xD0B41C30 => Some("Game::EventFontTex"),
        0xA0C4CCF3 => Some("Game::FontTexTemplateForEvent"),
        0xE97A532F => Some("Game::EventTexBox"),
        0xDCDB4F7B => Some("Game::EventFontBoxALT"),
        0xE33D9AD2 => Some("Game::EventFontBox"),
        0xFD7AA25C => Some("Game::EventFade"),
        0xF2A82C9C => Some("Game::EventPlayMovie"),
        0xD68DEB1F => Some("Game::EventEnableDisableItemSpawner"),
        0xD9DEB13E => Some("Game::EventModifyPower"),
        0xD1E75823 => Some("Game::EventKillPlayer"),
        0xBF14BCC9 => Some("Game::EventSpawnItemAtPlayer"),
        0xEC1ED504 => Some("Game::EventSpawnItem"),
        0xEA0AF8D0 => Some("Game::EventAIAllSettings"),
        0xE3EA7633 => Some("Game::EventAISettings"),
        0xE8FB3826 => Some("Game::EventAIHate"),
        0xBCC650F7 => Some("Game::Event"),
        0xD24634FE => Some("Game::EventSequence"),
        0xDDEC024E => Some("Game::Entity"),
        0xC43D420D => Some("Game::EffectStringReference"),
        0xA5B6016D => Some("Game::EffectManager"),
        0xA6D66BE7 => Some("Game::DynamicDataTimedEffects"),
        0xF56BAEBA => Some("Game::EffectID"),
        0xA3328A2B => Some("Game::DynamicRumbleEffectData"),
        0xE8A85BC2 => Some("Game::EffectRumbleType"),
        0xF2EAF975 => Some("Game::DynamicGlowEffectData"),
        0xAED1C71E => Some("Game::EffectGlowType"),
        0xEF73FDE0 => Some("Game::DynamicParticleParams"),
        0xA8A46E1D => Some("Game::EmitterType"),
        0x831A6FB2 => Some("Game::Trail"),
        0xE2B181D0 => Some("Game::TrailTypeB"),
        0x9CC93660 => Some("Game::TrailUniversalParameters"),
        0x9435ED21 => Some("Game::TrailBoneParameters"),
        0xABA8D22E => Some("Game::TrailType"),
        0xD56B4E12 => Some("Game::DynamicCameraShakeEffectData"),
        0xB9CFC0B1 => Some("Game::EffectCamShakeType"),
        0xD95188A4 => Some("Game::DynamicLightEffectData"),
        0xC51D687B => Some("Game::EffectLightType"),
        0xC9F7CE39 => Some("Game::DynamicColorEffectData"),
        0xBE58F418 => Some("Game::EffectFadeOpacity"),
        0xD90CF408 => Some("Game::EffectColorType"),
        0xC38D0E39 => Some("Game::DynamicSoundEffectData"),
        0xAD86077D => Some("Game::EffectSound"),
        0xC0EC833E => Some("Game::DynamicScaleEffectData"),
        0xA3B2F988 => Some("Game::EffectScale"),
        0x9C5FF9AA => Some("Game::DynamicEffectDataParams"),
        0x8A47CEC1 => Some("Game::EffectOnEffect"),
        0x8B525DE2 => Some("Game::OrbiterType"),
        0xFA4BBEA6 => Some("Game::EffectCloneMesh"),
        0xA282EE26 => Some("Game::EffectType"),
        0xFC0B6BE4 => Some("Game::z_mesh_common_class_do_not_use"),
        0xAC440797 => Some("Game::EffectBase"),
        0xC8E0C03F => Some("Game::DynamicThrowable"),
        0x9E577451 => Some("Game::Throwable"),
        0xD6EADC7A => Some("Game::ThrowableType"),
        0xCE151AE3 => Some("Game::Powerup"),
        0xBE7B44BA => Some("Game::PowerupType"),
        0xF1E8852E => Some("Game::Potion"),
        0xF05C7BD3 => Some("Game::PotionType"),
        0xD0EB4C91 => Some("Game::Weapon"),
        0xFE392AB6 => Some("Game::WeaponType"),
        0xB85D4A76 => Some("Game::Prop"),
        0x9CE2D8DC => Some("Game::PropType"),
        0xC6864C7D => Some("Game::Item"),
        0xC888B0E5 => Some("Game::ItemType"),
        0xF193ED57 => Some("Game::Projectile"),
        0x8811292E => Some("Game::ProjectileType"),
        0xF12F7B1F => Some("Game::Target"),
        0xACF81788 => Some("Game::camManager"),
        0x8F5B2D4A => Some("Game::camLevelSpec"),
        0xC8A6232B => Some("Game::camBehaviorTrackEntity"),
        0xA202922A => Some("Game::camBehaviorFourPlayer"),
        0xFF434612 => Some("Game::camBehaviorThreeAngle"),
        0x8CF16624 => Some("Game::camBehaviorThreeAngleClone"),
        0x9B6514FB => Some("Game::camBehaviorFixedCam"),
        0xB69429AD => Some("Game::camBehaviorCombatSimple"),
        0xC81F5CEF => Some("Game::camBehaviorChase"),
        0x87A280B4 => Some("Game::camBehaviorOrbit"),
        0x85DD409E => Some("Game::camBehaviorOffset"),
        0x8B03BBB2 => Some("Game::camBehaviorStatic"),
        0xB8B20936 => Some("Game::camBehaviorVert"),
        0xB922A6DE => Some("Game::camBehavior"),
        0xDDDDCA20 => Some("Game::Mark"),
        0xD306D805 => Some("Game::BehaviorFightingControlShrek"),
        0xDB77A1E2 => Some("Game::Behavior"),
        0xC7A5FA2C => Some("Game::MedalDisplay"),
        0xF04367CA => Some("Game::TrophyInitializer"),
        0x8FC1A7A3 => Some("Game::MedalRewardStat"),
        0xBA5D16BE => Some("Game::MedalRewardEvent"),
        0xA2275A8C => Some("Game::Medal"),
        0xA319D47F => Some("Game::Trophy"),
        0xC5ABEF06 => Some("Game::NavGraph"),
        0xADA7C902 => Some("Game::NavResult"),
        0xD3563F1D => Some("Game::NavArea"),
        0xA27701A8 => Some("Game::NavAreaLink"),
        0xAC0932FD => Some("Game::NavSplineLink"),
        0x80DBAFCF => Some("Game::NavLink"),
        0xE2AD9980 => Some("Game::BehaviorAIFighting"),
        0x910EDFA6 => Some("Game::PlanThread"),
        0x9B2B1A2E => Some("Game::AIParams"),
        0xFE2687CC => Some("Game::BehaviorElement"),
        0xF22F1C64 => Some("Game::MovementParams"),
        0xE523E5DC => Some("Game::ComboSpecParams"),
        0xA824A923 => Some("Game::AttackChoice"),
        0xEDF80CFE => Some("Game::AIAttackFilter"),
        0xCECA960A => Some("Game::ResponseCurve"),
        0xE2371B05 => Some("Game::StrategySet"),
        0xFC9F4683 => Some("Game::Strategy"),
        0x847B38C6 => Some("Game::FitnessFunction"),
        0xDB39AF73 => Some("Game::FitnessFunctionElement"),
        0xC3D11ABB => Some("Game::PlanElement"),
        0xE92DC70E => Some("Game::AIPerception"),
        0xD97760D6 => Some("gf::Object"),
        0xBFC7788D => Some("gf::LocalizedString"),
        0xA128E61A => Some("GF_TEMP::ScriptDB"),
        0xCA75C29D => Some("GF_TEMP::HandyObject"),
        0x9B3DDBED => Some("gf::DB"),
        0xD9BB3F0F => Some("anim::LookatData"),
        0xA3F86286 => Some("render::SplineInstance"),
        0xDEEEEA98 => Some("render::DirectionalEmitter"),
        0xDEEC048D => Some("render::DiskEmitter"),
        0xA1BE9F14 => Some("render::SphericalEmitter"),
        0xFF156AC3 => Some("render::PartSysFXEmitterDesc"),
        0x890ED3DE => Some("render::LightInstance"),
        0xFD7247FC => Some("render::FITLoader"),
        0xABBF44AB => Some("render::FITex"),
        0xA9B7911F => Some("render::TexBoxSimple"),
        0xF0D52087 => Some("render::TexBoxALT"),
        0xC33EF61E => Some("render::FontBoxALT"),
        0xDEF96960 => Some("render::FontBox"),
        0x87810BFF => Some("render::TexBox"),
        0xB8425678 => Some("render::InterfaceBox"),
        0xFD7DDE8C => Some("render::FontStringManager"),
        0xC4A179E2 => Some("render::FontString"),
        0xEF562E2E => Some("render::FontStyle"),
        0xF89C5168 => Some("render::FontManager"),
        0xA4E55832 => Some("render::FontLoader"),
        0xFD02A278 => Some("render::MaxCamera"),
        0xCBD51018 => Some("render::Camera"),
        0xC51F1BDA => Some("render::BoxInstance"),
        0xE5E23CC9 => Some("render::AuxiliaryObject"),
        0xCCEB6FFA => Some("bin::Object"),
        0xE13BE71C => Some("core::ObjCollection"),
        _ => None,
    }
}
