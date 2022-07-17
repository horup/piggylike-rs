use engine::macroquad;
use macroquad::prelude::*;
use engine::macroquad_tiled;

#[macroquad::main("BasicShapes")]
async fn main() {
    let map_json = load_string("assets/maps/test.tmj").await.unwrap();

    let tiles_tileset_json = load_string("assets/tilesets/tiles.tsj").await.unwrap();
    let entities_tileset_json = load_string("assets/tilesets/entities.tsj").await.unwrap();

    let tiles_texture = load_texture("assets/textures/tiles.png").await.unwrap();
    tiles_texture.set_filter(FilterMode::Nearest);
    let entities_texture = load_texture("assets/textures/entities.png").await.unwrap();
    entities_texture.set_filter(FilterMode::Nearest);

    let texture_map = [("../textures/tiles.png", tiles_texture.clone()), ("../textures/entities.png", entities_texture.clone())];
    let tileset_map = [("../tilesets/tiles.tsj", tiles_tileset_json.as_str()), ("../tilesets/entities.tsj", entities_tileset_json.as_str())];

    let map = macroquad_tiled::load_map(&map_json, &texture_map, &tileset_map).unwrap();
    
    

    loop {
        clear_background(WHITE);
        let s = if screen_width() < screen_height() { screen_width()} else {screen_height()};
        let dest_rect = Rect::new(0., 0., s,  s);
        let source_rect = Rect::new(0.0, 0.0, 16.0, 16.0);


        
        map.draw_tiles("tiles", dest_rect, None);
        let entities = map.layers.get("entities").unwrap();

       /* let size = 32.0;
        for y in 0..256 {
            for x in 0..256 {
                let x = x as f32;
                let y = y as f32;
                draw_texture_ex(
                    texture,
                    x * size,
                    y * size,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(size, size)),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                );
            }
        }

        draw_line(42.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS! 123", 20.0, 20.0, 30.0, DARKGRAY);
*/
        draw_text(&format!("fps:{}", get_fps()), 16.0, 16.0, 16.0, BLACK);
        next_frame().await
    }
}
