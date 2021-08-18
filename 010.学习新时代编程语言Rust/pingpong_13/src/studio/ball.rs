use rand::Rng;

pub struct Ball {
    x: f64,
    y: f64,
    r: f64,
    step_x: f64,
    step_y: f64,
}

impl Ball {
    pub fn new(area_width: f64, area_height: f64) -> Self {
        let r = 25.0;
        Self {
            x: area_width / 2.0 - r,
            y: area_height / 2.0 - r,
            r,
            step_x: 2.0,
            step_y: 2.0,
        }
    }
    pub fn draw(&self, ctx: Context, gph: &mut impl Graphics) {
        ellipse([0.0, 1.0, 0.0, 1.0], // red
                [self.x, self.y, self.r * 2.0, self.r * 2.0], // rectangle
                ctx.transform, gph);
    }
    pub fn mv(&mut self) {
        self.x += self.step_x;
        self.y += self.step_y;
    }
    fn gen_range_step(&self) -> f64 {
        rand::thread_rng().gen_range(15..=30) as f64 / 10.0
    }
    pub fn rebound(&mut self) {
        self.step_x = -self.step_x;
        self.step_y = -self.step_y;
        let rdn_step_x = self.gen_range_step();
        let max_step = 20.0;
        if self.step_x < 0.0 { // 用生成的随机数增加步长
            self.step_x = if self.step_x < -max_step {
                self.step_x + rdn_step_x
            } else {
                self.step_x - rdn_step_x
            }
        } else {
            self.step_x = if self.step_x > max_step {
                self.step_x - rdn_step_x
            } else {
                self.step_x + rdn_step_x
            }
        }

        let rdn_step_y = self.gen_range_step();
        if self.step_y < 0.0 {
            self.step_y = if self.step_y < -max_step {
                self.step_y + rdn_step_y
            } else {
                self.step_y - rdn_step_y
            }
        } else {
            self.step_y = if self.step_y > max_step {
                self.step_y - rdn_step_y
            } else {
                self.step_y + rdn_step_y
            }
        }
    }
    pub fn is_knock_top(&self) -> bool {
        if self.step_y < 0.0 {
            self.y < 0.0
        } else {
            false
        }
    }
    pub fn is_knock_bottom(&self, area_height: f64) -> bool {
        if self.step_y > 0.0 {
            self.y + self.r * 2.0 > area_height
        } else {
            false
        }
    }
    pub fn is_knock_left(&self) -> bool {
        if self.step_x < 0.0 {
            self.x < 0.0
        } else {
            false
        }
    }
    pub fn is_knock_right(&self, area_width: f64) -> bool {
        if self.step_x > 0.0 {
            self.x + self.r * 2.0 > area_width
        } else {
            false
        }
    }
    pub fn is_knock_racket(&self, racket: &Racket) -> bool {
        if self.step_y > 0.0 {
            self.x + self.r * 2.0 > racket.x &&
                    self.x + self.r < racket.x + racket.width &&
                    self.y + 2.0 * self.r >= racket.y &&
                    self.y + 2.0 * self.r >= racket.y + racket.height
        } else {
            false
        }
    }
}
