fn main() {
    //无限循环
    // loop {
    //     println!("Loop forever!");
    // }


    //while
    // let mut x = 5; // mut x: i32
    // let mut done : bool = false; // mut done: bool

    // while !done {
    //     x += x - 3;
    //     println!("{}", x);
    //     if x % 5 == 0 {
    //         done = true;
    //     }
    // }


    //for
    // for x in 0..10 {
    //     println!("{}", x); // x: i32
    // }


    //Enumerate 方法 , 记录循环了多少次
    // for (index, value) in (5..90).enumerate() {
    //     println!("index = {} and value = {}", index, value);
    // }

    //对于迭代器（On iterators）:
    // let lines = "hello\nworld".lines();
    // for (linenumber, line) in lines.enumerate() {
    //     println!("{}: {}", linenumber, line);
    // }

    //提早结束迭代（Ending iteration early）
    // let mut x = 5;
    // loop {
    //     x += x - 3;
    //     println!("{}", x);
    //     if x % 5 == 0 { break; }
    // }
        //终结当前循环
    // for x in 0..10 {
    //     if x % 2 == 0 { continue; }
    //     println!("{}", x);
    // }

    //循环标签（Loop labels）  用来标记当前循环是哪一个,然后打断
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }


}
