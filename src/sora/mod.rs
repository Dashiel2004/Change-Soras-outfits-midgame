mod main;
mod default;
mod misc;
mod sound;
mod extra;
pub fn install(){
    main::install();
    misc::install();
    extra::install();
}
