use serde::{Deserialize, Serialize};
use smash::lib::lua_const::{
    *
};
use std::{io::Write, net::TcpStream, ops::ShrAssign, sync::LazyLock};
use rmp_serde::Serializer;

use super::STREAM;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Message {
    pub stage: usize,
    pub cpu: FighterInfo,
    pub opp: FighterInfo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FighterInfo {
    pub location: Vector3fInfo,
    pub damage: f32,
    pub is_shield: bool,
    pub shield_strength: f32,
    pub attack: AttackInfo,
    pub grounded_ke: Vector3fInfo,
    pub situation: SituationKindInfo,
    pub status: StatusKindInfo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum SituationKindInfo {
    #[default]
    None,
    Air,
    Odd,
    Term,
    Cliff,
    Ground,
    Ladder,
    Outfield,
    Restraint,
    Water,
}

impl From<i32> for SituationKindInfo {
    fn from(x: i32) -> Self {
        let none: i32 = SITUATION_KIND_NONE.into();
        let air: i32 = SITUATION_KIND_AIR.into();
        let odd: i32 = SITUATION_KIND_ODD.into();
        let cliff: i32 = SITUATION_KIND_CLIFF.into();
        let ground: i32 = SITUATION_KIND_GROUND.into();
        let ladder: i32 = SITUATION_KIND_LADDER.into();
        let outfield: i32 = SITUATION_KIND_OUTFIELD.into();
        let restraint: i32 = SITUATION_KIND_RESTRAINT.into();
        let water: i32 = SITUATION_KIND_WATER.into();
        if x == none {
            Self::None
        } else if x == air {
            Self::Air
        } else if x == odd {
            Self::Odd
        } else if x == cliff {
            Self::Cliff
        } else if x == ground {
            Self::Ground
        } else if x == ladder {
            Self::Ladder
        } else if x == outfield {
            Self::Outfield
        } else if x == restraint {
            Self::Restraint
        } else if x == water {
            Self::Water
        } else {
            Self::None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum StatusKindInfo {
    #[default]
    Unmapped,
    Ice,
    Run,
    Bury,
    Fall,
    Dash,
    Jump,
    Lose,
    Walk,
    Grab,
    Sleep,
    CliffWait,
    CliffCatch,
    CliffClimb,
    CliffJump1,
    CliffJump2,
    CliffJump3,
    CliffAttack,
    CliffEscape,
    EscapeAir,
    Escape,
    EscapeB,
    EscapeF,
    Dead,
}

impl From<i32> for StatusKindInfo {
    fn from(x: i32) -> Self {
        let ice: i32 = FIGHTER_STATUS_KIND_ICE.into();
        let run: i32 = FIGHTER_STATUS_KIND_RUN.into();
        let bury: i32 = FIGHTER_STATUS_KIND_BURY.into();
        let dash: i32 = FIGHTER_STATUS_KIND_DASH.into();
        let dead: i32 = FIGHTER_STATUS_KIND_DEAD.into();
        let jump: i32 = FIGHTER_STATUS_KIND_JUMP.into();
        let walk: i32 = FIGHTER_STATUS_KIND_WALK.into();
        let catch: i32 = FIGHTER_STATUS_KIND_CATCH.into();
        let sleep: i32 = FIGHTER_STATUS_KIND_SLEEP.into();
        let cliff_wait: i32 = FIGHTER_STATUS_KIND_CLIFF_WAIT.into();
        let cliff_attack: i32 = FIGHTER_STATUS_KIND_CLIFF_ATTACK.into();
        let cliff_catch: i32 = FIGHTER_STATUS_KIND_CLIFF_CATCH.into();
        let cliff_climb: i32 = FIGHTER_STATUS_KIND_CLIFF_CLIMB.into();
        let cliff_jump1: i32 = FIGHTER_STATUS_KIND_CLIFF_JUMP1.into();
        let cliff_jump2: i32 = FIGHTER_STATUS_KIND_CLIFF_JUMP2.into();
        let cliff_jump3: i32 = FIGHTER_STATUS_KIND_CLIFF_JUMP3.into();
        let cliff_escape: i32 = FIGHTER_STATUS_KIND_CLIFF_ESCAPE.into();
        let escape_air: i32 = FIGHTER_STATUS_KIND_ESCAPE_AIR.into();
        let escape: i32 = FIGHTER_STATUS_KIND_ESCAPE.into();
        let escape_b: i32 = FIGHTER_STATUS_KIND_ESCAPE_B.into();
        let escape_f: i32 = FIGHTER_STATUS_KIND_ESCAPE_F.into();
        if x == ice {
            Self::Ice
        } else if x == run {
            Self::Run
        } else if x == bury {
            Self::Bury
        } else if x == dash {
            Self::Dash
        } else if x == dead {
            Self::Dead
        } else if x == jump {
            Self::Jump
        } else if x == walk {
            Self::Walk
        } else if x == catch {
            Self::Grab
        } else if x == sleep {
            Self::Sleep
        } else if x == cliff_wait {
            Self::CliffWait
        } else if x == cliff_attack {
            Self::CliffAttack
        } else if x == cliff_catch {
            Self::CliffCatch
        } else if x == cliff_climb {
            Self::CliffClimb
        } else if x == cliff_jump1 {
            Self::CliffJump1
        } else if x == cliff_jump2 {
            Self::CliffJump2
        } else if x == cliff_jump3 {
            Self::CliffJump3
        } else if x == cliff_escape {
            Self::CliffEscape
        } else if x == escape_air {
            Self::EscapeAir
        } else if x == escape {
            Self::Escape
        } else if x == escape_b {
            Self::EscapeB
        } else if x == escape_f {
            Self::EscapeF
        } else {
            Self::Unmapped
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AttackInfo {
    pub is_attack: bool,
    pub is_landed: bool,
    pub is_grab: bool,
    pub power: f32,
    pub knockback_growth: i32,
    pub fixed_knockback: i32,
    pub bonus_knockback: i32,
    pub bb1: Vector3fInfo,
    pub bb2: Vector3fInfo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Vector3fInfo {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Message {
    pub fn send_state(&mut self, stream: &mut TcpStream) -> std::io::Result<()> {
	self.serialize(&mut Serializer::new(stream)).unwrap();
        Ok(())
    }
}
