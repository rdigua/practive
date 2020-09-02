//use std::num;
// (number as f64).sqrt() as i64 + 1;
fn main() {
    let mut leap: u32 = 1;
    let mut h: u32 = 0;
    let mut k: u32;
    print!("\n");
    for m in 100..201 {
        k = sqrt(m + 1) as u32;
        //        print!("k is {} ",k);
        for i in 2..k + 2 {
            //	 for i in 2..(m+1) as f64).sqrt() as u32 +1{
            if m % i == 0 {
                leap = 0;
                //		 continue;
                break;
            }
        }
        //            println!("i is {}, m is {} leap is {} ", i, m, leap);
        if leap == 1 {
            print!("{} ", m);
            h += 1;
            if h % 10 == 0 {
                print!("\n");
            }
        }
        leap = 1;
    }
    print!("\n The total is {} \n", h);
}

fn sqrt(n: u32) -> f64 {
    let a: f64 = n as f64;
    let mut x: f64 = 1.0;
    let mut i: u32 = 0;

    while i < n {
        x = 0.5 * (x + a / x);
        i += 1;
    }

    x
}
