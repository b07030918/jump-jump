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
        .insert_resource(PrepareJumpTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )))
        .insert_resource(ScoreUpQueue(Vec::new()))
        .insert_resource(AccumulationSoundController(None))
        
        //摄像机
        .add_startup_system(setup_camera)
        //地面
        .add_startup_system(setup_ground)
        //图
        .add_startup_system(setup_ui_images)
        //音乐
        .add_startup_system(setup_game_sounds)

        // Main Menu(开始页面)
        .add_systems((
            //渲染开始界面
            setup_main_menu,
            //清玩家（小柱子）
            clear_player,
            //清平台（地下方块）
            clear_platforms,
            //清分数
            despawn_scoreboard,).in_schedule(OnEnter(GameState::MainMenu)))
            
        //点击开始按钮
        .add_systems((click_button,).in_set(OnUpdate(GameState::MainMenu)))

        //清开始界面
        .add_systems((despawn_screen::<OnMainMenuScreen>,).in_schedule(OnExit(GameState::MainMenu)))

        // Playing（游戏页面）
        //进入
        .add_systems((
            //清理游戏元素（人、台、分）
            clear_player,
            clear_platforms,
            despawn_scoreboard,
            //生成台子
            setup_first_platform.after(clear_platforms),
            //生成人
            setup_player.after(clear_player),
            //生成分
            setup_scoreboard.after(despawn_scoreboard),
            //清分
            reset_score,
            reset_prepare_jump_timer,
        ).in_schedule(OnEnter(GameState::Playing)))

        //玩游戏
        .add_systems((
            prepare_jump,
            //生成下个平台坐标
            generate_next_platform,
            //移动镜头
            move_camera,
            //跳跃
            player_jump,
            //更新分数
            update_scoreboard,
            animate_jump,
            animate_fall,
            animate_player_accumulation,
            animate_platform_accumulation.after(player_jump),
            spawn_score_up_effect,
            sync_score_up_effect,
            shift_score_up_effect,
        ).in_set(OnUpdate(GameState::Playing)))

        // GameOver （结束页面）
        .add_systems((setup_game_over_menu,).in_schedule(OnEnter(GameState::GameOver)))
        // 点击结束或者返回开始界面
        .add_systems((click_button,).in_set(OnUpdate(GameState::GameOver)))
        // 清结束页面
        .add_systems((despawn_screen::<OnGameOverMenuScreen>,).in_schedule(OnExit(GameState::GameOver)))
        .run();
}
