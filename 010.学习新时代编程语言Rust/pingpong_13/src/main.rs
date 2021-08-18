use piston_window::*;
use pingpong::studio::{Racket, Ball};

fn draw(ctx: Context, gph: &mut impl Graphics, racket: &Racket, ball: &Ball) {
    clear([1.0; 4], gph);
    racket.draw(ctx, gph);
    ball.draw(ctx, gph);
}

fn main() {
    let win_width = 640;
    let win_height = 480;
    let mut window: PistonWindow =
            WindowSettings::new("Game 乒乓!", [win_width, win_height])
                    .exit_on_esc(true).build().unwrap();
    let mut racket = Racket::new();
    let mut ball = Ball::new(win_width as f64, win_height as f64);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if let Key::Left = key {
                racket.mv_left();
            }
            if let Key::Right = key {
                racket.mv_right(win_width as f64);
            }
        }
        if let Some(_) = event.render_args() {
            if ball.is_knock_top() ||
                    ball.is_knock_racket(&racket) ||
                    ball.is_knock_left() ||
                    ball.is_knock_right(win_width as f64) {
                ball.rebound();
            }
            ball.mv();
        }
        window.draw_2d(&event, |context, graphics, _| {
            draw(context, graphics, &racket, &ball);
        });
    }
}
