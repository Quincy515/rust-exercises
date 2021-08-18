
pub struct Racket {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Racket {
    pub fn new() -> Self {
        Self { x: 270.0, y: 460.0, width: 100.0, height: 20.0 }
    }
    pub fn draw(&self, ctx: Context, gph: &mut impl Graphics) {
        rectangle([0.0, 1.0, 0.0, 1.0], // red
                  [self.x, self.y, self.width, self.height], // rectangle
                  ctx.transform, gph);
    }
    fn get_step(&self) -> f64 {
        self.width / 2.0
    }
    pub fn mv_left(&mut self) {
        self.x -= self.get_step();
        if self.x < 0.0 {
            self.x = 0.0;
        }
    }
    pub fn mv_right(&mut self, area_width: f64) {
        self.x += self.get_step();
        if self.x + self.width > area_width {
            self.x = area_width - self.width;
        }
    }
}
