use {
    smashline::{*}
};
use crate::airborne::*;



pub fn install() {
    Agent::new("trail")
        .on_line(Main, airborne_change)
        .install();
}
