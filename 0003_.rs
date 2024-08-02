// 滑动窗口方式思考
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut max_length = 0;
        let mut left_index = 0;
        let mut _right_index = 0; // 没有用到这个
        let mut map = HashMap::new();

        for (i, vel) in s.chars().enumerate() {
            // 在字典中找是否有这个字符
            if let Some(&map_idx) = map.get(&vel) {
                left_index = map_idx + 1;
            }

            // 没找到就存一下
            map.insert(vel, i);
            max_length = max_length.max(i + 1 - left_index); // 对比一下当前找到的最长长度
        }
        max_length as i32
    }
}
