use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::{lua_const::*},
    },
    smash_script::*,
};

/// The default effect for Sora's up taunt. (Stopga, Aeroga, Curaga)
pub unsafe extern "C" fn default_effect_utaunt(agent: &mut L2CAgentBase) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 1{
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_cure_hold"), Hash40::new("haver"), 0, 12, 0, 0, 0, 0, 1, true);
            }
            frame(agent.lua_state_agent, 24.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_cure_flower_root"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1, true);
            }
            frame(agent.lua_state_agent, 26.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
            }
            frame(agent.lua_state_agent, 30.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_cure"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, true);
            }
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 2{
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_stopga"), Hash40::new("top"), 1.3, 11.5, -0.3, 0, 0, 0, 1.03, true);
            }
            frame(agent.lua_state_agent, 20.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FLW_POS(agent, Hash40::new("trail_stopga_number"), Hash40::new("top"), 1.3, 11.5, -0.3, -5, -135, -3.5, 1, true);
            }
            else {
                if macros::is_excute(agent) {
                    macros::EFFECT_FOLLOW(agent, Hash40::new("trail_stopga"), Hash40::new("top"), -1.3, 11.5, 0.3, 0, 0, 0, 1.03, true);
                }
            }
            frame(agent.lua_state_agent, 20.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FLW_POS(agent, Hash40::new("trail_stopga_number"), Hash40::new("top"), -1.3, 11.5, 0.3, -5, 135, 3.5, 1, true);
            }
            frame(agent.lua_state_agent, 23.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_stopga_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(agent.lua_state_agent, 24.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
            }
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 3{
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_air_hold"), Hash40::new("haver"), 0, 12, 0, 0, 0, 0, 1, true);
            }
            frame(agent.lua_state_agent, 22.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_air"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, true);
            }
            frame(agent.lua_state_agent, 24.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
            }
            frame(agent.lua_state_agent, 26.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_air_shot"), Hash40::new("haver"), 0, 12, 0, 0, 0, 0, 1, true);
            }
        }

}

pub unsafe extern "C" fn default_sound_utaunt(agent: &mut L2CAgentBase) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 1{
            frame(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_h01"));
            }
            frame(agent.lua_state_agent, 19.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_h01_02"));
            }
            frame(agent.lua_state_agent, 23.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_trail_appeal_h02"));
            }
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 2{
            frame(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_h03"));
            }
            frame(agent.lua_state_agent, 23.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_trail_appeal_h01"));
            }
            frame(agent.lua_state_agent, 27.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_h03_02"));
            }
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 3{
            frame(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_h02"));
            }
            frame(agent.lua_state_agent, 23.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_trail_appeal_h03"));
            }
            frame(agent.lua_state_agent, 24.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_h02_02"));
            }
        }

}

/// The default effect for Sora's side taunt. (Keyblade Spin and Twinkle)
pub unsafe extern  "C" fn default_effect_staunt(agent: &mut L2CAgentBase) {
        frame(agent.lua_state_agent, 13.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_appeals_slash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        frame(agent.lua_state_agent, 32.0);
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_appeals_twinkle"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        frame(agent.lua_state_agent, 37.0);
            if macros::is_excute(agent) {
                macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_appeals_slash"), -1);
            }
        frame(agent.lua_state_agent, 48.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_appeals_twinkle"), false, false);
            }
}

pub unsafe extern  "C" fn default_sound_staunt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_s01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_trail_appeal_s01"));
    }
}

/// The default effect for Sora's down taunt. (Here we go!)
pub unsafe extern  "C" fn default_effect_dtaunt(agent: &mut L2CAgentBase) {
        frame(agent.lua_state_agent, 17.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            }
        frame(agent.lua_state_agent, 48.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(agent, 1.2);
            }
}

pub unsafe extern  "C" fn default_sound_dtaunt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_l01_01"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_trail_appeal_l01"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_trail_appeal_l01_02"));
    }

}
