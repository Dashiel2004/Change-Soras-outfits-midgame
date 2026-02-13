use {
    smash::{
        lua2cpp::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::{lua_const::*},
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

use crate::vars::*;
use std::sync::atomic::{Ordering};

/// Loads the default outfit when Sora's model is reset
pub unsafe extern "C" fn on_start(agent: &mut L2CAgentBase) {
    load_default(agent);
}
/// Loads the default outfit when Sora's entrance animation plays
pub unsafe extern "C" fn on_entry(agent: &mut L2CAgentBase) {
    load_outfit1(agent);
    hide_weapon(agent);
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        show_weapon(agent);
    }
}
/// Loads the default outfit when Sora respawns after being KO'd
pub unsafe extern "C" fn trail_opff(agent: &mut L2CAgentBase) {
    if StatusModule::status_kind(agent.module_accessor) == FIGHTER_STATUS_KIND_DEAD {
        load_outfit1(agent);
    }
}

pub unsafe extern "C" fn on_end(agent: &mut L2CAgentBase) {
    if OUTFIT_WIN1.load(Ordering::Relaxed) {
        load_outfit1(agent);
    }
    else if OUTFIT_WIN2.load(Ordering::Relaxed) {
        load_outfit2(agent);
    }
    else if OUTFIT_WIN3.load(Ordering::Relaxed) {
        load_outfit3(agent);
    }
    else if OUTFIT_WIN4.load(Ordering::Relaxed) {
        load_outfit4(agent);
    }
    else if OUTFIT_WIN5.load(Ordering::Relaxed) {
        load_outfit5(agent);
    }
    OUTFIT_WIN1.store(true, Ordering::Relaxed);
    OUTFIT_WIN2.store(false, Ordering::Relaxed);
    OUTFIT_WIN3.store(false, Ordering::Relaxed);
    OUTFIT_WIN4.store(false, Ordering::Relaxed);
    OUTFIT_WIN5.store(false, Ordering::Relaxed);

}

pub unsafe extern "C" fn on_lose(agent: &mut L2CAgentBase) {
    hide_weapon(agent);
}

pub unsafe extern "C" fn on_something(agent: &mut L2CAgentBase) {
    show_weapon(agent);
}

pub fn install() {
    Agent::new("trail")
        .game_acmd("game_win1", on_end, Default)
        .game_acmd("game_win2", on_end, Default)
        .game_acmd("game_win3", on_end, Default)
        .game_acmd("game_lose", on_lose, Default)
        .game_acmd("game_entryl", on_entry, Default)
        .game_acmd("game_entryr", on_entry, Default)
        .game_acmd("game_damagen1", on_something, Default)
        .game_acmd("game_damagen2", on_something, Default)
        .game_acmd("game_damagen3", on_something, Default)
        .game_acmd("game_wait1", on_something, Default)
        .on_start(on_start)
        .on_line(Main, trail_opff)
        .install();
}
