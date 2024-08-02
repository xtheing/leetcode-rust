// use std::cell::RefCell;

// fn main() {
//     let value = RefCell::new(42); // 包裹一个 i32 类型的值

//     // 借用并修改 value
//     {
//         let mut value_borrowed_mut = value.borrow_mut(); // 获取一个可变引用
//         *value_borrowed_mut += 1; // 修改内部值
//     } // value_borrowed_mut 在这里离开作用域并被释放

//     // 再次借用并读取 value
//     {
//         let value_borrowed = value.borrow(); // 获取一个不可变引用
//         println!("The value is: {}", *value_borrowed);
//     } // value_borrowed 在这里离开作用域并被释放
// }

fn main() {
    let mut s = String::from("hello");
    let mut z = s.clone();

    s.push_str(", world!");
    z.push_str(" world!");

    println!("s: {}", s);
    println!("z: {}", z);
}
