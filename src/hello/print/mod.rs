mod print_debug;
mod print_display;
mod fmt;


pub fn init(){
    println!("===============print::init=============");
    run();
    println!("===============print_debug::run=============");
    print_debug::run();
    println!("===============print_display::run=============");
    print_display::init();
    println!("===============fmt::run=============");
    fmt::run();
}




// 格式化输出
fn run(){
    // 通常情况下， `{}` 会被任意变量替换
    // 值内容会转化成字符串
    println!("{} days", 31);

    // 不加后缀的话， 31 自动成为 I32 类型
    // 你可以加后缀来改变31的原来类型

    // 下面有多种可选形式
    // 可以使用的位置参数
    println!("{0}, thisis {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用赋值语句。
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    //特殊的格式实现可以在后面加上 `:` 符号
    println!("{} of {:b} people know binary, the other half don't,", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出“    ·”， 5个空格后面连着1。
    println!("{number:>width$}", number=1, width=6);

    // 你可以对数字左边位数上补0. 下面语句输出“000001”。
    println!("{number:>0width$}", number=1, width=6);

    // 你可以对数字左边位数上补0. 下面语句输出“000001”
    println!("{number:>0width$}", number=1, width=6);

    // println! 会检查使用到的参数数量是否正确
    println!("My name is {0}, {1}, {0}", "James","Bond");
    // 改正 ^ 补上漏掉的参数： “James"

    // 创建一个包含 I32 类型结构体（structure）。 命名为 `Structure`
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样自定义类型需要理复杂的方式来处理。
    // 下面语句无法运行
    //println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行

    //练习: 打印 Pi is roughly 3.142。 let pi = 3.1415926 (note: 小数位参考std::fmt)
    let pi = 3.1415926;
    println!("Pi is roughly {:.*}", 2, pi);
}


