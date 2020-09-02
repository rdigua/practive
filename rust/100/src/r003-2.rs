//The water flower number.
fn main() {
    let keymul = |x: u32| {
        (x / 100 as u32) * (x / 100 as u32) * (x / 100 as u32)
            + ((((x / 10) as u32) % 10) as u32)
                * ((((x / 10) as u32) % 10) as u32)
                * ((((x / 10) as u32) % 10) as u32)
            + ((x % 10) as u32) * ((x % 10) as u32) * ((x % 10) as u32)
    };
    let mut mul1: u32;
    print!("The water flower number is: ");
    for num in 100..1000 {
        mul1 = keymul(num);
        if mul1 == num {
            print!("{} ", num)
        }
    }
    print!("\n");
}
