// 插入排序和冒泡其实还是挺像的，左右对比
fn insertion_sort(arr: &mut [i32]){
    let len = arr.len();
    for i in 1..len {  // 可以直接从二个元素遍历，和前面的比较。
        let mut j = i;
        while j>0 && arr[j-1] > arr[j] {  // 如果前面的元素大于当前元素，则交换位置
            arr.swap(j-1,j);
            // 注意继续检查，j已经交换了位置为j-1
            j -= 1;
        }
    }
}

fn main() {
    let mut numbers = [5, 3, 4, 1, 2];
    insertion_sort(&mut numbers);
    println!("{:?}", numbers);
}
