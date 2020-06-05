fn main() {
    // println!("Hello, world!");
    //
    // //重影概念  可以重新绑定的不可变变量
    // let a=12;
    // let a=15;
    // println!("a is {}",a);
    //
    // //let b=456; //不可变变量
    // //TODO mut 使其变为可变变量
    // let mut b=456;
    // b=789;
    //
    // println!("b is {}",b);

    //加减乘除
    // let sum = 5 + 10; // 加
    // let difference = 95.5 - 4.3; // 减
    // let product = 4 * 30; // 乘
    // let products=4.5*2.6;
    // let quotient = 56.7 / 32.2; // 除
    // let remainder = 43 % 5; // 求余
    // println!("sum is {}",sum);
    // println!("difference is {}",difference);
    // println!("product is {}",product);
    // println!("products is {}",products);
    // println!("quotient is {}",quotient);
    // println!("remainder is {}",remainder);
    //TODO ：Rust 不支持 ++ 和 --，因为这两个运算符出现在变量的前后会影响代码可读性，减弱了开发者对变量改变的意识能力。

    //TODO :字符型用 char 表示。Rust的 char 类型大小为 4 个字节，代表 Unicode标量值，这意味着它可以支持中文，日文和韩文字符等非英文字符甚至表情符号和零宽度空格在 Rust 中都是有效的 char 值。
    //TODO:
    // Unicode 值的范围从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF （包括两端）。 但是，"字符"这个概念并不存在于 Unicode 中，因此您对"字符"是什么的直觉可能与Rust中的字符概念不匹配。
    // 所以一般推荐使用字符串储存 UTF-8 文字（非英文字符尽可能地出现在字符串中）。
    // 注意：由于中文文字编码有两种（GBK 和 UTF-8），所以编程中使用中文字符串有可能导致乱码的出现，这是因为源程序与命令行的文字编码不一致，
    // 所以在 Rust 中字符串和字符都必须使用 UTF-8 编码，否则编译器会报错。
    //TODO:复合类型
    // 元组用一对 ( ) 包括的一组数据，可以包含不同种类的数据：
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// // tup.0 等于 500
// // tup.1 等于 6.4
// // tup.2 等于 1
//     let (x, y, z) = tup;
// // y 等于 6.4
//     print!("x{}",x);
//     println!("y {}",y);
//     println!("z {}",z);

    //TODO 数组
//     let a = [1, 2, 3, 4, 5];
// // a 是一个长度为 5 的整型数组
//
//     let b = ["January", "February", "March"];
// // b 是一个长度为 3 的字符串数组
//
//     let c: [i32; 5] = [1, 2, 3, 4, 5];
// // c 是一个长度为 5 的 i32 数组
//
// TODO：
//    let d = [3; 5];
//    等同于 let d = [3, 3, 3, 3, 3];
//
//     let first = a[0];
//     let second = a[1];
// // 数组访问
//
//   //  a[0] = 123; // 错误：数组 a 不可变
//     let mut a = [1, 2, 3];
//     a[0] = 4; // 正确

//  TODO：rust方法：
    //TODO：函数返回值

    // // 在函数体中，随时都可以以 return 关键字结束函数运行并返回一个类型合适的值。这也是最接近大多数开发者经验的做法：
    // let l=5;
    // let h=90;
    // let p=sumNumber(l,h);
    // println!("{}",p);
    // fn sumNumber(l :i32,h :i32)->i32{
    //     let o=l+h;
    //     return o;
    // }
    // 在上一个嵌套的例子中已经显示了 Rust 函数声明返回值类型的方式：在参数声明之后用 -> 来声明函数返回值的类型（不是 : ）。
    //TODO:
    // 但是 Rust 不支持自动返回值类型判断！如果没有明确声明函数返回值的类型，函数将被认为是"纯过程"，不允许产生返回值，return 后面不能有返回值表达式。这样做的目的是为了让公开的函数能够形成可见的公报。
    // 注意：函数体表达式并不能等同于函数体，它不能使用 return 关键字。
   //TODO: 条件判断语句:
    //TODO: 表达式：
   //      let a = 3;
   //      let number = if a > 0 { 1 } else { -1 };
   //      println!("number 为 {}", number);
    //TODO:普通：
    // let a = 12;
    // let b;
    // if a > 0 {
    //     b = 1;
    // }
    // else if a < 0 {
    //     b = -1;
    // }
    // else {
    //     b = 0;
    // };
    // println!("b is {}", b);

    //TODO： 循环
    //TODO: while:
    // let mut number:i64 = 1;
    // while number != 4 {
    //     println!("{}", number);
    //     number += 1;
    // }
    // println!("EXIT");

    //TODO: for
    //     let a = [10, 20, 30, 40, 50];
    //     for i in a.iter() {
    //         println!("值为 : {}", i);
    //     }

    //TODO ： loop
    // let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    // let mut i = 0;
    // loop {
    //     let ch = s[i];
    //     if ch == 'O' {
    //         break;
    //     }
    //     println!("\'{}\'", ch);
    //     i += 1;
    // }

    //TODO:所有权对大多数开发者而言是一个新颖的概念，它是 Rust 语言为高效使用内存而设计的语法机制。所有权概念是为了让 Rust 在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念。
    // 1.所有权规则
    // 所有权有以下三条规则：
    // Rust 中的每个值都有一个变量，称为其所有者。
    // 一次只能有一个所有者。
    // 当所有者不在程序运行范围时，该值将被删除。

    //TODO: 变量与数据交互的方式
    //基本数据类型经过赋值后可以同时存在两个
    //TODO: 注意：
    // 两个个 String 对象在栈中，每个 String 对象都有一个指针指向堆中的 "hello" 字符串。在给 s2 赋值时，只有栈中的数据被复制了，堆中的字符串依然还是原来的字符串。
    // 前面我们说过，当变量超出范围时，Rust 自动调用释放资源函数并清理该变量的堆内存。但是 s1 和 s2 都被释放的话堆区中的 "hello" 被释放两次，这是不被系统允许的。
    // 为了确保安全，在给 s2 赋值时 s1 已经无效了。没错，在把 s1 的值赋给 s2 以后 s1 将不可以再被使用。下面这段程序是错的：
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1); // 错误！s1 已经失效。s1 名存实亡。

    //TODO: clone
    // Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，且采用移动的方式进行数据交互。但如果需要将数据单纯的复制一份以供他用，可以使用数据的第二种交互方式——克隆。
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);
    //这里是真的将堆中的 "hello" 复制了一份，所以 s1 和 s2 都分别绑定了一个值，释放的时候也会被当作两个资源。
    // 当然，克隆仅在需要复制的情况下使用，毕竟复制数据会花费更多的时间。
}
