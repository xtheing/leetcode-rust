fn quick_sort(arr: &mut [i32]) {
    if arr.len() <=1{
        return;
    }
    // 分区返回一个基准位置
    let pivot_index = partition(arr);
    // 递归我们需要一个基准位置，根据基准位置进行下一次的递归分区
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index+1..]);  // ? 这里pivot_index 需要加一，是不包括pivot这个index的。

}


// partition 函数接收一个引用数组，返回基准位置
fn partition(arr: &mut [i32] ) -> usize {
    // 使用最后一个元素作为基准
    let mut i = 0;
    let pivot_idx = arr.len()-1;
    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);  // 小的排在前面
            i += 1;
        }
    }
    // 将pivot 放到正确的位置 索引i处，此时pivot 左侧的元素都比pivot小，右侧的元素都比pivot大
    arr.swap(i, pivot_idx);
    i
}

fn main() {
    let mut arr = [5, 3, 8, 6, 2, 7, 1, 4];
    quick_sort(&mut arr);
    println!("{:?}", arr);
}
