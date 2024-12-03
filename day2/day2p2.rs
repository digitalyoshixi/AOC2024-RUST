use std::fs;
use std::io;

fn dec_sum(report: &Vec<i32>) -> i32{
    let mut i = 0;
    let mut errs = 0;
    let reportlen = report.len();
    while i < reportlen - 1 {
        if report[i] <= report[i + 1] || report[i] - 3 > report[i + 1] {
            errs+=1;
            if errs == 2{
                return 0;
            }
            if i + 2 < report.len(){
                if report[i] <= report[i + 2] || report[i] - 3 > report[i + 2] {
                    return 0;
                }
            }
            i+=2;
        }
        i += 1;
    }
    return 1;
}

fn inc_sum(report: &Vec<i32>) -> i32{
    let mut i = 0;
    let mut errs = 0;
    let reportlen = report.len();
    while i < reportlen - 1 {
        if report[i] >= report[i + 1] || report[i] < report[i + 1]- 3 {
            errs+=1;
            if errs == 2{
                return 0;
            }
            if i + 2 < report.len(){
                if report[i] >= report[i + 2] || report[i] < report[i + 2] - 3 {
                    return 0;
                }
            }
            i+=2;
        }
        i += 1;
    }
    return 1;
}


fn main() -> io::Result<()> {
    let file_contents : String = fs::read_to_string("day2input.txt")?;
    let reports : Vec<_> = file_contents.split("\n").collect();
    let mut goodsum : i32 = 0;
    for i in reports{
        let report : Vec<i32> = i.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        //println!("{:?}", report);
        if report[0] > report[1]{
            let rval = dec_sum(&report);
            goodsum += rval;
        }
        else{
            let rval = inc_sum(&report);
            goodsum += rval;
        }
        
            // check if increasing gradually
        println!("{}", goodsum);
    }

    Ok(())
}
