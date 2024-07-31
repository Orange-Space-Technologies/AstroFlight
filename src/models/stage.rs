#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Stage {
    Init,
    PadIdle,
    PoweredAscent,
    Coast,
    Apogee,
    Parachute,
    Landed,
}
