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
    n % 2 == 0
}


fn main() {
    let num_array: [i32; 10] = [1,2,3,4,5,6,7,8,9,15];
    println!("array is: {:?}", num_array);


    for num in num_array {
        if is_even(num) {
            //not correctly running "FizzBuzz" if placed after the other two
            if num % 3 == 0 && num % 5 == 0 {
                println!("FizzBuzz")
            }
            else if num % 3 == 0 {
                println!("Fizz")
            }
            else if num % 5 == 0 {
                println!("Buzz")
            }
            else {
                println!("{} is even", num);
            }
        }
        else {
            //not correctly running "FizzBuzz" if placed after the other two
            if num % 3 == 0 && num % 5 == 0 {
                println!("FizzBuzz")
            }
            else if num % 3 == 0 {
                println!("Fizz")
            }
            else if num % 5 == 0 {
                println!("Buzz")
            }
            else {
                println!("{} is odd", num);
            }
        }
    }

    let mut incrementor = 0;
    let mut _sum = 0;
    while incrementor < num_array.len(){
        _sum += num_array[incrementor];
        incrementor += 1;
    }
    println!("Sum of the array is: {}", _sum);

    //setting the variable as the value of the first element in the array
    let mut largest = num_array[0];
    for num in num_array {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}