#![warn(clippy::all, clippy::pedantic)]


fn main() {
    let config_max = Some(8_u8);
    let a = if let Some(max) = config_max {
        max
    } else {
        0
    };
}
