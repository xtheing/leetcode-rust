fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        // 假设最小的元素为当前位置元素
        let mut min_index = i;
        // 在剩余未排序的元素中继续寻找真正的最小元素
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        // 如果找到的最小元素不是当前位置元素，交换它们
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

fn main() {
    let mut arr = [64, 25, 12, 22, 11];
    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
