use {
    smash::{
        lua2cpp::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*},
    },
    smash_script::*,
    smashline::{*, Priority::*}
};
use crate::vars::*;

pub unsafe extern "C" fn effect_back_throw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("trail_keyblade_fade"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        hide_weapon(agent);
        macros::EFFECT_FOLLOW(agent, Hash40::new("trail_throw_b"), Hash40::new("throw"), 0.4, 3, 0.4, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 0, -7, 180, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 11, 2.5, 8, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6, 2.5, 10, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 18, -7, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_fade"), Hash40::new("haver"), 0, 0.5, 0, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        show_weapon(agent);
    }
}



pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_throwb", effect_back_throw, Default)
        .install();
}
