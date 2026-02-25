use {
    smash::{
        lua2cpp::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*},
    },
};
use crate::vars::*;
use std::sync::atomic::Ordering;

pub unsafe extern "C" fn airborne_change(agent: &mut L2CAgentBase) {
    let module_accessor = sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let situation_kind = StatusModule::situation_kind(module_accessor);    
    if situation_kind == *SITUATION_KIND_AIR {
        if ControlModule::check_button_on_trriger(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
            if !OUTFIT2.load(Ordering::Relaxed){
                load_outfit2(agent);
            }
            else {
                load_outfit1(agent);
            }
        }
        else if ControlModule::check_button_on_trriger(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
            if !OUTFIT3.load(Ordering::Relaxed){
                load_outfit3(agent);
            }
            else {
                load_outfit1(agent);
            }
        }
        else if ControlModule::check_button_on_trriger(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
            if !OUTFIT4.load(Ordering::Relaxed){
                load_outfit4(agent);
            }
            else {
                load_outfit1(agent);
            }
        }
        else if ControlModule::check_button_on_trriger(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L){
            if !OUTFIT5.load(Ordering::Relaxed){
                load_outfit5(agent);
            }
            else {
                load_outfit1(agent);
            }
        }
    }
}

pub unsafe extern "C" fn global_opff(agent: &mut L2CAgentBase) {
    let module_accessor = sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let status_kind = StatusModule::status_kind(module_accessor); 
    if status_kind == FIGHTER_STATUS_KIND_DEAD {
        load_outfit1(agent);
    }  
}