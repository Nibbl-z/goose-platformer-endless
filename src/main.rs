use std::time::{SystemTime, UNIX_EPOCH};

use enemy::Enemy;
use goose_platformer_endless::player::Player;
use lava::Lava;
use macroquad::{prelude::*, rand::srand};
use goose_platformer_endless::*;
use map::Platform;
use interfaces::GameOver;

#[allow(dead_code)]
fn conf() -> Conf {
    Conf {
        window_title: String::from("Goose Platformer Endless"),
        fullscreen: false,
        ..Default::default()
    }
}

fn create_map(platforms : &mut Vec<Platform>) {
    let mut direction = true;

    for i in 0..1000 {
        if i == 0 {
            platforms.push(Platform::new(-100.0, 200.0, 200.0, 100.0));
        } else {
            let start_slice = if i > 10 {i - 10} else {0};

            platforms.push(platforms[i - 1].generate_next(direction, &platforms[start_slice..i - 1]));
        }
        
        if rand::gen_range(1, 7) == 5 { direction = !direction }
    }
}

#[macroquad::main("Goose Platformer Endless")]
async fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seed = since_the_epoch.as_secs() as u64;
    srand(seed);
    
    let mut player = Player::new().await;
    let mut enemy = Enemy::new().await;
    let mut platforms: Vec<Platform> = Vec::new();
    let mut lava = Lava::new().await; 
    
    create_map(&mut platforms);
    
    let mut fixed_timer = 0.0;
    let fixed_update_interval = 1.0 / 60.0;
    
    let mut game_over_interface = GameOver::init().await;
    
    
    
    loop {   
        clear_background(WHITE);
        
        let dt = get_frame_time();
        player.update(dt);

        fixed_timer += dt;

        if fixed_timer >= fixed_update_interval {
            enemy.update(&mut player);
            player.fixed_update();
            fixed_timer -= fixed_update_interval;
        }

        lava.update(dt, &mut player);
        
        let camera = Camera2D {
            target: vec2(player.rect.x, player.rect.y),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            ..Default::default()
        };
        
        set_camera(&camera);
        
        for platform in platforms.iter() {
            if platform.rect.in_camera_view(&camera) {
                platform.draw();
                platform.update(&mut player);
            }
        }
        
        player.draw();
        enemy.draw();
        lava.draw(&player);
        
        set_default_camera();

        draw_text(&format!("SCORE: {}", player.score), 10.0, 20.0, 30.0, BLACK);

        if player.died {
            if game_over_interface.update(player.died_time) == true {
                player.reset();
                enemy.reset();
                lava.y = 600.0;

                platforms.clear();
                create_map(&mut platforms);
            };
            game_over_interface.draw();
        }
        
        next_frame().await;
    }
}
