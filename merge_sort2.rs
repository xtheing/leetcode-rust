fn merge_sort(arr: &mut [i32]){
    let len = arr.len();
    if len<=1 { 
        return;
    }
    // 取整对半分
    let mid = len/2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    // todo merge
    let result = merge(&arr[..mid],&arr[mid..]);
    arr.copy_from_slice(&result)
}
fn merge(left: &[i32],right: &[i32]) -> Vec<i32>{
    let mut result = Vec::new();
    let mut left_idx = 0;
    let mut right_idx= 0;

    while left_idx<left.len() && right_idx< right.len() {
        if left[left_idx] <= right[right_idx] {
            result.push(left[left_idx]);
            left_idx +=1;
        } else {
            result.push(right[right_idx]);
            right_idx +=1;
        }
    }
    
    if left_idx < left.len(){
        result.extend_from_slice(&left[left_idx..]);
    }
    if right_idx < right.len(){
        result.extend_from_slice(&right[right_idx..]);
    }
    result
}

fn main(){
    let mut numbers = vec![5, 3, 4, 1, 2];
    merge_sort(&mut numbers);
    println!("排序后为 {:?}",numbers);
}
