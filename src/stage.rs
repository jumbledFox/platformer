use macroquad::math::Vec2;

pub struct Stage {
    tiles: Vec<u8>,
    width: usize,
}

impl Stage {
    pub fn new(tiles: Vec<u8>, width: usize) -> Stage {
        Stage { tiles, width }
    }

    pub fn tiles(&self) -> &Vec<u8> { &self.tiles }
    pub fn width(&self) -> usize    { self.width }

    pub fn tile_solid(&self, pos: Vec2) -> bool {
        let pos = pos / 16.0;
        // If outside the map, not solid
        if pos.x < 0.0 || pos.x >= self.width as f32 + 1.0 || pos.y < 0.0  || pos.y > (self.tiles.len() / self.width) as f32 + 1.0 {
            return false;
        }
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;

        let index = y * self.width + x;
        self.tiles.get(index).is_some_and(|t| *t != 0)
    }
}