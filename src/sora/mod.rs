mod main;
mod default;
mod misc;
mod sound;
mod extra;
mod airborne;
pub fn install(){
    main::install();
    misc::install();
    extra::install();
    airborne::install();
}
