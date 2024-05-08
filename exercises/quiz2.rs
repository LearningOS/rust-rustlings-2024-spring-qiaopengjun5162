// quiz2.rs
//
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
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    /*
    这段代码定义了一个函数 `transformer` ，
    接受一个元组类型的 `Vec` 作为输入，
    元组包含一个 `String` 和一个 `Command` 。
    函数会根据 `Command` 的不同对 `String` 进行处理，
    然后返回处理后的 `String` 组成的 `Vec` 。

    1. 创建一个空的 `Vec` 类型的 `output` 用于存储处理后的 `String` 。
    2. 遍历输入的 `Vec` ，对每个元组中的 `String` 和 `Command` 进行处理。
    3. 根据 `Command` 的类型，分别对 `String` 进行不同的处理：
        - 如果是 `Uppercase` 类型的 `Command` ，将 `String` 转换为大写并加入到 `output` 中。
        - 如果是 `Trim` 类型的 `Command` ，将 `String` 去除空格并加入到 `output` 中。
        - 如果是 `Append` 类型的 `Command` ，将 `String` 复制n次"bar"后拼接，并加入到 `output` 中。
    4. 返回处理后的 `output` 。
    */
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => output.push(string.to_owned() + &"bar".repeat(*n)),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
