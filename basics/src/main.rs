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
    //变量绑定 ,
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); // y 访问不到
}