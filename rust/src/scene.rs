use rand::prelude::ThreadRng;

use crate::components::{fog::FogLine, radiation_wing::RadiationWing};
use crate::core::{Art, Tick};
use crate::rendering::*;

use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

const TICK_MS: u64 = 50;

pub fn run() {
    // тред для генерации рандомного тумана
    let mut rng = rand::thread_rng();

    let mut artist = TerminalArtist::new();

    // линии
    let mut fog_lines = generate_fog_lines(&mut rng, &artist.resolution);

    // крылья
    let mut wings: [RadiationWing; 3] = [
        RadiationWing::new(rad_wing_start_angle(0.0), &artist.resolution),
        RadiationWing::new(rad_wing_start_angle(1.0), &artist.resolution),
        RadiationWing::new(rad_wing_start_angle(2.0), &artist.resolution),
    ];

    let ms_duration = Duration::from_millis(TICK_MS);

    loop {
        // чистка буфера для начала
        artist.buffer.clear();

        let delta_ms = TICK_MS as u32;

        // тик и прорисовка линий
        for line in fog_lines.iter_mut() {
            line.tick(delta_ms);
            line.draw(&mut artist);
        }

        // тик и прорисовка крыльев
        for wing in wings.iter_mut() {
            wing.tick(delta_ms);
            wing.draw(&mut artist);
        }

        // рендеринг буффера в терминале
        artist.render();

        // ожидание следующего тика
        // TODO замерять, сколько времени ушло на последний цикл, и спать меньше с учётом этого
        thread::sleep(ms_duration);
    }
}

fn rad_wing_start_angle(index: f32) -> f32 {
    index * 2.0 * PI / 3.0
}

/**
 * Генерация линий тумана
 */
fn generate_fog_lines(rng: &mut ThreadRng, res: &TerminalResolution) -> Vec<FogLine> {
    const COUNT: u32 = 50;

    let (rows, cols) = res.get_rows_cols();
    (0..COUNT)
        .map(|_| FogLine::new_random(rows, cols, rng))
        .collect()
}
