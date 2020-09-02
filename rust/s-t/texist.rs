

fn main() {
//get home_dir
//env::home_dir()
//std::env::current_exe()
//dirs::config_dir()
//dirs::executable_dir();

    let kno = path_exist("/home/jay/k");
    let kyes = path_exist("/home/jay");
    println!("no: {}, yes: {}", kno, kyes);
    let kno = file_exist("/home/jay/k");
    let kyes = file_exist("/home/jay");
    println!("fno: {}, fyes: {}", kno, kyes)
}

fn path_exist(s: &str) -> bool {
    use std::path::Path;
    Path::new(s).exists() && Path::new(s).is_dir()
}
fn file_exist(s: &str) -> bool {
    use std::path::Path;
    Path::new(s).exists() && Path::new(s).is_file()
}

fn extract_int(inputstr: String) -> i32 {
    let mut ts = String::new();
    for c in inputstr.chars() {
        match c {
            '0'..='9' => ts.push(c),
            _ => {}
        }
    }

    if ts.len() == 0 {
        return 0;
    };
    return ts.parse::<i32>().unwrap();
}

trait Strint {
    fn toint(&self) -> u32;
}

impl Strint for String {
    fn toint(&self) -> u32 {
        let st: String = self.to_string();
        let ts: String = st.chars().filter(|x| x.is_digit(10)).collect();
        if ts.len() == 0 {
            return 0;
        };
        return ts.parse::<u32>().unwrap();
    }
	time-stamp-date(&self) - > date {
	
	}
	
}

//config = "0.10.1" 
//pkg-config = "0.3.17"
//directories = "2.0.2"
