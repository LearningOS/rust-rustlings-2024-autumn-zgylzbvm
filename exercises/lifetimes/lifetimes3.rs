// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



struct Book {
    author: String,
    title: String,
}

fn main() {
    let name = String::from("Jill Smith");
    let title1 = String::from("Fish Flying");
    let book = Book { author: name, title: title1 };
    println!("{} by {}", book.title, book.author);
}
