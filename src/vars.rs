use std::sync::atomic::{AtomicBool, Ordering};
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{lua_bind::*},
    },
};




/// Atomic booleans to track which outfit is currently loaded
pub static OUTFIT1: AtomicBool = AtomicBool::new(false);
pub static OUTFIT2: AtomicBool = AtomicBool::new(false);
pub static OUTFIT3: AtomicBool = AtomicBool::new(false);
pub static OUTFIT4: AtomicBool = AtomicBool::new(false);
pub static OUTFIT5: AtomicBool = AtomicBool::new(false);

/// Atomic booleans to track which outfit's will be active in the win screen
pub static OUTFIT_WIN1: AtomicBool = AtomicBool::new(false);
pub static OUTFIT_WIN2: AtomicBool = AtomicBool::new(false);
pub static OUTFIT_WIN3: AtomicBool = AtomicBool::new(false);
pub static OUTFIT_WIN4: AtomicBool = AtomicBool::new(false);
pub static OUTFIT_WIN5: AtomicBool = AtomicBool::new(false);

pub static NO_WEAPON_VISIBLE: AtomicBool = AtomicBool::new(false);


/// Functions to load each outfit by setting the appropriate atomic booleans and updating model visibility
pub unsafe fn load_default(agent: &mut L2CAgentBase){
    OUTFIT1.store(true, Ordering::Relaxed);
    OUTFIT2.store(false, Ordering::Relaxed);
    OUTFIT3.store(false, Ordering::Relaxed);
    OUTFIT4.store(false, Ordering::Relaxed);
    OUTFIT5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), false);

}
pub unsafe fn load_outfit1(agent: &mut L2CAgentBase) {
    OUTFIT1.store(true, Ordering::Relaxed);
    OUTFIT2.store(false, Ordering::Relaxed);
    OUTFIT3.store(false, Ordering::Relaxed);
    OUTFIT4.store(false, Ordering::Relaxed);
    OUTFIT5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), false);
    OUTFIT_WIN1.store(true, Ordering::Relaxed);
    OUTFIT_WIN2.store(false, Ordering::Relaxed);
    OUTFIT_WIN3.store(false, Ordering::Relaxed);
    OUTFIT_WIN4.store(false, Ordering::Relaxed);
    OUTFIT_WIN5.store(false, Ordering::Relaxed);
}
pub unsafe fn load_outfit2(agent: &mut L2CAgentBase) {
    OUTFIT1.store(false, Ordering::Relaxed);
    OUTFIT2.store(true, Ordering::Relaxed);
    OUTFIT3.store(false, Ordering::Relaxed);
    OUTFIT4.store(false, Ordering::Relaxed);
    OUTFIT5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), false);
    OUTFIT_WIN1.store(false, Ordering::Relaxed);
    OUTFIT_WIN2.store(true, Ordering::Relaxed);
    OUTFIT_WIN3.store(false, Ordering::Relaxed);
    OUTFIT_WIN4.store(false, Ordering::Relaxed);
    OUTFIT_WIN5.store(false, Ordering::Relaxed);
}
pub unsafe fn load_outfit3(agent: &mut L2CAgentBase) {
    OUTFIT1.store(false, Ordering::Relaxed);
    OUTFIT2.store(false, Ordering::Relaxed);
    OUTFIT3.store(true, Ordering::Relaxed);
    OUTFIT4.store(false, Ordering::Relaxed);
    OUTFIT5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), false);
    OUTFIT_WIN1.store(false, Ordering::Relaxed);
    OUTFIT_WIN2.store(false, Ordering::Relaxed);
    OUTFIT_WIN3.store(true, Ordering::Relaxed);
    OUTFIT_WIN4.store(false, Ordering::Relaxed);
    OUTFIT_WIN5.store(false, Ordering::Relaxed);
}
pub unsafe fn load_outfit4(agent: &mut L2CAgentBase) {
    OUTFIT1.store(false, Ordering::Relaxed);
    OUTFIT2.store(false, Ordering::Relaxed);
    OUTFIT3.store(false, Ordering::Relaxed);
    OUTFIT4.store(true, Ordering::Relaxed);
    OUTFIT5.store(false, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), false);
    OUTFIT_WIN1.store(false, Ordering::Relaxed);
    OUTFIT_WIN2.store(false, Ordering::Relaxed);
    OUTFIT_WIN3.store(false, Ordering::Relaxed);
    OUTFIT_WIN4.store(true, Ordering::Relaxed);
    OUTFIT_WIN5.store(false, Ordering::Relaxed);
}
pub unsafe fn load_outfit5(agent: &mut L2CAgentBase) {
    OUTFIT1.store(false, Ordering::Relaxed);
    OUTFIT2.store(false, Ordering::Relaxed);
    OUTFIT3.store(false, Ordering::Relaxed);
    OUTFIT4.store(false, Ordering::Relaxed);
    OUTFIT5.store(true, Ordering::Relaxed);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5"), true);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), true);
    OUTFIT_WIN1.store(false, Ordering::Relaxed);
    OUTFIT_WIN2.store(false, Ordering::Relaxed);
    OUTFIT_WIN3.store(false, Ordering::Relaxed);
    OUTFIT_WIN4.store(false, Ordering::Relaxed);
    OUTFIT_WIN5.store(true, Ordering::Relaxed);
}


pub unsafe fn hide_weapon(agent: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), false);
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), false);
}

pub unsafe fn show_weapon(agent: &mut L2CAgentBase){
    if OUTFIT1.load(Ordering::Relaxed) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit1_weapon"), true);
    }
    else if OUTFIT2.load(Ordering::Relaxed) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit2_weapon"), true);
    }
    else if OUTFIT3.load(Ordering::Relaxed) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit3_weapon"), true);
    }
    else if OUTFIT4.load(Ordering::Relaxed) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit4_weapon"), true);
    }
    else if OUTFIT5.load(Ordering::Relaxed) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("outfit5_weapon"), true);
    }
}
