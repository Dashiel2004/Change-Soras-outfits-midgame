use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*},
    },
    smash_script::*,
};



pub unsafe extern "C" fn sound_enter_outfit_change(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_trail_drive_enter"));
    }

}

pub unsafe extern "C" fn sound_exit_outfit_change(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_trail_drive_leave"));
    }

}
