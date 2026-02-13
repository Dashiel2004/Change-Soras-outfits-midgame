use {
    smash::{
        lua2cpp::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*},
    },
    smashline::{*}
};
use crate::vars::*;

/// Made for online play, changes Sora's outfit based on dpad inputs
pub unsafe extern "C" fn airborne_change(agent: &mut L2CAgentBase) {
    let module_accessor = sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let situation_kind = StatusModule::situation_kind(module_accessor);    
    if situation_kind == *SITUATION_KIND_AIR {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
            load_outfit2(agent);
        }
        else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
            load_outfit3(agent);
        }
        else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
            load_outfit4(agent);
        }
        else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L){
            load_outfit5(agent);
        }
    }
}

pub fn install() {
    Agent::new("trail")
        .on_line(Main, airborne_change)
        .install();
}
