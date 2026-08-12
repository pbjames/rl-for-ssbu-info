mod constants;
mod message;
use constants::{CPU_INDEX, HOST, OPP_INDEX, REMOTE_HOST};
use std::{
    fmt::Debug,
    io::{Read, Write},
    net::{self, TcpListener, TcpStream},
    sync::LazyLock,
};

use crate::hooks::message::{Message, SituationKindInfo, StatusKindInfo};

use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
};

static mut OUTGOING: LazyLock<TcpStream> = LazyLock::new(|| {
    let stream = TcpStream::connect(REMOTE_HOST).unwrap();
    stream.set_nodelay(true);
    stream
});
static mut INCOMING: LazyLock<TcpStream> = LazyLock::new(|| {
    let listener = TcpListener::bind(HOST).unwrap();
    let (stream, _) = listener.accept().unwrap();
    stream.set_nodelay(true);
    stream
});
static mut MESSAGE_BUF: LazyLock<Message> = LazyLock::new(|| Message::default());

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(
    module_accessor: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
    // FIX: Completely inaccurate opponent data Pokemon Stadium stage - might've been fixed with category check
    let res = original!()(module_accessor);
    let category = utility::get_category(module_accessor);
    if category != BattleObjectCategory::Fighter as i32 {
        return res;
    }
    let player_slot =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let target = match player_slot {
        0 => &mut MESSAGE_BUF.opp,
        1 => &mut MESSAGE_BUF.cpu,
        _ => return res,
    };
    let vel_3f =
        KineticModule::get_sum_speed3f(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let is_attack = AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_ALL);
    let is_attack_landed =
        AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL);
    let attack_data = *AttackModule::attack_data(module_accessor, 0, false);
    target.location.x = PostureModule::pos_x(module_accessor);
    target.location.y = PostureModule::pos_y(module_accessor);
    target.location.z = PostureModule::pos_z(module_accessor);
    target.damage = DamageModule::damage(module_accessor, 0);
    target.is_shield = ShieldModule::is_shield(module_accessor, 0, 0);
    // target.shield_strength = WorkModule::get_float(
    target.situation = SituationKindInfo::from(res);
    target.status = StatusKindInfo::from(StatusModule::status_kind(module_accessor));
    target.attack.is_attack = is_attack;
    target.attack.is_landed = is_attack_landed;
    target.attack.is_grab = CatchModule::is_catch(module_accessor);
    target.attack.power = attack_data.power;
    target.attack.knockback_growth = attack_data.r_eff;
    target.attack.fixed_knockback = attack_data.r_fix;
    target.attack.bonus_knockback = attack_data.r_add;
    target.attack.bb1.x = attack_data.x;
    target.attack.bb1.y = attack_data.y;
    target.attack.bb1.z = attack_data.z;
    target.attack.bb2.x = attack_data.x2;
    target.attack.bb2.y = attack_data.y2;
    target.attack.bb2.z = attack_data.z2;
    target.grounded_ke.x = vel_3f.x;
    target.grounded_ke.y = vel_3f.y;
    target.grounded_ke.z = vel_3f.z;
    // MESSAGE_BUF.turn = player_slot;
    MESSAGE_BUF.stage = stage::get_stage_id() as usize;
    if player_slot == 1 {
        MESSAGE_BUF
            .send_state(&mut OUTGOING)
            .map_err(|err| println!("{}", err))
            .unwrap();
        OUTGOING.write_all(b"END");
        let mut buf = [0u8; 4096];
        let _ = INCOMING.read(&mut buf);
    }
    res
}

unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.module_accessor;
    let player_slot =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let target = match player_slot {
        0 => &mut MESSAGE_BUF.opp,
        1 => &mut MESSAGE_BUF.cpu,
        _ => return,
    };
    let vel_3f =
        KineticModule::get_sum_speed3f(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let is_attack = AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_ALL);
    let is_attack_landed =
        AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL);
    let attack_data = *AttackModule::attack_data(module_accessor, 0, false);
    target.location.x = PostureModule::pos_x(module_accessor);
    target.location.y = PostureModule::pos_y(module_accessor);
    target.location.z = PostureModule::pos_z(module_accessor);
    target.damage = DamageModule::damage(module_accessor, 0);
    target.is_shield = ShieldModule::is_shield(module_accessor, 0, 0);
    // target.shield_strength = WorkModule::get_float(
    target.situation = SituationKindInfo::from(StatusModule::situation_kind(module_accessor));
    target.status = StatusKindInfo::from(StatusModule::status_kind(module_accessor));
    target.attack.is_attack = is_attack;
    target.attack.is_landed = is_attack_landed;
    target.attack.is_grab = CatchModule::is_catch(module_accessor);
    target.attack.power = attack_data.power;
    target.attack.knockback_growth = attack_data.r_eff;
    target.attack.fixed_knockback = attack_data.r_fix;
    target.attack.bonus_knockback = attack_data.r_add;
    target.attack.bb1.x = attack_data.x;
    target.attack.bb1.y = attack_data.y;
    target.attack.bb1.z = attack_data.z;
    target.attack.bb2.x = attack_data.x2;
    target.attack.bb2.y = attack_data.y2;
    target.attack.bb2.z = attack_data.z2;
    target.grounded_ke.x = vel_3f.x;
    target.grounded_ke.y = vel_3f.y;
    target.grounded_ke.z = vel_3f.z;
    // MESSAGE_BUF.turn = player_slot;
    MESSAGE_BUF.stage = stage::get_stage_id() as usize;
    if player_slot == 1 {
        MESSAGE_BUF
            .send_state(&mut OUTGOING)
            .map_err(|err| println!("{}", err))
            .unwrap();
        OUTGOING.write_all(b"END");
        let mut buf = [0u8; 4096];
        let _ = INCOMING.read(&mut buf);
    }
}

pub fn install() {
    // skyline::install_hooks!(situation_kind_replace);
    Agent::new("marth").on_line(Main, fighter_frame).install();
    //.on_start(agent_init)
    //.on_end(agent_deinit);
}
