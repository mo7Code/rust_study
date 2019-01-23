fn main() {

    //移动语义****
    // fn take(_v: Vec<i32>) {
    //     println!("v[1] is: {}", _v[1]);
    //     // What happens here isn’t important.
    // }

    // let v = vec![1, 2, 3];

    // println!("v[0] is: {}", v[0]);
    // take(v);

    // let v = vec![1, 2, 3];
    // println!("v[0] is: {}", v[0]);
    // let v2 = v;
    // println!("v[0] is: {}", v2[0]);

    //Copy类型
    let v = 1;

    let v2 = v;

    println!("v is: {}", v);
    println!("v2 is: {}", v2);

    //所有权之外（More than ownership） 没看懂这部分


}

