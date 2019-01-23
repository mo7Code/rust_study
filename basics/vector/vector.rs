fn main() {
    //访问元素
    // let v = vec ![1, 2, 3, 4, 5];
    // println!("The third element of v is {}", v[2]);

    // usize 类型进行索引
    // let v = vec![1, 2, 3, 4, 5];
    // let i: usize = 0;
    // // let j: i32 = 0;      //i32索引会报错
    // // v[j];         //i32索引会报错
    // println!("The third element of v is {}", v[i]);


    //越界访问
    // let v = vec![1, 2, 3, 4, 5, 6,7 ,9 ,10];
    // match v.get(7) {
    //     Some(x) => println!("Item 7 is {}", x),
    //     None => println!("Sorry, this vector is too short.")
    // }

    //迭代
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }


}