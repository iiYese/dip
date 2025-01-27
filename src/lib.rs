#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub use dip_core as core;
pub use dip_utils as utils;

#[cfg(feature = "cli")]
pub use dip_cli as cli;

#[cfg(feature = "desktop")]
pub use dip_desktop as desktop;

pub use dip_bundle as bundle;

/// Web3 related plugins
pub mod web3 {
    pub use dip_device as device;
}

pub use bevy;

#[cfg(feature = "desktop")]
pub use dioxus;

pub use tokio;

///
pub mod prelude {
    pub use bevy::prelude::*;
    pub use dioxus::prelude::*;
    pub use dip_core::prelude::*;
    pub use dip_macro::{ui_action, ui_state, ConfigPlugin};
    pub use dip_utils::DipRes;

    #[cfg(feature = "cli")]
    pub use dip_cli::prelude::*;

    #[cfg(feature = "desktop")]
    pub use dip_desktop::prelude::*;
}
