// https://users.rust-lang.org/t/should-simple-enum-derive-copy-trait/11483/2
#[derive(Copy, Clone, Debug)]
pub(crate) enum Mode {
    Never,
    Auto,
    Always,
}

pub(crate) fn read_mode_from_env() -> crate::Result<Mode> {
    if crate::env::get("CLICOLOR_FORCE")?.map_or(false, |value| value != "0") {
        // CLICOLOR_FORCE is present and non-zero
        return Ok(Mode::Always);
    }
    if crate::env::get("CLICOLOR")?.map_or(false, |value| value == "0") {
        // CLICOLOR is present and zero
        return Ok(Mode::Never);
    }
    Ok(Mode::Auto)
}
