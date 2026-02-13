use {
    smash::{
        lua2cpp::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*},
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

use crate::{config::Config};
use std::sync::atomic::Ordering;

use crate::vars::*;
use crate::sora::sound::*;
use crate::effect::*;
use crate::sora::default::*;

pub unsafe extern "C" fn effect_utaunt(agent: &mut L2CAgentBase) {
    let config = Config::get();
    let color_slot = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let enabled = if config.enable_change_outfit.all_slots {
        true
    } else{
    match color_slot {
        0 => config.enable_change_outfit.c00,
        1 => config.enable_change_outfit.c01,
        2 => config.enable_change_outfit.c02,
        3 => config.enable_change_outfit.c03,
        4 => config.enable_change_outfit.c04,
        5 => config.enable_change_outfit.c05,
        6 => config.enable_change_outfit.c06,
        7 => config.enable_change_outfit.c07,
        _ => false,
    }
    };
    if enabled {
        if !OUTFIT2.load(Ordering::Relaxed){
            enter_outfit_change(agent);
            load_outfit2(agent);
            }
        else{
            exit_outfit_change(agent);
            load_outfit1(agent);
        }

        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor,*FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

        }

    }
    else{
        default_effect_utaunt(agent);
    }
}
pub unsafe extern "C" fn sound_utaunt(agent: &mut L2CAgentBase) {
    let config = Config::get();
    let color_slot = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let enabled = if config.enable_change_outfit.all_slots {
        true
    } else{
    match color_slot {
        0 => config.enable_change_outfit.c00,
        1 => config.enable_change_outfit.c01,
        2 => config.enable_change_outfit.c02,
        3 => config.enable_change_outfit.c03,
        4 => config.enable_change_outfit.c04,
        5 => config.enable_change_outfit.c05,
        6 => config.enable_change_outfit.c06,
        7 => config.enable_change_outfit.c07,
        _ => false,
    }
    };
    if enabled{
        if !OUTFIT2.load(Ordering::Relaxed){
            sound_enter_outfit_change(agent);
            frame(agent.lua_state_agent, 8.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_trail_final01"));
            }
        }
        else{
            sound_exit_outfit_change(agent);
            frame(agent.lua_state_agent, 8.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_trail_damage01"));
            }
        }

    }
    else{
        default_sound_utaunt(agent);
    }
}

pub unsafe extern  "C" fn effect_staunt(agent: &mut L2CAgentBase) {
    let config = Config::get();
    let color_slot = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let enabled = if config.enable_change_outfit.all_slots {
        true
    } else{
    match color_slot {
        0 => config.enable_change_outfit.c00,
        1 => config.enable_change_outfit.c01,
        2 => config.enable_change_outfit.c02,
        3 => config.enable_change_outfit.c03,
        4 => config.enable_change_outfit.c04,
        5 => config.enable_change_outfit.c05,
        6 => config.enable_change_outfit.c06,
        7 => config.enable_change_outfit.c07,
        _ => false,
    }
    };
    if enabled{
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
            if !OUTFIT3.load(Ordering::Relaxed){
                enter_outfit_change(agent);
                load_outfit3(agent);

            }
            
            else{
                exit_outfit_change(agent);
                load_outfit1(agent);
            }
    }
        else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L){
            if !OUTFIT5.load(Ordering::Relaxed){
                enter_outfit_change(agent);
                load_outfit5(agent);
            }
            
            else{
                exit_outfit_change(agent);
                load_outfit1(agent);

            }
        }
    }
    else{
        default_effect_staunt(agent);
        }
}

pub unsafe extern  "C" fn sound_staunt(agent: &mut L2CAgentBase) {
    let config = Config::get();
    let color_slot = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let enabled = if config.enable_change_outfit.all_slots {
        true
    } else{
    match color_slot {
        0 => config.enable_change_outfit.c00,
        1 => config.enable_change_outfit.c01,
        2 => config.enable_change_outfit.c02,
        3 => config.enable_change_outfit.c03,
        4 => config.enable_change_outfit.c04,
        5 => config.enable_change_outfit.c05,
        6 => config.enable_change_outfit.c06,
        7 => config.enable_change_outfit.c07,
        _ => false,
    }
    };
    if enabled{
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
            if !OUTFIT3.load(Ordering::Relaxed){
                sound_enter_outfit_change(agent);
                frame(agent.lua_state_agent, 8.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_trail_final01"));
                }
            }
            else{
                sound_exit_outfit_change(agent);
                frame(agent.lua_state_agent, 8.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_trail_damage01"));
                }
            }
        }
        else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L){
            if !OUTFIT5.load(Ordering::Relaxed){
                sound_enter_outfit_change(agent);
                frame(agent.lua_state_agent, 8.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_trail_final01"));
                }
            }
            else{
                sound_exit_outfit_change(agent);
                frame(agent.lua_state_agent, 8.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_trail_damage01"));
                }
            }
        }
    }
    else{
        default_sound_staunt(agent);
        }
}

pub unsafe extern  "C" fn effect_dtaunt(agent: &mut L2CAgentBase) {
    let config = Config::get();
    let color_slot = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let enabled = if config.enable_change_outfit.all_slots {
        true
    } else{
    match color_slot {
        0 => config.enable_change_outfit.c00,
        1 => config.enable_change_outfit.c01,
        2 => config.enable_change_outfit.c02,
        3 => config.enable_change_outfit.c03,
        4 => config.enable_change_outfit.c04,
        5 => config.enable_change_outfit.c05,
        6 => config.enable_change_outfit.c06,
        7 => config.enable_change_outfit.c07,
        _ => false,
    }
    };
    if enabled{
        if !OUTFIT4.load(Ordering::Relaxed) {
            enter_outfit_change(agent);
            load_outfit4(agent);
        } else {
            exit_outfit_change(agent);
            load_outfit1(agent);
        }
    } else {
        default_effect_dtaunt(agent);
    }
}

pub unsafe extern  "C" fn sound_dtaunt(agent: &mut L2CAgentBase) {
    let config = Config::get();
    let color_slot = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let enabled = if config.enable_change_outfit.all_slots {
        true
    } else{
    match color_slot {
        0 => config.enable_change_outfit.c00,
        1 => config.enable_change_outfit.c01,
        2 => config.enable_change_outfit.c02,
        3 => config.enable_change_outfit.c03,
        4 => config.enable_change_outfit.c04,
        5 => config.enable_change_outfit.c05,
        6 => config.enable_change_outfit.c06,
        7 => config.enable_change_outfit.c07,
        _ => false,
    }
    };
    if enabled{
        if !OUTFIT4.load(Ordering::Relaxed) {
            sound_enter_outfit_change(agent);
            frame(agent.lua_state_agent, 8.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_trail_final01"));
            }
        } else {
            sound_exit_outfit_change(agent);
            frame(agent.lua_state_agent, 8.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_trail_damage01"));
            }
        }
    } else {
        default_sound_dtaunt(agent);
    }
}


pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_appealhir", effect_utaunt, Default)
        .effect_acmd("effect_appealhil", effect_utaunt, Default)
        .effect_acmd("effect_appeallwl", effect_dtaunt, Default)
        .effect_acmd("effect_appeallwr", effect_dtaunt, Default)
        .effect_acmd("effect_appealsr", effect_staunt, Default)
        .effect_acmd("effect_appealsl", effect_staunt, Default)
        .sound_acmd("sound_appealhir", sound_utaunt, Default)
        .sound_acmd("sound_appealhil", sound_utaunt, Default)
        .sound_acmd("sound_appeallwl", sound_dtaunt, Default)
        .sound_acmd("sound_appeallwr", sound_dtaunt, Default)
        .sound_acmd("sound_appealsr", sound_staunt, Default)
        .sound_acmd("sound_appealsl", sound_staunt, Default)
        .install();
}
