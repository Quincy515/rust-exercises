use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
struct Tv {
    is_turn_on: bool
}

impl Tv {
    fn new() -> Tv {
        Tv { is_turn_on: false }
    }
    fn turn_on(&mut self) {
        self.is_turn_on = true;
    }
    fn turn_off(&mut self) {
        self.is_turn_on = false;
    }
}

trait TurnOnOffTv {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}

struct Boy<'a> {
    tv: &'a mut Tv
}

impl<'a> TurnOnOffTv for Boy<'a> {
    fn turn_on(&mut self) {
        self.tv.turn_on();
    }
    fn turn_off(&mut self) {
        self.tv.turn_off();
    }
}

fn main() {
    let mut tv = Tv::new();
    let mut elder = Boy{tv:&mut tv};
    /*let mut younger = Boy{tv: &mut tv};
    println!("turn on tv");

    let is_gege_first_in = true;
    if is_gege_first_in {
        elder.turn_on();
    } else {
        younger.turn_on();
    }
    println!("tv:{:?}", tv);

    println!("trun off tv");
    let is_younger_last_out = true;
    if is_younger_last_out {
        younger.turn_off();
    } else {*/
        elder.turn_off();
    //}
    println!("tv:{:?}", tv);
}
