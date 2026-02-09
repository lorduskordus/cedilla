// SPDX-License-Identifier: GPL-3.0

use crate::config::CedillaConfig;
use cosmic::cosmic_config;

/// Flags given to our COSMIC application to use in it's "init" function.
#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: CedillaConfig,
}

pub fn flags() -> Flags {
    let (config_handler, config) = (CedillaConfig::config_handler(), CedillaConfig::config());

    Flags {
        config_handler,
        config,
    }
}
