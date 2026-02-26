// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

use crate::quiz_module::transformer;

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod quiz_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }
    // ("hello".to_string(), Command::Uppercase),
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for (label, action) in input {
            match action {
                Command::Uppercase => {
                    // when inserting an element to the end of the vector use lenght of the vector for the index.
                    result.insert(result.len(), label.to_uppercase());
                }
                Command::Trim => {
                    // trim() converts label to &str so you have to convert it back to String with to_string()
                    result.insert(result.len(), label.trim().to_string());
                }
                Command::Append(num) => {
                    let mut temp = label;

                    // To iterate a variable of usize you convert it into an iterator by typing 0..num
                    for _i in 0..num {
                        // When appending strings together, the first operand must be of type String while the second one will be of type &str.
                        // So it will be String = String + &str
                        temp = temp + "bar";
                        // or temp.push_str(&label);
                    }

                    // To append to a vector, you use the insert method.
                    result.insert(result.len(), temp.to_string());
                }
            }
        }

        return result;
    }
}

fn main() {
    // You can optionally experiment here.
    let input = vec![
        ("hello".to_string(), Command::Uppercase),
        (" all roads lead to rome! ".to_string(), Command::Trim),
        ("foo".to_string(), Command::Append(1)),
        ("bar".to_string(), Command::Append(5)),
    ];

    let output = transformer(input);
    println!("My output is {:?}", output)
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use crate::quiz_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
