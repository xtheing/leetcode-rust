// 可能找到，最好的做法是用一个Option类型。
fn sequential_search(arr: &[i32], target: &i32)-> Option<usize>{
    for i in 0..arr.len(){
        if arr[i] == *target{
            return Some(i);
        }
    }
    None
}

fn main(){
    let arr = vec![4,2,5,7,9,6];
    let target = 7;
    if let Some(target_idx) = sequential_search(&arr,&target) {
        println!{"{:?}",target_idx};
    } else {
        println!{"-1"};
    }
}
