use std::io;

fn get_input(letter: &'static str) -> i32 {
    
    let mut return_int = -1i32;
    
    //I suspect I am doing an ugly hack to makes this work in rust
    //BUT while return_int is  -1 I just keep on asking
    //TODO: post to stack overflow to find the correct way to handle this case
    while return_int < 0 {

        println!("Please Enter {}: ", letter);

        let mut return_txt = String::new();

        let reader = io::stdin();

        reader.read_line(& mut return_txt).ok()
            .expect("Failed to read line.");

        let return_opt: Option<i32> = return_txt.trim()
            .parse::<i32>().ok();

        return_int = match return_opt {
            Some(return_int)    => return_int,
            None                => -1,
            
        };
    }

    return_int
}


fn three_n_one (i: i32, j: i32) -> i64 {
    
    let mut count = 1;

    let mut max_count = count;

    for n in i..j {
        let mut iter = n;
        count = 1;
        while iter > 1 {
            if iter % 2 == 0 {
                iter = iter/2;
                count += 1;
            } else {
                iter = (iter * 3) + 1;
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
        }
    }
    max_count
}


fn main() {
    println!("This is an implementation of 3n+1");

    println!("The problem can be found at:");

    println!("http://www.programming-challenges.com/pg.php?page=downloadproblem&probid=110101&format=html");

    let i = get_input("i");

    let j = get_input("j");

    let count = three_n_one(i, j); 

    println!("{} {} {}", i, j, count);
}


