

pub fn run(){
    // 这是行注释的例子
    // 注意这里有两个斜线在本行的开头
    // 在这里面的所有内容编译器都不会读取

    println!("Hello world!");

    // 想要运行上述语句？ 现在请将上述语句的两条斜线删掉，并重新运行

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x={}", x);
}