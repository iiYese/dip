mod cli;

use crate::cli::DipCliPlugin;
use dip::{
    bevy::{
        app::App,
        log::{Level, LogPlugin},
    },
    utils::DipRes,
};

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.insert_resource(DipRes(LogPlugin {
        filter: "info,dip=debug".into(),
        level: Level::DEBUG,
    }));

    #[cfg(not(debug_assertions))]
    app.insert_resource(LogSettings {
        filter: "warn".into(),
        level: Level::WARN,
    });

    app.add_plugin(DipCliPlugin)
        .add_plugin(LogPlugin::default())
        .run();
}
