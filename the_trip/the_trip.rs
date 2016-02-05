use std::io;
fn get_integer() -> i32 {

    let mut return_int = -1i32;

    //I suspect I am doing an ugly hack to makes this work in rust
    //BUT while return_int is  -1 I just keep on asking
    //TODO: post to stack overflow to find the correct way to handle this case
    while return_int < 0 {

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

//I would  use a linked list to this properly but to answer their specific question this will do :|

fn calc_value(people: i32) -> i32 {

    println!("Entering calc_value");
    let mut total = 0;

    let mut v = vec![people; 0];

    let mut count = 0;
    for i in &mut v {
        //let index: usize = *i as usize;
        count += 1;
        println!("Enter Value for person {}", count);
        *i = get_integer();
        total += *i;
    }

    let avg = total/people;
    
    let mut change_hands = 0;

    count = 0;
    for i in &v {
        //let index: usize = *i as usize;
        count += 1;
        if *i > avg {
            let owed = *i - avg;
            println!("Person {} is owed ${}", count, owed);
        } else {
            let must_pay = avg - *i;
            println!("Person {} must pay ${}", count, must_pay);
            change_hands += *i;
        }
    }
    change_hands
}

fn main () {

    println!("How many people went on the trip: ");

    let people = get_integer();

    let change_hands = calc_value(people);

    println!("Total money to change hands: ${}", change_hands);
}
    
   
