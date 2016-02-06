use std::io;

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

fn get_string() -> String {

    let mut return_string = String::new();

    let reader = io::stdin();

    reader.read_line(& mut return_string).ok()
        .expect("I have failed :(");

    return_string
}

fn main() {
    println!("so give me a string?");

    let new_string = get_string();

    println!("you entered {}", new_string);
}
