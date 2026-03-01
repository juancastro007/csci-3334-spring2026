/*
    //01rust-programming-mutables
    /*
    allows the variable be able to mutate to fix the data type post initialization
    */
    let mut result : f32 = 0.0; //int
    let x:i32 = 5;              //float
    result = result + x as f32; //no implicit conversion

	println!("{}", result);

    /*
    //example utilizing shadowing and showing variable scope
    which are the {} with the print inside and out
    let x = 5;
    {
        let x = x + 10;
        println!("x: {}",x);
    }
    println!("x: {}",x);
    */
*/////////////////////////////////////////////////////

/*
    //02rust-variables-shadowing
    let x:i32 = 5;
    let x:f32 = 1.012;
    println!("{}",x)
    /*
    using the keyword "let" before a variable name allows you to use
    the same name AS LONG AS the data type is NOT shared between them.
    i.e. hardcoding x to an int 5 and also x as float 1.012 will provide
    two seperate values.
*/*////////////////////////////////////////////////////

/*
    // Converting a string to a number
    let num = "25";
    let num: u32 = num.parse()
        .expect("Please provide a valid number!");
    
    println!("Parsed number: {}", num);
    
    // Further transformations
    let num = num + 25;
    let num = num * 2;
    
    println!("Final value of num: {}", num);
*///////////////////////////////////////////////

/*
//05rust-functions.md
fn double(x:i32) -> i32 {
    //the arrow "->" indicates what kind of data type that should be returned
    x*2
}

fn main() {
    println!("double {} equals to {}", 5, double(5));
}
*/////////////////////////////////////////////////