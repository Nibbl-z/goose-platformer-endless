use goose_platformer_endless::player::Player;
use macroquad::prelude::*;
use goose_platformer_endless::map::Platform;

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
    let mut player = Player::new().await;
    let mut platforms: Vec<Platform> = Vec::new();
    
    let mut direction = false;

    for i in 0..100 {
        if i == 0 {
            platforms.push(Platform::new(-100.0, 200.0, 200.0, 100.0));
        } else {
            platforms.push(platforms[i - 1].generate_next());
        }

        //if rand::gen_range(1, 4) == 1 { direction = true } else { direction = false }
    }
    
    loop {   
        clear_background(WHITE);
        
        let dt = get_frame_time();
        player.update(dt);
        
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

        set_default_camera();
        
        let fps = (1.0 / dt).round();
        draw_text(&format!("FPS: {}", fps), 10.0, 20.0, 30.0, BLACK);
        
        next_frame().await;
    }
}
