use std::collections::HashMap;
// 滑动窗口方式思考 可以参考 0003_.rs
// 文档可以参考ob中的滑动窗口
// 滑动窗口的思路是：
// 1. 定义一个滑动窗口，初始时窗口的起始位置为0，窗口的结束位置为0。
// 2. 遍历字符串，将当前字符加入滑动窗口中。
// 3. 更新最长子串的长度。
// 4. 重复步骤2和3，直到遍历完整个字符串。
// 5. 返回最长子串的长度。
fn length_of_longest_substring(s: String) -> i32 {
    let mut char_indices = HashMap::new(); // 存储字符及其最新索引
    let mut max_length = 0; // 最长子串的长度
    let mut start = 0; // 滑动窗口的起始位置

    for (i, c) in s.chars().enumerate() {
        // 如果字符已经存在于当前窗口中，则更新窗口的起始位置
        if let Some(&prev_idx) = char_indices.get(&c) {
            start = start.max(prev_idx + 1); // 这里start 和 prev_idx + 1 取最大值，是为了保证start是最小的索引
        }

        // 更新字符的最新索引
        char_indices.insert(c, i);

        // 更新最长子串的长度
        max_length = max_length.max(i - start + 1);
    }

    max_length as i32
}

fn main() {
    let s = String::from("abcabcbb");
    println!(
        "The length of the longest substring without repeating characters is: {}",
        length_of_longest_substring(s)
    );
}
