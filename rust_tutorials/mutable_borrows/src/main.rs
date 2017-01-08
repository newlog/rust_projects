pub fn main() {
    let (mut prefix, suffix) = two_words();
    let mut prefix_copy1 = prefix.clone();
    let prefix_copy2 = prefix.clone() ;
    join_words1(&mut prefix, &suffix);
    join_words2(&mut prefix_copy1, &suffix);
    let joined_words1 = join_words3(prefix_copy2.clone(), suffix.clone());
    let joined_words2 = join_words4(prefix_copy2, suffix);


    println!("concatenated string is {:?}", prefix);
    println!("concatenated string is {:?}", prefix_copy1);
    println!("concatenated string is {:?}", joined_words1);
    println!("concatenated string is {:?}", joined_words2);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

/// Concatenate `suffix` onto the end of `prefix`.
fn join_words1(prefix:&mut String, suffix: &String) {
    prefix.push(' '); // separate the words with a space
    for ch in suffix.chars() {
        prefix.push(ch);
    }
}

fn join_words2(prefix:&mut String, suffix: &String) {
    prefix.push_str(&(" ".to_owned() + suffix));
}

fn join_words3(prefix: String, suffix: String) -> String {
    format!("{} {}", prefix, suffix)
}

fn join_words4(prefix: String, suffix: String) -> String {
    format!("{} {}", prefix, suffix)
}

// Challenge: Convert `join_words` to use borrowing, not ownership.
// The new function should mutate `prefix` in place, and should not
// take ownership of `suffix`.
//
// Hint: If you'd like a hint as to how to proceed, open
// <http://rust-tutorials.com/exercises/hint-mutable-borrow-1.html>.

// Question: Now that you've converted `strcat`, what happens if you
// call `strcat` using the same string for `prefix` and `suffix`?
// Why?
