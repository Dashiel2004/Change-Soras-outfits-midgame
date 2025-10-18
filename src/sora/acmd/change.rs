use {
    skyline::libc::c_uint, smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    }, smash_script::*, smashline::*, std::hash

};

use std::sync::atomic::{AtomicBool, Ordering};


static OUTFIT1: AtomicBool = AtomicBool::new(false);
static OUTFIT2: AtomicBool = AtomicBool::new(false);
static OUTFIT3: AtomicBool = AtomicBool::new(false);
static OUTFIT4: AtomicBool = AtomicBool::new(false);
static OUTFIT5: AtomicBool = AtomicBool::new(false);

pub unsafe fn load_outfit1(agent: &mut L2CAgentBase) {
    outfit1.store(true, Ordering::Relaxed);
    outfit2.store(false, Ordering::Relaxed);
    outfit3.store(false, Ordering::Relaxed);
    outfit4.store(false, Ordering::Relaxed);
    outfit5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);

}
pub unsafe fn load_outfit2(agent: &mut L2CAgentBase) {
    outfit1.store(false, Ordering::Relaxed);
    outfit2.store(true, Ordering::Relaxed);
    outfit3.store(false, Ordering::Relaxed);
    outfit4.store(false, Ordering::Relaxed);
    outfit5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
}
pub unsafe fn load_outfit3(agent: &mut L2CAgentBase) {
    outfit1.store(false, Ordering::Relaxed);
    outfit2.store(false, Ordering::Relaxed);
    outfit3.store(true, Ordering::Relaxed);
    outfit4.store(false, Ordering::Relaxed);
    outfit5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
}
pub unsafe fn load_outfit4(agent: &mut L2CAgentBase) {
    outfit1.store(false, Ordering::Relaxed);
    outfit2.store(false, Ordering::Relaxed);
    outfit3.store(false, Ordering::Relaxed);
    outfit4.store(true, Ordering::Relaxed);
    outfit5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
}
pub unsafe fn load_outfit5(agent: &mut L2CAgentBase) {
    outfit1.store(false, Ordering::Relaxed);
    outfit2.store(false, Ordering::Relaxed);
    outfit3.store(false, Ordering::Relaxed);
    outfit4.store(false, Ordering::Relaxed);
    outfit5.store(true, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), true);
}



unsafe extern "C" fn effect_utaunt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        if (outfit2.load(Ordering::Relaxed)) == false{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit2(agent);

        }
        else{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_curse"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit1(agent);
        }
    }
}


unsafe extern  "C" fn effect_rtaunt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        if (outfit3.load(Ordering::Relaxed)) == false{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit3(agent);

        }
        else{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_curse"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit1(agent);
        }
    }
}


unsafe extern  "C" fn effect_dtaunt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        if (outfit4.load(Ordering::Relaxed)) == false{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit4(agent);

        }
        else{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_curse"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit1(agent);
        }
    }
}


unsafe extern  "C" fn effect_ltaunt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        if (outfit5.load(Ordering::Relaxed)) == false{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit5(agent);

        }
        else{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_curse"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            load_outfit1(agent);
        }
    }
}


unsafe extern "C" fn on_start(agent: &mut L2CAgentBase) {
    load_outfit1(agent);
}

unsafe extern "C" fn on_death(agent: &mut L2CAgentBase) {
    load_outfit1(agent);
}

unsafe extern "C" fn on_entry(agent: &mut L2CAgentBase) {
    load_outfit1(agent);
}
