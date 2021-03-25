use anyhow::*;
use vergen::{vergen, Config, TimeZone, TimestampKind};

fn main() -> Result<()> {
    let mut config = Config::default();

    *config.build_mut().kind_mut() = TimestampKind::All;
    *config.build_mut().timezone_mut() = TimeZone::Local;

    vergen(config)
}
