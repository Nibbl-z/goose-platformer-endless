pub struct Rect {
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32
}

impl Rect {
    pub fn collides_with(&self, other : &Rect) -> bool {
        self.x < other.x + other.w && 
        other.x < self.x + self.w && 
        self.y < other.y + other.h &&
        other.y < self.y + self.h
    }
}