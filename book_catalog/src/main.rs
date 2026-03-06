// Juan Castro csci-3334
// Book Catalog

// 1. Create a "Book" struct w/ fields
// title: String
// author: String
// year: u16

// 2. Implement the fn's
// save_books(books: &Vec<Book>, filename: &str): Saves all books to a file.
// load_books(filename: &str) -> Vec<Book>: Loads books from a file.

// 3.  In main:
// Create a few "Book" instances
// Save the books to a file
//Load the books from the file and print them

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}