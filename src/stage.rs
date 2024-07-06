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

    pub fn tile_solid_pos(&self, pos: Vec2) -> bool {
        let index = match self.pos_to_index(pos) {
            Some(i) => i,
            None    => return false,
        };
        self.tiles.get(index).is_some_and(|t| *t != 0)
    }

    pub fn set_tile_pos(&mut self, tile: u8, pos: Vec2) {
        let index = match self.pos_to_index(pos) {
            Some(i) => i,
            None    => return,
        };
        if let Some(t) = self.tiles.get_mut(index) {
            *t = tile;
        }
    }

    pub fn pos_to_index(&self, pos: Vec2) -> Option<usize> {
        let pos = pos / 16.0;
        // If outside the map, do nothing
        if pos.x < 0.0 || pos.x >= self.width as f32 || pos.y < 0.0  || pos.y > (self.tiles.len() / self.width) as f32 + 1.0 {
            return None;
        }

        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        Some(y * self.width + x)
    }
}