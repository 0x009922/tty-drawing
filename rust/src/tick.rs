use std::thread;
use std::time::{Duration, Instant};

pub trait Tick {
    fn tick(&mut self, ms: u64);
}

pub fn run_tick_loop(tick_time: u64, ticker: &mut impl Tick) {
    // для замеров того, сколько цикл занял времени
    let now = Instant::now();

    loop {
        // замеряю время до
        let before = now.elapsed().as_millis();

        ticker.tick(tick_time);

        // получаю, сколько времени ушло на тик
        let actual_delta = (now.elapsed().as_millis() - before) as u64;

        /* Если время оказалось больше отданной мне длительности,
           то не жду совсем. Если меньше, то вычесть потраченное время
           и подождать столько, сколько ещё можно подождать. Так FPS будет
           нормализовываться в зависимости от затрат на каждый кадр
        */

        // println!()

        if actual_delta < tick_time {
            thread::sleep(Duration::from_millis(tick_time - actual_delta));
        }
    }
}
