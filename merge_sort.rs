fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    // 归并排序直接对半分
    let mid = len / 2;
    // 左边进行递归分割到单个元素。
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    // merge()
    // 经过递归处理后，这里merge 的都是arr中的某一段的左右两段。
    let result = merge(&arr[..mid],&arr[mid..]);
    // arr.copy_from_slice(&result)
    arr.copy_from_slice(&result)  //覆写到arr这个切片中
}

// 这里需要实现两个数组的排序合并。
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    // let mut result = Vec::with_capacity(left.len() + right.len());
    let mut result = Vec::new();
    let mut left_idx = 0;
    let mut right_idx = 0;

    // 循环推送到新的数组中
    while left_idx < left.len() && right_idx < right.len() {
        // 按照较小的元素加入到结果中
        if left[left_idx] <= right[right_idx] {
            result.push(left[left_idx]);
            left_idx +=1;
        } else {
            result.push(right[right_idx]);
            right_idx +=1;
        }
    }
    
    // 还要处理一下左边剩余，或者右边剩余的情况。
    // 如果左边还有剩余或者右边还有剩余，先添加左边的，再添加右边的。
    if left_idx < left.len() {
        result.extend_from_slice(&left[left_idx..]);
    }
    if right_idx < right.len() {
        result.extend_from_slice(&right[right_idx..]);
    }
    result
}

fn main() {
    let mut numbers = vec![5, 3, 4, 1, 2];
    merge_sort(&mut numbers);
    println!("After: {:?}", numbers);
}
