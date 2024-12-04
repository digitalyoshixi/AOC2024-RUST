use std::fs;
use std::io;
use std::cmp;

fn dec_sum(report : Vec<i32>) -> i32{
    let mut i = 0;
    let reportlen = report.len();
    while i < reportlen - 1 {
        if report[i] <= report[i + 1] || report[i] - 3 > report[i + 1] {
            return 0;
        }
        i += 1;
    }
    return 1;
}

fn inc_sum(report : Vec<i32>) -> i32{
    let mut i = 0;
    let reportlen = report.len();
    while i < reportlen - 1 {
        if report[i] >= report[i + 1] || report[i] < report[i + 1]- 3 {
            return 0;
        }
        i += 1;
    }
    return 1;
}


fn dec_sum_list(report : Vec<i32>) -> i32 {
    let mut retsum : i32 = 0;
    for i in 0..report.len(){
        if i == 0{
            let inpvec = (&report[1..]).to_vec();
            println!("Looking at sublist: {:?}", inpvec);
            retsum += dec_sum(inpvec.clone());
            retsum += inc_sum(inpvec.clone());
        }
        else if i == report.len() - 1{
            let inpvec = (&report[..i]).to_vec();
            println!("Looking at sublist: {:?}", inpvec);
            retsum += dec_sum(inpvec.clone());
            retsum += inc_sum(inpvec.clone());
        }
        else{
            let mut inpvec : Vec<_> = vec![]; 
            for i in (&report[..i]).to_vec(){
                inpvec.push(i);
            }
            for i in (&report[i+1..]).to_vec(){
                inpvec.push(i);
            }
            println!("Looking at sublist: {:?}", inpvec);
            retsum += dec_sum(inpvec.clone());
            retsum += inc_sum(inpvec.clone());
        }
        println!("Retsum is: {}", retsum);
    }
    println!("to return: {}", cmp::min(retsum,1));
    return cmp::min(retsum, 1);
}


fn main() -> io::Result<()> {
    let file_contents : String = fs::read_to_string("day2input.txt")?;
    let reports : Vec<_> = file_contents.split("\n").collect();
    let mut goodsum : i32 = 0;
    for i in reports{
        let report : Vec<i32> = i.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        println!("-----");
        println!("{:?}", report);
        println!("-----");
        goodsum += dec_sum_list(report);
        // check if increasing gradually
        //println!("{}", goodsum);
    }
    println!("{}", goodsum);

    Ok(())
}
