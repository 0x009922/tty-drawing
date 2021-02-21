use std::thread;
use std::time::Duration;

pub trait Tick {
    fn tick(&mut self, ms: u64);
}

pub fn run_tick_loop(delta: u64, ticker: &mut impl Tick) {
    let ms_duration = Duration::from_millis(delta);
    // let delta_u32 = delta as u32;

    loop {
        ticker.tick(delta);
        // // чистка буфера для начала
        // artist.buffer.clear();

        // let delta_ms = TICK_MS as u32;

        // // тик и прорисовка линий
        // for line in fog_lines.iter_mut() {
        //     line.tick(delta_ms);
        //     line.draw(&mut artist);
        // }

        // // тик и прорисовка крыльев
        // for wing in wings.iter_mut() {
        //     wing.tick(delta_ms);
        //     wing.draw(&mut artist);
        // }

        // // рендеринг буффера в терминале
        // artist.render();

        // ожидание следующего тика
        // TODO замерять, сколько времени ушло на последний цикл, и спать меньше с учётом этого
        thread::sleep(ms_duration);
    }
}
