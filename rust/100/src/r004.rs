 use std::env;
 use std::fs::File;

 fn main() -> std::io::Result<()> {
     let mut dir = env::temp_dir();
     print!("{:?}", dir); 
     dir.push("foo.txt");
     let f = File::create(dir)?;
     Ok(())
 }
