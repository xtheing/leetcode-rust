// 选择排序，从小大到排序。
fn selection_sort(arr: &mut [i32]) {  // 借用传进来的i32数组
    let len = arr.len();  // 先找到长度，用于循环遍历
    for i in 0..len {
        // 先给定一个最小元素用于比对。
        let mut min_index = i;  // 这里是栈的copy
        // 比对剩下的元素，找到真正的min_index
        for j in (i+1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        // 更改arr，进行排序
        if min_index != i {
            arr.swap(i,min_index);  // 使用swap交换两个值
        }
    }
}

fn main() {
    let mut arr=[5, 3, 2, 4, 1];
    selection_sort(&mut arr);  // 如果不使用可变引用，会不会清晰一些？
    println!("sorted array: {:?}",arr)  // format 输出
}
