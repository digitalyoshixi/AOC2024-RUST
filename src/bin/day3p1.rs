use std::fs;
use regex::Regex;
use std::io;

fn mul(num1 : i32, num2 : i32) -> i32{
    return num1 * num2;
}


fn main() -> io::Result<()> {
    let file_contents = fs::read_to_string("day3input.txt")?;
    println!("info.txt content =\n{file_contents}");
    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    //let lines : Vec<_> = file_contents.split("\n").collect();
    let mut sum : i32 = 0;
    let matches: Vec<&str> = re.find_iter(&file_contents).map(|m| m.as_str()).collect();
    println!("{:?}", matches);
    for matched in matches{
        // second regex for finding numbers
        let numre = Regex::new(r"\d*").unwrap();
        let mut nummatches: Vec<&str> = numre.find_iter(&matched).map(|m| m.as_str()).collect();
        nummatches.retain(|&x| !x.is_empty()); // remove "" from list
        let i32_list: Vec<i32> = nummatches.iter().map(|&x| x.parse().unwrap()).collect();        // convert to numerical list
        println!("{:?}", i32_list);
        sum += mul(i32_list[0], i32_list[1]);
        
    }
    println!("sum is {}", sum);
    Ok(())
}

