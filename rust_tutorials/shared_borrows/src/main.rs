pub fn main() {
    let string = format!("my friend");
    greet(&string[3..]);
    greet(&string[3..]);
    count_word_lenghts(string);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn count_word_lenghts(quote: String) {
    for word in quote.split(' ') {
        println!("Word: {:?}, Length: {:?}", word, word.len());
    }
}

// Goal #1: Convert `greet` to use borrowing, not ownership, so that
// this program executes without doing any cloning.
//
// Goal #2: Use a subslice so that it prints "Hello, friend" instead of
// "Hello, my friend".
