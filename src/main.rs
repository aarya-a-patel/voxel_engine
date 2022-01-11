use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use noise::{NoiseFn, Perlin};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn get_voxel(x: f32, y: f32, z: f32, perlin: &Perlin) -> (bool, u32) {
    if (perlin.get([x as f64 / 10.0, y as f64 / 10.0]) *  10.0).floor() >= z.floor() as f64 {
        return (true, (z.abs() as u32) * 0x00040000 + (x.abs() as u32) * 0x00000400
                + (y.abs() as u32) * 0x00000004);
    }

    (false, 0)
}

fn cast_ray(angle: (f32, f32), x: f32, y: f32, z: f32, perlin: &Perlin) -> u32 {
    let ray_dir: [f32; 3] = [angle.0.sin() * angle.1.cos(), angle.0.sin() * angle.1.sin(), angle.0.cos()];
    let delta_dist: [f32; 3] = ray_dir.map(|i| (1f32 / i).abs());
    let step: [i32; 3] = ray_dir.map(|i| (i / i.abs()) as i32);
    let mut map: [i32; 3] = [x, y, z].map(|i| i.floor() as i32);
    let mut side_dist: [f32; 3] = [0.0; 3];

    loop {
        let mut min_index = 0;
        let mut total_dist = 0.0;
        for (k, v) in side_dist.iter().enumerate() {
            if v < &side_dist[min_index] {
                min_index = k;
            }
            total_dist += (map[k] * map[k]) as f32;
        }
        if total_dist > 2500.0 {
            break 0;
        }
        side_dist[min_index] += delta_dist[min_index];
        map[min_index] += step[min_index];
        let voxel = get_voxel(map[0] as f32, map[1] as f32, map[2] as f32, perlin);
        if voxel.0 {
            break voxel.1;
        }
    }
}

fn gen_buffer(width: usize, height: usize, camera_angle: f32, perlin: &Perlin) -> Vec<u32> {
    let mut buffer = vec![0x00FFAA00; width * height];
    for y in 0..height {
        for x in 0..width {
            buffer[y * width + x] = cast_ray((
                    std::f32::consts::PI * (1.0 / 3.0 + (y as f32 / height as f32) / 2.0), 
                    camera_angle + std::f32::consts::PI * (1.0 / 4.0 + (x as f32 / width as f32) / 2.0)
                    ), 0.0, 0.0, 25.0, perlin);
        }
    }
    buffer
}

fn main() {
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

    let perlin = Perlin::new();
    let mut camera_angle: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (width, height) = window.get_size();
        let buffer: Vec<u32> = gen_buffer(width / 4, height / 4, camera_angle, &perlin);
        camera_angle += 0.05;

        window
            .update_with_buffer(&buffer, width / 4, height / 4)
            .unwrap();
    }
}
