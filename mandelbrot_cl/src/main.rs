use termion::terminal_size;
use mandelbrot_calculator::MandelbrotCalculator;

fn main() {
    let (w, h) = if let Ok((w, h)) = terminal_size() {
        (w, h)
    } else {
        panic!("Couldn't get dimensions");
    };
    let calc = MandelbrotCalculator::new(w.into(), h.into(), 100);
    for i in 0..h {
        for j in 0..w {
            if calc.pos_part_of_set(j.into(), i.into()) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}