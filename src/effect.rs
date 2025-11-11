use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*},
    },
    smash_script::*,
};




/// Effects that play when entering the outfit change state aka changing outfits
pub unsafe extern "C" fn enter_outfit_change(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura"), Hash40::new("bust"), 0.0, 0.0, 0.0, -5, -135, -3.5, 1.5, true);
        }


}

/// Effects that play when exiting the outfit change state aka returning to default outfit
pub unsafe extern "C" fn exit_outfit_change(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_curse"), Hash40::new("bust"), 0.0, 0.0, 0.0, -5, -135, -3.5, 1.5, true);        
    }
}
