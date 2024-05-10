// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    /*
        这段代码是Rust语言中的一个匹配表达式（match expression），用于处理可能为`Some`也可能为`None`的`Option`类型。`Option`类型是Rust中的一种枚举类型，用于表示一个值的存在可能。当处理可能为`None`的值时，可以使用`match`表达式来避免`panic`（崩溃）。

    具体来说，这段代码的目的是打印一个点的坐标。`y`是一个`Option`类型的变量，可能包含一个`Point`类型的值，也可能为`None`。如果`y`包含一个`Point`类型的值，那么就打印该点的坐标；如果`y`为`None`，那么就触发`panic`。

    `match`表达式的原理是通过将`y`与`Some`进行匹配，如果匹配成功，则执行`println!`宏，打印坐标；如果匹配失败（即`y`为`None`），则执行`panic!`宏，导致程序崩溃。

    需要注意的是，`match`表达式只能用于`Option`类型，不能用于其他类型。如果需要处理其他类型的值，可以考虑使用`if`语句或其他逻辑操作。


         */
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

/*
在Rust语言中，`ref`关键字用于创建一个引用。引用是一种指向特定变量的指针，允许你在代码中多次使用同一个值，而不需要多次分配内存。

在这个例子中，`ref p`是一个模式，用于匹配一个可选的值，并将其解包为一个引用的`p`。这意味着`p`是一个指向`Some`中的值的引用，而不是一个具体的值。

当你在`match`表达式中使用`ref`关键字时，你需要确保在`match`的分支中使用`p`，而不是`Some(p)`。这是因为`ref`关键字创建了一个引用，而不是将值解包到`p`。

这是一个使用`ref`关键字的例子：

```rust
fn main() {
    let mut Some(ref p) = Some(10);
    println!("p: {}", p); // 输出：p: 10
}
```

在这个例子中，我们首先创建了一个包含`Some(10)`的`Option`类型，然后使用`Some(ref p)`模式将其解包为一个引用的`p`。然后我们打印`p`的值，输出为`10`。



*/
