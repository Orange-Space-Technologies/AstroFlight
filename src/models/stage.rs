#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Stage {
    Init,
    PadIdle,
    Launch,
    Coast,
    Apogee,
    Parachute,
    Landed,
}
