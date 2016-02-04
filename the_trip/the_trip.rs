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
    
    let mut total = 0;

    let mut v = vec![people; 0];

    let total = 0;
    for i in &mut v {
        let index: usize = *i as usize;
        println!("Enter Value for person {}", i);
        v[index] = get_integer();
        total += v[index];
    }

    let avg = total/people;
    
    let change_hands = 0;

    for i in &v {
        let index: usize = *i as usize;
        if v[index] > avg {
            let owed = v[index] - avg;
            println!("Person {} is owed ${}", i, owed);
        } else {
            let must_pay = avg - v[index];
            println!("Person {} must pay ${}", i, must_pay);
            change_hands += v[index];
        }
    }
    change_hands
}

fn main () {

    println!("How many people went on the trip: ");

    let mut people = get_integer();

    let change_hands = calc_value(people);

    println!("Total money to change hands: ${}", change_hands);
}
    
   
