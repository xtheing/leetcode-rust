// 定义插入排序函数，接受一个可变引用的整数数组作为参数
fn insertion_sort(arr: &mut [i32]) {
    // 获取数组的长度
    let len = arr.len();
    // 从第二个元素开始遍历数组，因为我们假设第一个元素是已经排序的
    for i in 1..len {
        // 使用 while 循环将选中的元素移动到正确的位置
        let mut j = i;
        // 检查当前元素是否比前一个元素小
        while j > 0 && arr[j - 1] > arr[j] {
            // 如果是，交换这两个元素
            arr.swap(j - 1, j);
            // 继续检查前一个元素，直到找到当前元素的正确位置
            j -= 1;
        }
    }
}

// 主函数，程序的入口点
fn main() {
    // 创建一个未排序的整数数组
    let mut numbers = [5, 3, 4, 1, 2];
    // 打印未排序的数组
    println!("Before: {:?}", numbers);
    // 调用插入排序函数对数组进行排序
    insertion_sort(&mut numbers);
    // 打印排序后的数组
    println!("After: {:?}", numbers);
}
