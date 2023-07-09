use esc_p::*;

fn main() {
    let mut ctl = Control::default();
    for i in 0..8 {
        ctl.set_color(i);
        ctl.add_repeat(20 * i as usize, Char::all());
        ctl.add_newline();
    }
    print!("{}", ctl);
}
