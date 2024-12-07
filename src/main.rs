use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Goose Platformer Endless"),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main("Goose Platformer Endless")]
async fn main() {
    let goose = load_texture("img/player.png").await.unwrap();
    
    loop {
        clear_background(WHITE);
        
        draw_texture(&goose, 100.0, 100.0, WHITE);

        next_frame().await;
    }
}
