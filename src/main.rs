use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use noise::{NoiseFn, Perlin};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const SPACE_SIZE: u32 = 50;

fn make_coord(coord: [i32; 3]) -> usize {
    let coord = coord.map(|i| (i + SPACE_SIZE as i32 / 2) as u32);
    (coord[0] + coord[1] * SPACE_SIZE + coord[2] * SPACE_SIZE * SPACE_SIZE) as usize
}

fn cast_ray(angle: (f32, f32), x: f32, y: f32, z: f32, space: &Vec<u32>) -> u32 {
    let ray_dir: [f32; 3] = [angle.0.sin() * angle.1.cos(), angle.0.sin() * angle.1.sin(), angle.0.cos()];
    let delta_dist: [f32; 3] = ray_dir.map(|i| (1f32 / i).abs());
    let step: [i32; 3] = ray_dir.map(|i| (i / i.abs()) as i32);
    let mut map: [i32; 3] = [x, y, z].map(|i| i.floor() as i32);
    let mut side_dist: [f32; 3] = [0.0; 3];

    loop {
        let mut min_index = 0;
        let total_dist: i32 = map.iter().fold(0, |acc, i| acc + i * i);
        
        if total_dist > (SPACE_SIZE / 2 - 1).pow(2) as i32 {
            break 0;
        }

        // for (k, v) in side_dist.iter().enumerate() {
        //    if v < &side_dist[min_index] {
        //        min_index = k;
        //    }
        //    total_dist += (map[k] * map[k]) as f32;
        // }

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

        side_dist[min_index] += delta_dist[min_index];
        map[min_index] += step[min_index];

        if make_coord(map) >= space.len() {
            break 0;
        }
        let voxel = space[make_coord(map)];
        if voxel != 0 {
            break voxel; 
        }
    }
}

fn gen_buffer(width: usize, height: usize, camera_angle: f32, space: &Vec<u32>) -> Vec<u32> {
    let mut buffer = vec![0x00FFAA00; width * height];
    for y in 0..height {
        for x in 0..width {
            buffer[y * width + x] = cast_ray((
                    std::f32::consts::PI * (1.0 / 3.0 + (y as f32 / height as f32) / 2.0), 
                    camera_angle + std::f32::consts::PI * (1.0 / 4.0 + (x as f32 / width as f32) / 2.0)
                    ), 0.0, 0.0, 10.0, space);
        }
    }
    buffer
}

fn gen_space() -> Vec<u32> {
    let perlin = Perlin::new();
    let mut space: Vec<u32> = vec![0u32; SPACE_SIZE.pow(3) as usize];
    for x in -(SPACE_SIZE as i32 / 2)..(SPACE_SIZE as i32 / 2) {
        for y in -(SPACE_SIZE as i32 / 2)..(SPACE_SIZE as i32 / 2) {
            let height = (perlin.get([x as f64 / 10.0, y as f64 / 10.0]) *  10.0).floor() as i32; 
            for z in -(SPACE_SIZE as i32 / 2)..height {
                let mut color = 0x00000000;
                for i in [x, y, z] {
                    color = color << 8;
                    color += (i + SPACE_SIZE as i32 / 2) as u32 * 2;
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
        WIDTH / 4,
        HEIGHT / 4,
        WindowOptions {
            scale: Scale::X4,
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
        let buffer: Vec<u32> = gen_buffer(width / 4, height / 4, camera_angle, &space);
        camera_angle += 0.25;

        window
            .update_with_buffer(&buffer, width / 4, height / 4)
            .unwrap();
    }
}
