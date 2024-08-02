// 导入标准库中的 HashSet 模块
use std::collections::HashSet;

// 定义一个名为 intersection 的函数，接收两个整数向量 nums1 和 nums2 作为参数，并返回一个整数组成的向量
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 将 nums1 转换为哈希集合 set1，去除重复元素
    let set1: HashSet<i32> = nums1.into_iter().collect();
    // 将 nums2 转换为哈希集合 set2，去除重复元素
    let set2: HashSet<i32> = nums2.into_iter().collect();

    // 找出 set1 和 set2 的交集，并将交集元素从引用转换为实际值，然后收集到一个向量中
    // set1.intersection(&set2).copied().collect()
    set1.intersection(&set2).map(|&x| x).collect::<Vec<i32>>()
}

// 主函数，用于测试 intersection 函数
fn main() {
    // 测试用例 1
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    // 调用 intersection 函数并打印结果
    let result = intersection(nums1, nums2);
    println!("{:?}", result); // 输出: [2]

    // 测试用例 2
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    // 调用 intersection 函数并打印结果
    let result = intersection(nums1, nums2);
    println!("{:?}", result); // 输出: [4, 9]
}

/**
问题：
- collect() 是一个收集器，一般配合 into_iter 使用。turbofish
具体可以参考：https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect

*/

// 其他解法
// use std::collections::HashSet;

// impl Solution {
//     pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//         nums1
//             .into_iter()
//             .collect::<HashSet<i32>>()
//             .intersection(&nums2.into_iter().collect::<HashSet<i32>>())
//             .map(|&x| x)
//             .collect()
//     }
// }

// 作者：Tab Liu
// 链接：https://leetcode.cn/problems/intersection-of-two-arrays/solutions/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
