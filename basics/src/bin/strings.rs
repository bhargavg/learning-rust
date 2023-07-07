fn main() {
    print_grapheme_clusters();
    appending_strings_with_add_method();
    appending_strings_with_format_macro();
}

fn print_grapheme_clusters() {
    use unicode_segmentation::UnicodeSegmentation;

    let s = "భార్గవ్ గుర్లంక";

    for cluster in s.graphemes(true) {
        print!("{cluster}, ");
    }

    println!();

    for c in s.chars() {
        print!("{c}, ");
    }

    println!();
}

fn appending_strings_with_add_method() {
    // The signature of `String::add` function is:
    // ```
    // fn add(self, s: &str) -> String
    // ```
    // Couple of points to note from the signature:
    // 1. To add a string to another:
    //   - left side should be an owned `String` type
    //   - right side should be an `&str`.
    // 2. After addition, left string is no longer available.
    let s1 = String::from("How ");
    let s2 = String::from("are ");
    let s3 = String::from("you?");

    // s1 + &s2 + &s3 => (s1 + &s2) + &s3
    //                => s1 + &s3  (s1 has s2 contents appended)
    //                => s1  (s1 has s3 contents appended)
    let s4 = s1 + &s2 + &s3;

    println!("{s4}");
}

fn appending_strings_with_format_macro() {
    let s1 = String::from("How ");
    let s2 = String::from("are ");
    let s3 = String::from("you?");
    let s4 = format!("{s1}{s2}{s3}");
    println!("{s4}");
}
