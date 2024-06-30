use std::slice::Chunks;

pub struct Stage {
    tiles: Vec<u8>,
    width: usize,
}

impl Stage {
    pub fn new(tiles: Vec<u8>, width: usize) -> Stage {
        Stage { tiles, width }
    }

    pub fn tiles(&self) -> Chunks<u8> { self.tiles.chunks(self.width) }
}