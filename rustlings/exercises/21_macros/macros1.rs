macro_rules! repeat_print {
    ($x: expr, $y: expr) => {
        for _ in 1..=$y{
            println!("{}", $x);
        }
    }
}

fn main() {
    // TODO: Fix the macro call.
    repeat_print!("suleman", 3);
}
