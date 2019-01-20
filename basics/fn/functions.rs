fn main() {
    // print_number(1000);
    // print_sum(5,6);
    // let x = 1;
    // println!("{}",add_one(x));
    // diverges()
    plus_one();
}
//参数必须声明类型
// fn print_number(x:i32) {
//     println!("x is: {}", x);
// }

//多个参数
// fn print_sum(x: i32, y: i32) {
//     println!("sum is: {}", x + y);
// }

//返回值 , rust只能返回一个值
// fn add_one( mut x: i32) -> i32 {
//     x += 1;
//     x += 1;
//     x += 1;
//     999 + x
// }


//发散函数 , 永不返回函数 , 一般用于错误或者崩溃提示
// fn diverges() -> ! {
//     panic!("This function never returns!");
// }

//函数指针 , 和js一毛一样
fn plus_one () {
    fn plus_one(i: i32) -> i32 {
        i + 1
    }
    let f = plus_one;
    let six = f(5);
    println!("sum is: {}", six);
}
