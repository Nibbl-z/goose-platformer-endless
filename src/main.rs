use std::time::{SystemTime, UNIX_EPOCH};

use enemy::Enemy;
use goose_platformer_endless::player::Player;
use lava::Lava;
use macroquad::{prelude::*, rand::srand};
use goose_platformer_endless::*;
use map::Platform;

#[allow(dead_code)]
fn conf() -> Conf {
    Conf {
        window_title: String::from("Goose Platformer Endless"),
        fullscreen: false,
        ..Default::default()
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
    
    let mut direction = true;
    
    let mut fixed_timer = 0.0;
    let fixed_update_interval = 1.0 / 60.0;
    
    for i in 0..100 {
        if i == 0 {
            platforms.push(Platform::new(-100.0, 200.0, 200.0, 100.0));
        } else {
            platforms.push(platforms[i - 1].generate_next(direction));
        }
        
        if rand::gen_range(1, 10) == 5 { direction = !direction }
    }
    
    loop {   
        clear_background(WHITE);
        
        let dt = get_frame_time();
        player.update(dt);

        fixed_timer += dt;

        if fixed_timer >= fixed_update_interval {
            enemy.update(&player);
            fixed_timer -= fixed_update_interval;
        }

        lava.update(dt, &player);
        
        let camera = Camera2D {
            target: vec2(player.rect.x, player.rect.y),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            ..Default::default()
        };
        
        set_camera(&camera);
        
        for platform in platforms.iter() {
            platform.draw();
            platform.update(&mut player);
        }
        
        player.draw();
        enemy.draw();
        lava.draw();
        
        set_default_camera();
        
        let fps = (1.0 / dt).round();
        draw_text(&format!("FPS: {}", lava.y), 10.0, 20.0, 30.0, BLACK);
        
        next_frame().await;
    }
}
