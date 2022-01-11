use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn get_voxel(x: f32, y: f32, z: f32) -> (bool, u32) {
    let x = x / 10f32 * std::f32::consts::PI;
    let y = y / 10f32 * std::f32::consts::PI;

    if (x.cos() + y.sin()).floor() == z {
        return (true, 0x00FFAA00);
    }

    (false, 0)
}

fn cast_ray(angle: (f32, f32), x: f32, y: f32, z: f32) -> u32 {
    let ray_dir: Vec<f32> = vec![angle.0.sin() * angle.1.cos(), angle.0.sin() * angle.1.sin(), angle.0.cos()];
    let delta_dist: Vec<f32> = ray_dir.iter().map(|i| (1f32 / i).abs()).collect();
    let step: Vec<i32> = ray_dir.iter().map(|i| (i / i.abs()) as i32).collect();
    let mut map: Vec<i32> = vec![x, y, z].iter().map(|i| *i as i32).collect();
    let mut side_dist: Vec<f32> = vec![0.0; 3];

    'cast_loop: loop {
        let mut min_index = 0;
        for (k, v) in side_dist.iter().enumerate() {
            if delta_dist[k] != 0.0 && v < &side_dist[min_index] {
                min_index = k;
            }

            if v > &10.0 {
                break 'cast_loop 0;
            }
        }
        side_dist[min_index] += delta_dist[min_index];
        map[min_index] += step[min_index];
        let voxel = get_voxel(map[0] as f32, map[1] as f32, map[2] as f32);
        if voxel.0 {
            break voxel.1;
        }
    }
}

fn gen_buffer(width: usize, height: usize) -> Vec<u32> {
    let mut buffer = vec![0x00FFAA00; width * height];
    for y in 0..height {
        for x in 0..width {
            buffer[y * width + x] = cast_ray((
                    std::f32::consts::PI * (1.0 / 4.0 + (y as f32 / height as f32) / 2.0), 
                    std::f32::consts::PI * (1.0 / 4.0 + (x as f32 / width as f32) / 2.0)
                    ), 0.0, 0.0, 3.0);
        }
    }
    buffer
}

fn main() {
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (width, height) = window.get_size();
        let buffer: Vec<u32> = gen_buffer(width, height); 

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}
