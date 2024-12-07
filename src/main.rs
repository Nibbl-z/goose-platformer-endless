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
    
    platforms.push(Platform::new(200.0, 250.0, 300.0, 150.0));
    platforms.push(Platform::new(600.0, 150.0, 300.0, 250.0));
    platforms.push(Platform::new(400.0, 500.0, 200.0, 50.0));
    platforms.push(Platform::new(0.0, 500.0, 700.0, 10.0));
    loop {
        clear_background(WHITE);
        
        let dt = get_frame_time();
        player.update(dt);
        
        /*let camera = Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(1.0 / screen_width(), 1.0 / screen_height()),
            ..Default::default()
        };*/

        //set_camera(&camera);
        
        

        for platform in platforms.iter() {
            platform.draw();
            platform.update(&mut player);
        }

        player.draw();

        //set_default_camera();
        
        
        next_frame().await;
    }
}
