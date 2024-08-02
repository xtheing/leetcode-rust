// 快速排序函数，接受一个可变借用的i32类型的切片
fn quick_sort(arr: &mut [i32]) {
    // 如果数组长度小于等于1，则不需要排序，直接返回
    if arr.len() <= 1 {
        return;
    }
    
    // 调用partition函数进行分区，返回pivot（基准）的最终位置
    let pivot_index = partition(arr);
    
    // 递归对pivot左边的子数组进行快速排序
    quick_sort(&mut arr[0..pivot_index]);
    
    // 递归对pivot右边的子数组进行快速排序
    quick_sort(&mut arr[pivot_index+1..]);
}

// 分区函数，用于找到pivot的正确位置，并返回这个位置的索引
fn partition(arr: &mut [i32]) -> usize {
    // 选择中间的元素作为pivot，并将其与数组最后一个元素交换
    // let pivot_index = arr.len() / 2;
    // arr.swap(pivot_index, arr.len() - 1);
    
    let mut i = 0;
    // 遍历数组（除了最后一个元素，即pivot）
    for j in 0..arr.len()-1 {
        // 如果当前元素小于或等于pivot，则将其与索引i处的元素交换
        // 这样可以确保i左边的所有元素都小于或等于pivot
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    // 将pivot放到正确的位置（即索引i处），此时pivot左边的所有元素都小于它，
    // 而右边的所有元素都大于它
    arr.swap(i, arr.len() - 1);
    
    // 返回pivot的最终位置索引
    i
}

// 主函数，用于测试快速排序
fn main() {
    let mut numbers = vec![5, 3, 4, 1, 2];
    quick_sort(&mut numbers);
    println!("排序后为 {:?}", numbers); // 打印排序后的数组
}
