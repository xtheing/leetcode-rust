use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 创建一个哈希映射来存储数值及其对应的索引
    let mut map = HashMap::new();

    // 遍历 nums 数组中的每一个元素
    for (i, &num) in nums.iter().enumerate() {
        // 计算当前元素所需的配对值
        let complement = target - num;

        // 检查哈希映射中是否存在这个配对值
        if let Some(&index) = map.get(&complement) {
            // 这里的&index 是一个模式匹配，index是解引用后的值。
            // 如果存在，说明找到了两个数，它们的和等于目标值
            return vec![index as i32, i as i32];
        }

        // 如果不存在，将当前元素及其索引加入哈希映射
        map.insert(num, i);
    }

    // 如果没有找到符合条件的两个数，返回一个空的 vector
    vec![]
}

fn main() {
    // 测试用例
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    // 调用函数并打印结果
    let result = two_sum(nums, target);
    println!("{:?}", result); // 输出: [0, 1]
}
