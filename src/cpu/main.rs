use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use noise::{NoiseFn, Perlin};
use std::time::{Instant, Duration};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const SPACE_SIZE: u32 = 6;
const MAX_DIST: i32 = (((1 << SPACE_SIZE - 1) - 1) as i32).pow(2);

fn make_coord(coord: [i32; 3]) -> usize {
    coord.map(|i| (i + (1u32 << (SPACE_SIZE - 1)) as i32 - 1) as u32).iter().fold(0, |acc, i| (acc << SPACE_SIZE) + i) as usize
}

fn cast_ray(angle: (f32, f32), x: f32, y: f32, z: f32, space: &Vec<u32>) -> u32 {
    let ray_dir: [f32; 3] = [angle.0.sin() * angle.1.cos(), angle.0.sin() * angle.1.sin(), angle.0.cos()];
    let delta_dist: [f32; 3] = ray_dir.map(|i| (1f32 / i).abs());
    let step: [i32; 3] = ray_dir.map(|i| (i > 0.0) as i32 - (i < 0.0) as i32);
    let mut map: [i32; 3] = [x, y, z].map(|i| i as i32);
    let mut side_dist: [f32; 3] = [0.0; 3];
    let mut total_dist: i32;

    loop {
        total_dist = map.iter().fold(0, |acc, i| acc + i * i);

        if total_dist > MAX_DIST {
            break 0;
        }

        let coord = make_coord(map);

        let voxel = space[coord];
        if voxel != 0 {
            break voxel; 
        }

        let mut min_index = 0;
        if side_dist[0] < side_dist[1] {
            if side_dist[0] > side_dist[2] {
                min_index = 2;
            }
        } else {
            if side_dist[1] < side_dist[2] {
                min_index = 1;
            } else {
                min_index = 2;
            }
        }

        map[min_index] += step[min_index];

        side_dist[min_index] += delta_dist[min_index];
    }
}

fn gen_buffer(width: usize, height: usize, camera_angle: f32, space: &Vec<u32>) -> Vec<u32> {
    let mut buffer = vec![0x00FFAA00; width * height];
    let start_phi: f32 = std::f32::consts::PI / 3.0;
    let start_theta: f32 = std::f32::consts::PI / 4.0 + camera_angle;
    let y_to_phi: f32 = std::f32::consts::PI / (height as f32 * 2.0);
    let x_to_theta: f32 = std::f32::consts::PI  / (width as f32 * 2.0);
    let start = Instant::now();
    for y in 0..height {
        for x in 0..width {
            buffer[y * width + x] = cast_ray((
                    y as f32 * y_to_phi + start_phi, x as f32 * x_to_theta + start_theta
                    ), 0.0, 0.0, 10.0, space);
        }
    }
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    buffer
}

fn gen_space() -> Vec<u32> {
    let perlin = Perlin::new();
    let mut space: Vec<u32> = vec![0u32; 1usize << (SPACE_SIZE * 3)];
    let half_space: i32 = (1i32 << (SPACE_SIZE - 1)) - 1;
    for x in -half_space..half_space {
        for y in -half_space..half_space {
            let height = (perlin.get([x as f64 / 32.0, y as f64 / 32.0]) *  10.0).floor() as i32;
            for z in -half_space..height {
                let mut color = 0x00000000;
                for i in [x, y, z] {
                    color = color << 8;
                    color += (i + half_space) as u32 * 2;
                }
                space[make_coord([x, y, z])] = color;
            }
        }
    }
    space
}

fn main() {
    let space = gen_space();

    let mut window = match Window::new(
        "Voxel Render",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
        ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let mut camera_angle: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (width, height) = window.get_size();
        let buffer: Vec<u32> = gen_buffer(width, height, camera_angle, &space);
        camera_angle += 0.25;

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}
