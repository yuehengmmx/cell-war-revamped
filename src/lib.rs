use bevy::{app::AppExit, prelude::*};

pub mod repetitive_code;
pub mod player;
pub mod wave;
pub mod bullet;
pub mod enemy;
pub mod base;
pub mod main_menu;
pub mod turret;
pub mod part;
pub mod pause_menu;
pub mod game_over;
pub mod hud;

pub use turret::TurretPlugin;
pub use bullet::BulletPlugin;
pub use enemy::EnemyPlugin;
pub use base::BasePlugin;
pub use main_menu::MainMenuPlugin;
pub use wave::WavePlugin;
pub use player::PlayerPlugin;
pub use pause_menu::PauseMenuPlugin;
pub use game_over::GameOverMenuPlugin;
pub use hud::HudPlugin;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

#[derive(Event)]
pub struct GameOver {
    pub time_alive: u64,
    pub base_level: i64,
}

#[derive(Resource)]
pub struct FinalScore {
    pub scores: Vec<(i64, u64)>,
}

impl Default for FinalScore {
    fn default() -> FinalScore {
        FinalScore {
            scores: Vec::new(),
        }
    }
}

#[derive(Bundle)]
pub struct Camera2dBundle {
    pub camera: Camera2d,
    pub transform: Transform,
}

pub fn create_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .init_resource::<FinalScore>()
        .init_state::<AppState>()
        .init_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_systems(OnEnter(AppState::Game), resume_simulation)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(TurretPlugin)
        .add_plugins(WavePlugin)
        .add_plugins(BasePlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(PauseMenuPlugin)
        .add_plugins(GameOverMenuPlugin)
        .add_plugins(HudPlugin)
        .add_systems(OnExit(AppState::Game), pause_simulation)
        .add_systems(
            Update,
            (
                exit_game,
                transition_to_game_state,
                transition_to_main_menu_state,
                update_final_score,
                (toggle_simulation, handle_game_over).run_if(in_state(AppState::Game)),
            ),
        );

    app
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera2d,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
    });
}

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            println!("game paused");
        }
        if *simulation_state.get() == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
            println!("game resumed");
        }
    }
}

pub fn update_final_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut final_scores: ResMut<FinalScore>,
) {
    for event in game_over_event_reader.read() {
        final_scores.scores.push((event.base_level, event.time_alive));
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write_default();
        println!("escaped");
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for i in game_over_event_reader.read() {
        println!(
            "Time alive: {} seconds | base level : {}",
            i.time_alive, i.base_level
        );
        next_app_state.set(AppState::GameOver);
        println!("Transitioning to Game Over State");
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Transitioning to Game State");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("Transitioning to Main Menu State");
        };
    }
}
