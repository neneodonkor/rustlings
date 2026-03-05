// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars(); // hexagon => ['h', 'e', 'x', 'a', 'g', 'o', 'n']
    match chars.next() {
        // => it extracts the first letter from the iterator.
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        // it makes the first letter uppercase, converts to string, then appends the rest of the iterator ['e', 'x', 'a', 'g', 'o', 'n']
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut collection = Vec::new();

    for word in words {
        let result = capitalize_first(word);
        collection.push(result);
    }
    return collection;
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    let mut text = String::new();

    for word in words {
        let result = capitalize_first(word);
        text.push_str(&result);
    }
    text.to_string()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
