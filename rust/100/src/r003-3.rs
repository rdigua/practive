//The water flower number.
fn main() { let mut h: u32; let mut t: u32; let mut u: u32; let keymul = 
|x:u32|{
     (x /100 as u32) * (x /100 as u32) * (x /100 as u32) + ((((x / 10) 
     as u32) % 10) as u32) * ((((x / 10) as u32) % 10) as u32)* ((((x / 
     10) as u32) % 10) as u32) + ((x % 10) as u32) * ((x % 10) as u32) * 
     ((x % 10) as u32)
};
let mul = |x:u32|{ 
    ((x / 100) as u32) * 100 + ((x / 10) as u32) % 10 + (x % 10) as u32
}; 
let mut mul1: u32; 
let mut key1: u32; 
print!("The water flower number is:  "); 
 for num in 100..1000 {
//        h = (num / 100) as u32; t = ((num / 10) as u32) % 10; u = (num 
//        % 10) as u32;
//		key1 = h * h * h + t * t * t + u * u * u;
        key1 = keymul(num); 
        mul1 = mul(num);
//		match mul { h * h * h + t * t * t + u * u * u => 
//     print!("{} ", num),
	if key1==mul1 { print!("{} ", num)
}
//		_ => {} ,
		
//        if h * 100 + t * 10 + u == h * h * h + t * t * t + u * u * u { 
//            print!("{} ", num);
//        }
    }
    print!("\n");
}