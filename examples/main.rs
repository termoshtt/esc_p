use esc_p::*;

fn main() {
    let mut ctl = Control::default();
    ctl.add_repeat(200, Char::all());
    print!("{}", ctl);
}
