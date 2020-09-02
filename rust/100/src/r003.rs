//The water flower number.
fn main() {
//    let num 
let mut h: u32;
let mut t: u32;
let mut u: u32;
    print!("The water flower number is: ");

    for num in 100..1000 {
        h = (num / 100) as u32;
        t = ((num / 10) as u32) % 10;
        u = (num % 10) as u32;
        if h * 100 + t * 10 + u == h * h * h + t * t * t + u * u * u {
            print!("{} ", num);
        }
    }
    print!("\n");
}
