// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// 定义宏：宏的名称是 my_macro
macro_rules! my_macro {
    // 模式匹配：当宏被无参数调用时，执行下面的代码
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 调用宏：注意宏调用需要使用感叹号 !
    my_macro!();  // 添加了感叹号
}