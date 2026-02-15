// Juan Castro csci-3334 assign#3
// Guessing game

// 1. Use a mutable variable to store a "secret_num" number (you can hard-code this).

// 2.Implement a function check_guess(guess: i32, secret_num: i32) -> i32 that returns:
// 0 if the guess is correct
// 1 if the guess is too high
// -1 if the guess is too low

// 3.In the main function:
// Use a loop to repeatedly:
// Set a mutable guess variable to a number of your choice (simulating user input)
// Call the check_guess function
// Use an if-else expression to print whether the guess was correct, too high, or too low
// If the guess was correct, break the loop

// After the loop ends, print how many guesses it took 
// (you'll need to track this in a variable)

fn check_guess(guess: i32, secret_num: i32) -> i32 {
    if guess == secret_num {
        0
    }
    else if guess > secret_num {
        1
    }
    else {
        -1
    }
}

fn main () {
    let secret_num = 7;

    let mut guess_count: i32 = 0;

    let guesses: [i32; 10] = [1,2,3,4,5,6,7,8,9,15];

    let mut index = 0;

    loop {
        let guess = guesses[index];
        guess_count += 1;
        println!("Guess #{:?}: {}", index+1, guess);

        let result = check_guess(guess, secret_num);

        if result == 0 {
            println!("Correct guess");
            break;
        }
        else if result == 1 {
            println!("Guess is too high");
        }
        else if result == -1 {
            println!("Guess is too low");
        }
        index += 1;
    }

    println!("Total guesses: {}", guess_count);
}