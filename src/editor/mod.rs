use avian3d::prelude::PhysicsDebugPlugin;
use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::{
    controls,
    editor_window::{EditorWindow, EditorWindowContext},
    egui, AddEditorWindow,
};

pub struct MyEditorWindow;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy_editor_pls::EditorPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(editor_controls())
        .add_editor_window::<MyEditorWindow>();
        //.add_systems(Update, is_in_editor);
    }
}

#[derive(Default)]
pub struct MyEditorWindowState {}
impl EditorWindow for MyEditorWindow {
    type State = MyEditorWindowState;
    const NAME: &'static str = "Another editor panel";

    fn ui(_world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        ui.label("Anything can go here");
    }
}

pub fn editor_controls() -> controls::EditorControls {
    let mut editor_controls = controls::EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

//pub fn is_in_editor(
//    mut editor_events: EventReader<EditorEvent>,
//    mut next_state: ResMut<NextState<AppState>>,
//) {
//    for ev in editor_events.read() {
//        next_state.set(match ev {
//            EditorEvent::Toggle { now_active } => {
//                if *now_active {
//                    AppState::Paused
//                } else {
//                    AppState::InGame
//                }
//            }
//            _ => AppState::InGame,
//        });
//    }
//}
