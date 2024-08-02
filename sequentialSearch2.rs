// 也可以直接返回-1
fn sequential_search(arr: &[i32], target: &i32) -> isize {
    for i in 0..arr.len(){
        if arr[i] == *target{
            return i as isize;
        }
    }
    -1
}

fn main(){
    let arr = vec![4,2,5,7,9,6];
    let target = 9;
    let target_idx = sequential_search(&arr,&target);
    println!{"{}",target_idx}
}
