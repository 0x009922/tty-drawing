use crate::rendering::TerminalArtist;

pub trait Tick {
    fn tick(&mut self, ms: u32);
}

pub trait Art {
    fn draw(&self, artist: &mut TerminalArtist);
}
