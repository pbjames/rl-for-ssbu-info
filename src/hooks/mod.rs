use std::sync::LazyLock;
use std::{
    io::Write,
    net::{self, TcpStream},
};
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
static mut stream: LazyLock<TcpStream> =
    LazyLock::new(|| TcpStream::connect("127.0.0.1:7878").unwrap());

static OPP_INDEX: usize = 0;
static OUR_INDEX: usize = 1;

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(
    module_accessor: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
    let player_slot =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if player_slot == OPP_INDEX {}
    if player_slot == OUR_INDEX {
        // On left side of stage
        // if PostureModule::pos_x(module_accessor) < 0.0 {
        //     return *SITUATION_KIND_GROUND;
        // }
        let our_x = PostureModule::pos_x(module_accessor);
        let our_y = PostureModule::pos_y(module_accessor);
        // Our damage
        // Opponent x, y
        // Opponent damage
        // Opponent shield
        // Opponent attack
        // Opponent movement (kinetics)
        // Opponent state
        // Opponent Character
        // INFO: n -> 310 signal END 310 -> n signal START
        let stage_id = stage::get_stage_id(); // INFO: one-hot encode?
        let s = format!("stage: {} cpu x: {} cpu y: {}", stage_id, our_x, our_y);
        let _ = stream.write_all(&s.into_bytes());
    }
    // let mut buffer = [0; 1024];
    // let n = stream.read(&mut buffer)?;
    // println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    // Run original code of situation_kind funct
    original!()(module_accessor)
}

pub fn install() {
    skyline::install_hooks!(situation_kind_replace);
    //Agent::new("fighter")
    //    .on_start(agent_init)
    //    .on_end(agent_deinit);
}
// // Game acmd script
// unsafe extern "C" fn example_acmd_script(agent: &mut L2CAgentBase) {}
//
// // Char opff, Global opff
// unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {}
//
// // Status script
// unsafe extern "C" fn example_status_script(fighter: &mut L2CFighterCommon) -> L2CValue {
//     0.into()
// }

// use {
//     smash::{
//         app::{lua_bind::*, sv_animcmd::*, *},
//         hash40,
//         lib::{lua_const::*, L2CAgent, L2CValue},
//         lua2cpp::*,
//         phx::*,
//     },
//     smash_script::*,
//     smashline::{Priority::*, *},
// };
//
// // Allows for variable to be tracked seperately for each player
// static mut paluPosX: [f32; 8] = [0.0; 8];
// static mut paluPosY: [f32; 8] = [0.0; 8];
// static mut paluPosZ: [f32; 8] = [0.0; 8];
//
// unsafe extern "C" fn palutena_game_appealhi(agent: &mut L2CAgentBase) {
//     // Get player number
//     let entry_id = WorkModule::get_int(
//         agent.module_accessor,
//         *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
//     ) as usize;
//     if macros::is_excute(agent) {
//         KineticModule::set_consider_ground_friction(
//             agent.module_accessor,
//             false,
//             *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN,
//         );
//         ArticleModule::generate_article(
//             agent.module_accessor,
//             *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING,
//             false,
//             -1,
//         );
//         ArticleModule::change_motion(
//             agent.module_accessor,
//             *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING,
//             Hash40::new("appeal_hi_l"),
//             false,
//             -1.0,
//         );
//         // Sets position
//         PostureModule::set_pos(
//             agent.module_accessor,
//             &Vector3f {
//                 x: paluPosX[entry_id],
//                 y: paluPosY[entry_id],
//                 z: paluPosZ[entry_id],
//             },
//         );
//     }
// }
//
// unsafe extern "C" fn palutena_game_appeallw(agent: &mut L2CAgentBase) {
//     let entry_id = WorkModule::get_int(
//         agent.module_accessor,
//         *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
//     ) as usize;
//     if macros::is_excute(agent) {
//         paluPosX[entry_id] = PostureModule::pos_x(agent.module_accessor);
//         paluPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
//         paluPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);
//     }
// }
//
// pub fn install() {
//     Agent::new("palutena")
//         .game_acmd("game_appealhil", palutena_game_appealhi, Default)
//         .game_acmd("game_appealhir", palutena_game_appealhi, Default)
//         .game_acmd("game_appeallwl", palutena_game_appeallw, Default)
//         .game_acmd("game_appeallwr", palutena_game_appeallw, Default)
//         .install();
// }
