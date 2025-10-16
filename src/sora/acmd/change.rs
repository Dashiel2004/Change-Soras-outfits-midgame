use {
    skyline::libc::c_uint, smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*,
        lua2cpp::*,
        phx::
    }, smash_script::*, smashline::*, std::hash

};

use std::sync::atomic::{AtomicBool, Ordering};



let outfitDefault = AtomicBool::new(false);
let outfit1 = AtomicBool::new(false);
let outfit2 = AtomicBool::new(false);
let outfit3 = AtomicBool::new(false);
let outfit4 = AtomicBool::new(false);

public unsafe fn oufitDefaultVisible(value: bool){
outfitDefault.store(true, atomic::Ordering::Relaxed);
outfit1.store(false, atomic::Ordering::Relaxed);
outfit2.store(false, atomic::Ordering::Relaxed);
outfit3.store(false, atomic::Ordering::Relaxed);
outfit4.store(false, atomic::Ordering::Relaxed);
}
