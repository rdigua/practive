fn main() {
// let (mut f1,f2): (u32,u32);
    let mut f1: u32=1;
    let mut f2: u32=1;
//    f1 = 1;
//    f2 = 1;
    for i in 1..21 {
        print!("Number: {} {} ", f1, f2);
        if i % 2 == 0 {
            print!("\n");
        }
        f1 = f1 + f2;
        f2 = f1 + f2;
    }
}
