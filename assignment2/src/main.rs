// Juan Castro csci-3334 assign#2
// Number analyzer

// 1. Create an array of 10 integer numbers

// 2. implement a function is_even(n: i32) -> bool
// that returns true if num is even and false otherwise

// 3. use a loop to iterate through the array and for each number
// print whether its even or odd
// if num is divisible by 3, print "Fizz" instead
// if num is divisible by 5, print "Buzz" instead
// if num is divisible by 3 and 5, print "FizzBuzz" instead

// 4. use a while loop to find and print the sum of all nums in the array

// 5. use a loop to find and print the largest num in the array


fn is_even(n: i32) -> bool {
    // if n/2
}

fn main() {
    let num_array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("array is: {:?}", num_array);

    for num in num_array.iter() {
        print!("{}", num);
    }
    println!();
}

// while loop, w/ counter = arraysize
// let mut total = 0
// start loop
// total += element value
// finish loop
// println!("The sum of the numbers in the array is: {}", total);

// 