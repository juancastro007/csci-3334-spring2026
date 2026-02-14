// Juan Castro csci-3334 assign#1
// Temperature Convertor

// 1. Declare constant for freezing point
// of water in fahrenheit (32F)

// 2. Implement two functions:
// far_to_cel(f: f64) -> (f64)
// cel_to_far(c: f64) -> (f64)

// 3. Declare mutable var w/ temp in F
// Convert it to cel and print result
// use a loop to convert and print next 5 ints
// ex. if starting w/ 32F also print conversions
// 33F, 34F, 35F, 36F, 37F

const FREEZING_POINT_IN_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_IN_F) * 5.0/9.0
}

/*fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + FREEZING_POINT_IN_F
}
*/

fn main() {
    let mut temp_f: f64 = 32.0;

    let temp_c = fahrenheit_to_celsius(temp_f);

    println!("Given temps {}F, you get {}C", temp_f,temp_c);

    println!("The next five degrees in F get converted to:");
    for _count in 1..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{}F = {}C", temp_f, temp_c);
    }
}
