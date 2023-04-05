use std::time::Duration;

use crate::camera::*;
use crate::platform::*;
use crate::player::*;
use crate::ui::*;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

mod camera;
mod platform;
mod player;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_state::<GameState>();

    app.insert_resource(CameraMoveState::default())
        .insert_resource(Score(0))
        .insert_resource(Accumulator(None))
        .insert_resource(JumpState::default())
        .insert_resource(FallState::default())
        .insert_resource(GenerateAccumulationParticleEffectTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )))
        .insert_resource(PrepareJumpTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )))
        .insert_resource(ScoreUpQueue(Vec::new()))
        .insert_resource(AccumulationSoundController(None))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ground)
        .add_startup_system(setup_ui_images)
        .add_startup_system(setup_game_sounds)
        // Main Menu
        .add_systems((setup_main_menu,clear_player,clear_platforms,despawn_scoreboard,).in_schedule(OnEnter(GameState::MainMenu)))
        .add_systems((click_button,).in_set(OnUpdate(GameState::MainMenu)))
        .add_systems((despawn_screen::<OnMainMenuScreen>,).in_schedule(OnExit(GameState::MainMenu)))

        // Playing
        .add_systems((clear_player,
            clear_platforms,
            despawn_scoreboard,
            setup_first_platform.after(clear_platforms),
            setup_player.after(clear_player),
            setup_scoreboard.after(despawn_scoreboard),
            reset_score,
            reset_prepare_jump_timer,).in_schedule(OnEnter(GameState::Playing)))

        .add_systems((prepare_jump,
            generate_next_platform,
            move_camera,
            player_jump,
            update_scoreboard,
            animate_jump,
            animate_fall,
            animate_player_accumulation,
            animate_platform_accumulation.after(player_jump),
            spawn_score_up_effect,
            sync_score_up_effect,
            shift_score_up_effect,).in_set(OnUpdate(GameState::Playing)))

        // GameOver
        .add_systems((setup_game_over_menu,).in_schedule(OnEnter(GameState::GameOver)))
        .add_systems((click_button,).in_set(OnUpdate(GameState::GameOver)))
        .add_systems((despawn_screen::<OnGameOverMenuScreen>,).in_schedule(OnExit(GameState::GameOver)))
        .run();
}
