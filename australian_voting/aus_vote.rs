use std::io;
use std::mem;

//This code makes Strings to static str
//ripped from https://stackoverflow.com/questions/23975391/how-to-convert-string-into-static-str
fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

/*
 * asks user for float
 * no prompt!
 */
#[allow(dead_code)]
fn get_float() -> f64 {

    let mut return_float = -1.0;

    //I suspect I am doing an ugly hack to makes this work in rust
    //BUT while return_int is  -1 I just keep on asking
    //TODO: post to stack overflow to find the correct way to handle this case
    while return_float < 0.0 {

        let mut return_txt = String::new();

        let reader = io::stdin();

        reader.read_line(& mut return_txt).ok()
            .expect("Failed to read line.");

        let return_opt: Option<f64> = return_txt.trim()
            .parse::<f64>().ok();

        return_float = match return_opt {
            Some(return_float)    => return_float,
            None                => -1.0,

        };
    }

    return_float
}

/*
 * asks user for integer
 * No prompt!
 */
fn get_integer() -> i32 {

    let mut return_int = 0i32;

    //I suspect I am doing an ugly hack to makes this work in rust
    //BUT while return_int is  -1 I just keep on asking
    //TODO: post to stack overflow to find the correct way to handle this case
    while return_int == 0 {

        let mut return_txt = String::new();

        let reader = io::stdin();

        reader.read_line(& mut return_txt).ok()
            .expect("Failed to read line.");

        let return_opt: Option<i32> = return_txt.trim()
            .parse::<i32>().ok();

        return_int = match return_opt {
            Some(return_int)    => return_int,
            None                => 0,

        };
    }

    return_int
}

/*
 * asks user for string and returns value stripped of 
 * white space
 */
fn get_string() -> &'static str {

    let mut return_string = String::new();

    let reader = io::stdin();

    reader.read_line(& mut return_string).ok()
        .expect("I have failed :(");

    string_to_static_str(return_string).trim()
}

/*
 * builds values for and calculates 
 * the winner for a single case
 */

fn lets_vote() -> &'static str {

    let mut candidates_num = 0;

    while candidates_num == 0 {
        println!("Please enter number of candidates (less than 20): ");
        candidates_num = get_integer();
        if candidates_num > 20 || candidates_num < 0 {
            candidates_num = 0;
        }
    }
    
    let mut candidates_names =vec![""; candidates_num as usize];
    
    let mut count = 0;
    
    for i in & mut candidates_names {
        
        count += 1;

        println!("Please enter candidate {} name: ", count);

        *i = get_string();

    }

    let mut voter_num = 1001;
    
    while voter_num >= 1000 {
        println!("Please enter number of voters (less than 1000): ");

        voter_num = get_integer();

    }

    let mut votes = vec![vec![0; candidates_num as usize]; voter_num as usize];
    
    let mut counti = 0;
    
    for i in &mut votes {
        
        counti += 1;
        println!("Voter {} please enter votes",counti);
        let mut countj = 0;

        for j in &mut *i {
            countj += 1;
            println!("Please enter vote for {} ", candidates_names[countj - 1]);
            *j = get_integer();
        }
    }

    //Now I have to calculate the input above to come up with a winner
    //votes contains each persons vote so to calculate 
    //I need to tally [[x]voter] will be the vote
    //for candidates_name[x]
    
    let mut vote_tally = vec![0; candidates_num as usize];
    let mut voter_count = 0;

    for i in & mut vote_tally {
        let mut tally = 0;
        
        for voter in  & mut votes {
            tally += voter[voter_count];
        }
        voter_count += 1;

        *i = tally;
    }
    
    let mut candid_count = 0;

    for i in & mut candidates_names {
        println!("{} Total Votes: {}", *i, vote_tally[candid_count]);
        candid_count += 1;
    }

    //given we now have the vote_tally we can calc the winner
    let mut winner = 0;
    let mut index_counter = 0;
    let mut index_winner = 0;

    for i in & mut vote_tally {
        if *i > winner {
            index_winner = index_counter;
            winner = *i;
            index_counter += 1;
        } else {
            index_counter += 1;
        }
    }
    
    candidates_names[index_winner]

}

/*
 * wraps let_vote() around x number of cases
 */
fn cases_vote(cases: i32)  {
    let mut winners = vec![""; cases as usize];
    let mut el_counto = 1;
    for i in & mut winners {
        println!("Please enter details for case {}.", el_counto);
        *i = lets_vote();
        println!("{} has won case {}.", *i, el_counto);
        el_counto += 1;
    }
}


fn main() {
    println!("Please enter number of cases: ");
    let cases = get_integer();
    cases_vote(cases);
}
