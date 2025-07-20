mod framebuffer;
mod game_of_life;
mod patterns;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use patterns::create_sun_pattern;
use raylib::prelude::*;
use gif::{Encoder, Frame, Repeat};
use std::fs::File;


fn main() {
    let window_width = 800;
    let window_height = 600;
    
    let game_width = 80;
    let game_height = 60;
    
    let scale_x = window_width as f32 / game_width as f32;
    let scale_y = window_height as f32 / game_height as f32;
    let scale = scale_x.min(scale_y);

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(game_width, game_height);
    let mut game = GameOfLife::new(game_width, game_height);

    setup_sun_pattern(&mut game);

    let mut frame_counter = 0;
    let update_rate = 8;

    let gif_width = game_width as u16;
    let gif_height = game_height as u16;
    let color_map = &[0, 0, 0, 255, 255, 255];
    let mut gif_file = File::create("conway_sun.gif").expect("Error");
    let mut encoder = Encoder::new(&mut gif_file, gif_width, gif_height, color_map)
        .expect("Error");
    encoder.set_repeat(Repeat::Infinite).expect("Error");
    
    let mut gif_frames = 0;
    let max_gif_frames = 300; 
    let mut recording = true; 

    while !window.window_should_close() && (recording || gif_frames < max_gif_frames) {
        if frame_counter % update_rate == 0 {
            game.update();
            
            if recording && gif_frames < max_gif_frames {
                let gif_frame_data = create_gif_frame(&game, gif_width, gif_height);
                let frame = Frame::from_indexed_pixels(gif_width, gif_height, &gif_frame_data, None);
                encoder.write_frame(&frame).expect("Error");
                gif_frames += 1;
                
                
                if gif_frames >= max_gif_frames {
                    recording = false;
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    break;
                }
            }
        }

        framebuffer.set_background_color(Color::BLACK);
        game.render(&mut framebuffer);
        framebuffer.swap_buffers(&mut window, &raylib_thread, scale);

        frame_counter += 1;
    }

    drop(encoder);
    drop(gif_file);
}

fn setup_sun_pattern(game: &mut GameOfLife) {
    let (pattern, x, y) = create_sun_pattern();
    game.initialize_with_pattern(pattern, x, y);
}

fn create_gif_frame(game: &GameOfLife, width: u16, height: u16) -> Vec<u8> {
    let mut frame_data = Vec::with_capacity((width * height) as usize);

    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let pixel = if game.is_alive(x, y) { 1 } else { 0 };
            frame_data.push(pixel);
        }
    }
    frame_data
}