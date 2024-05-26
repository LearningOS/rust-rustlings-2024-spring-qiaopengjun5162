// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    /// 这段代码是Rust语言中的一个表达式，
    /// 用于计算从1到给定整数`num`的范围内所有整数的乘积。
    /// `(1..=num)`是一个范围表达式，表示从1开始，到`num`（包括`num`）结束的整数序列。
    /// `product()`是一个方法，用于计算序列中所有元素的乘积。
    /// 这个表达式的结果是一个整数，表示从1到`num`的范围内所有整数的乘积。
    /// 例如，如果`num`等于5，那么表达式的结果就是1乘以2乘以3乘以4乘以5，等于120。
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
