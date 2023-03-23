use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, window::PresentMode,
};
use langton_plugin::LangtonPlugin;

const WINDOW_HEIGHT: f32 = 1000.;
const WINDOW_WIDTH: f32 = 1000.;

fn main() {
    let mut app = App::new();

    //Window setup
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Langton's Ant".to_string(),
            resolution: (WINDOW_HEIGHT, WINDOW_WIDTH).into(),
            fit_canvas_to_parent: true,
            present_mode: PresentMode::AutoNoVsync,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugin(LogDiagnosticsPlugin::default());
    app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.add_startup_system(camera_setup);

    app.add_plugin(LangtonPlugin);

    app.run()
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
