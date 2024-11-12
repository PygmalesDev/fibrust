use std::env;

fn fib(nums: &Vec<u16>) -> Vec<u32> {
    let mut resvec: Vec<u32> = Vec::new();
    let max_val: u16 = match nums.last() {
        Some(num) => *num,
        None => return Vec::new()
    };

    let mut result: u32 = 0;
    let mut prev: u32   = 1;
    let mut tmp;
    
    for i in 0..=max_val {
        tmp = result;
        result += prev;
        prev = tmp;

        if nums.contains(&i) {
            resvec.push(result);
        }
    }

    resvec
}

fn map_integers(strvec: Vec<String>) -> Result<Vec<u16>, ()> {
    // Parse args to int
    let mut argint: Vec<u16> = Vec::new();
    for arg in strvec {
        let ps: u16 = match arg.parse::<u16>() {
            Ok(num) => num,
            Err(_)  => {
                print_usage(arg);
                return Err(());
            }
        };
        argint.push(ps);
    }

    argint.sort();
    Ok(argint)
}

fn map_and_print(argint: Vec<u16>) {
    let mut tb: &str;
    for pair in argint.iter().zip(fib(&argint)) {
        tb = if *pair.0 < 10 {">  "} else {"> "};
        println!("{}{}: {}", tb, pair.0, pair.1);
    }
}

fn normal_mode(argstr: Vec<String>) {
    match map_integers(argstr) {
        Ok(vec) => map_and_print(vec),
        Err(_)  => return
    };
}

fn range_mode(argstr: Vec<String>) {
    if argstr.len() < 2 {
        //TODO: explain the error here
        println!("fib: missing the second range argument!");
        return
    }

    match map_integers(argstr) {
        Ok(vec) => {
            let mut nums: Vec<u16> = Vec::new();
            for i in vec[0]..=vec[1] {
                nums.push(i);
            }
            
            map_and_print(nums);
        },
        Err(_)  => return
    };


}

fn proceed_command(argstr: Vec<String>) {
    let cmd: &String = &argstr[0];
    let nums: Vec<String> = argstr[1..].to_vec();
    if nums.len() == 0 { return }

    if cmd.contains('r') {
        return range_mode(nums);
    }
}

fn main() {
    // Collect CL args
    let argstr: Vec<String> = env::args()
        .collect::<Vec<String>>()[1..].to_vec();
    if argstr.len() == 0 { return }

    match argstr[0].parse::<u16>() {
        Ok(_)  => normal_mode(argstr),
        Err(_) => proceed_command(argstr)
    }
}

fn print_usage(num: String) {
    println!("fib: '{num}' is not an integer!");
    println!("Usage: fib [OPTION]... [ARG]...");
    println!("Try 'fib --help' for more information.");
}
