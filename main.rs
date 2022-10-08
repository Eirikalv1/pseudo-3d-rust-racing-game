use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

const PIXEL_SIZE: i32 = 6;
const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

fn main() {
    let mut distance = 0.0;
    let mut speed = 0.0;

    let mut curvature = 0.0;
    let mut track_curvature = 0.0;
    let mut player_curvature = 0.0;
    let mut track_distance = 0.0;

    let mut vec_track: Vec<(f32, f32)> = Vec::new();

    vec_track.push((0.0, 10.0));
    vec_track.push((0.0, 200.0));
    vec_track.push((1.0, 200.0));
    vec_track.push((0.2, 300.0));
    vec_track.push((0.0, 200.0));
    vec_track.push((-0.5, 100.0));
    vec_track.push((-1.0, 100.0));
    vec_track.push((0.0, 200.0));
    vec_track.push((1.0, 200.0));
    vec_track.push((0.2, 300.0));
    vec_track.push((0.0, 200.0));
    vec_track.push((-0.5, 100.0));
    vec_track.push((-1.0, 100.0));

    for t in 0..vec_track.len() {
        track_distance += vec_track[t].1;
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut offset = 0;
        let mut track_section = 0;

        if rl.is_key_down(KEY_W) {
            speed += 2.0 * rl.get_frame_time()
        } else {
            speed -= 1.0 * rl.get_frame_time()
        }

        if rl.is_key_down(KEY_A) {
            player_curvature -= 0.7 * rl.get_frame_time();
        }

        if rl.is_key_down(KEY_D) {
            player_curvature += 0.7 * rl.get_frame_time();
        }

        if f32::abs(player_curvature - track_curvature) >= 0.8 {
            speed -= 5.0 * rl.get_frame_time();
        }

        if speed < 0.0 {
            speed = 0.0;
        }
        if speed > 1.0 {
            speed = 1.0;
        }

        distance += (70.0 * speed) * rl.get_frame_time();

        if distance >= track_distance {
            distance -= track_distance
        }

        while track_section < vec_track.len() && offset as f32 <= distance {
            offset += vec_track[track_section].1 as i32;
            track_section += 1;
        }

        let target_curavature = vec_track[track_section - 1].0;

        let track_curve_diff = (target_curavature - curvature) * rl.get_frame_time() * speed;
        curvature += track_curve_diff;

        track_curvature += curvature * rl.get_frame_time() * speed;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_rectangle(0, 0, WIDTH, HEIGHT / 4, Color::DARKBLUE);
        d.draw_rectangle(0, HEIGHT / 4, WIDTH, HEIGHT / 4, Color::BLUE);

        for x in (0..WIDTH).step_by(PIXEL_SIZE as usize) {
            let hill_height = f32::abs(
                f32::sin(x as f32 * 0.01 / PIXEL_SIZE as f32 + track_curvature)
                    * 16.0
                    * PIXEL_SIZE as f32,
            );
            for y in (((HEIGHT as f32 / 2.0 - hill_height) as i32)..(HEIGHT / 2))
                .step_by(PIXEL_SIZE as usize)
            {
                d.draw_rectangle(x as i32, y as i32, PIXEL_SIZE, PIXEL_SIZE, Color::ORANGE);
            }
        }

        for y in (0..HEIGHT / 2).step_by(PIXEL_SIZE as usize) {
            for x in (0..WIDTH).step_by(PIXEL_SIZE as usize) {
                let perspective = y as f32 / (HEIGHT / 2) as f32;

                let middle_point = 0.5 + curvature * f32::powf(1.0 - perspective, 3.0);
                let mut road_width = 0.05 + perspective as f32 * 0.8;
                let clip_width = road_width * 0.15;

                road_width *= 0.5;

                let left_grass = (middle_point - road_width - clip_width) * WIDTH as f32;
                let left_clip = (middle_point - road_width) * WIDTH as f32;
                let right_grass = (middle_point + road_width + clip_width) * WIDTH as f32;
                let right_clip = (middle_point + road_width) * WIDTH as f32;

                let row = HEIGHT / 2 + y;

                let grass_color: Color;
                if f32::sin(20.0 * f32::powf(1.0 - perspective, 3.0) + distance * 0.1) > 0.0 {
                    grass_color = Color::GREEN;
                } else {
                    grass_color = Color::DARKGREEN;
                }

                let clip_color: Color;
                if f32::sin(80.0 * f32::powf(1.0 - perspective, 2.0) + distance) > 0.0 {
                    clip_color = Color::WHITE;
                } else {
                    clip_color = Color::RED;
                }

                if (x as f32) >= 0.0 && (x as f32) < left_grass {
                    d.draw_rectangle(x, row, PIXEL_SIZE, PIXEL_SIZE, grass_color);
                }
                if (x as f32) >= left_grass && (x as f32) < left_clip {
                    d.draw_rectangle(x, row, PIXEL_SIZE, PIXEL_SIZE, clip_color);
                }
                if (x as f32) >= left_clip && (x as f32) < right_clip {
                    d.draw_rectangle(x, row, PIXEL_SIZE, PIXEL_SIZE, Color::GRAY);
                }
                if (x as f32) >= right_clip && (x as f32) < right_grass {
                    d.draw_rectangle(x, row, PIXEL_SIZE, PIXEL_SIZE, clip_color);
                }
                if (x as f32) >= right_grass && (x < WIDTH) {
                    d.draw_rectangle(x, row, PIXEL_SIZE, PIXEL_SIZE, grass_color);
                }
            }
        }

        let local_car_pos = player_curvature - track_curvature;
        let car_pos = WIDTH / 2 + ((WIDTH as f32 * local_car_pos) / 2.0) as i32 - PIXEL_SIZE * 6;

        d.draw_rectangle(
            car_pos,
            HEIGHT - 200,
            PIXEL_SIZE * 8,
            PIXEL_SIZE * 12,
            Color::PURPLE,
        );
    }
}
