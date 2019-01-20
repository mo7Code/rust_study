fn main() {
    // variate();
    variate_scope();
}
//变量
fn variate(){
    //变量绑定
    let x = 5; // x: i32
    println!("The value of x is: {}", x);

    //模式
    let (x, y) = (1, 2);
    println!("The value of x is: {}", x,);
    println!("The value of y is: {}", y);

    // 类型注解
    let x: i32 = 5;
    println!("The value of x is: {}", x,);

    //可变性
    let mut x = 5; // mut x: i32
    println!("The value of x is: {}", x);
    x = 10;
    println!("The value of x is: {}", x);

    //初始化绑定
    // let x: i32;
    // println!("The value of x is: {}", x)//error
}
//变量作用域 和js一毛一样~
fn variate_scope(){
    /*
    //变量局部作用域,闭包
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // println!("The value of x is {} and value of y is {}", x, y); // y 访问不到会报错
    */

    /*
    //覆盖 + 闭包
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "8".
    let x =  42;
    println!("{}", x); // Prints "42".
    */

    let mut x: i32 = 1;
    println!("111  The value is: {}", x);
    x = 7;
    println!("222  The value is: {}", x);
    let x = x; // `x` is now immutable and is bound to `7`
    println!("333  The value is: {}", x);

    let y = 4;
    println!("444 The value is: {}", y);
    let y = "I can also be bound to text!"; // `y` is now of a different type
    println!("555 The value is: {}", y);

}
