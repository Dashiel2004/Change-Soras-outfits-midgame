mod main;
mod default;
mod misc;
mod sound;
mod extra;
mod air;
pub fn install(){
    main::install();
    misc::install();
    extra::install();
    air::install();
}
