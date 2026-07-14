mod constants;
mod message;
use constants::{HOST, OPP_INDEX, OUR_INDEX};
use std::{
    io::Write,
    net::{self, TcpStream},
    sync::LazyLock,
};

use crate::hooks::message::Message;

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

static mut STREAM: LazyLock<TcpStream> = LazyLock::new(|| TcpStream::connect(HOST).unwrap());
static mut MESSAGE_BUF: LazyLock<Message> = LazyLock::new(|| Message::default());

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(
    module_accessor: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
    let player_slot =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    // BUG: Completely inaccurate opponent data Pokemon Stadium stage
    if player_slot == OPP_INDEX {
        MESSAGE_BUF.opp_x = PostureModule::pos_x(module_accessor);
        MESSAGE_BUF.opp_y = PostureModule::pos_y(module_accessor);
        MESSAGE_BUF.opp_damage = DamageModule::damage(module_accessor, 0);
        // Opponent shield
        // Opponent attack
        // Opponent movement (kinetics)
        // Opponent state
        // Opponent Character
    }
    if player_slot == OUR_INDEX {
        // INFO: n -> 310 signal END 310 -> n signal START
        MESSAGE_BUF.stage = stage::get_stage_id() as usize;
        MESSAGE_BUF.cpu_x = PostureModule::pos_x(module_accessor);
        MESSAGE_BUF.cpu_y = PostureModule::pos_y(module_accessor);
        MESSAGE_BUF.cpu_damage = DamageModule::damage(module_accessor, 0);
        MESSAGE_BUF
            .send_state(&mut STREAM)
            .map_err(|err| println!("{}", err));
    }
    original!()(module_accessor)
}

pub fn install() {
    skyline::install_hooks!(situation_kind_replace);
    //Agent::new("fighter")
    //    .on_start(agent_init)
    //    .on_end(agent_deinit);
}
