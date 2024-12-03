use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_contents : String = fs::read_to_string("day2input.txt")?;
    let reports : Vec<_> = file_contents.split("\n").collect();
    let goodsum : i32 = 0;
    for i in reports{
        println!("{i}");
    }

    Ok(())
}
