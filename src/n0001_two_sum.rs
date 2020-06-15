// use std::collections::HashMap;
//
// /**
//  * [1] Two Sum
//  *
//  * Given an array of integers, return indices of the two numbers such that they
//  * add up to a specific target.
//  *
//  * You may assume that each input would have exactly one solution, and you may
//  * not use the same element twice.
//  *
//  * Example:
//  *
//  *
//  * Given nums = [2, 7, 11, 15], target = 9,
//  *
//  * Because nums[0] + nums[1] = 2 + 7 = 9,
//  * return [0, 1].
//  *
//  */
//
// // 遍历列表，通过Hash表记录元素位置，如果target-元素 值存在于列表中，则直接返回两个元素的位置
// //pub struct Solution;
// //
// //use std::collections::HashMap;
// //
// //impl Solution {
// //    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
// ////        声明map，分配空间
// //        let mut map = HashMap::with_capacity(nums.len());
// //        for (index, num) in nums.iter().enumerate() {
// //            println!("当前数字和下标: {}-{}", index, num);
// //            match map.get(&(target - num)) {
// //                None => {
// //                    map.insert(num, index);
// //                },
// //                Some(sub_index) => {
// //                    println!("sub_index是什么: {}", sub_index);
// //                    return vec![*sub_index as i32, index as i32];
// //                },
// //            }
// //        }
// //        vec![]
// //    }
// //}
// //
// //#[cfg(test)]
// //mod tests{
// //    use super::*;
// //
// //    #[test]
// //    fn test_1(){
// //        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 3, 5, 8, 10], 5));
// ////        assert_eq!(vec![1, 3], Solution::two_sum(vec![2, 3, 5, 8, 10], 11));
// //    }
// //}
//
// // TODO:BTreeMap比HashMap节省内存，试一下BTreeMap的方案
// pub struct Solution;
//
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
// //        先声明一个map，并且将map大小初始化
//         let mut map = HashMap::with_capacity(nums.len());
// //        遍历map
//         for (index, num) in nums.iter().enumerate(){
// //            get<Q: ?Sized>(&self, k: &Q)  这里get里面的参数必须是借用
//             println!("index 和 num : {}-{}", index, num);
//             match map.get(&(&target - num)) {
//                 None => {
//                     map.insert(num, index);
//                     println!("map.len={}", map.len());
//                 },
// //                如果target-num正好匹配到一个下标，则直接返回index和sub_index
//                 Some(sub_index) => return vec![*sub_index as i32, index as i32],
//             }
//             println!("map中index 和 num: {}-{:?}", num, map.get(&num));
//         }
// //        如果找不到匹配的结果直接返回一个空的数组
//         vec![]
//     }
// }
//
// #[cfg(test)]
// mod tests{
//     use super::*;
//
//     #[test]
//     fn test_1(){
//         assert_eq!(vec![3,4], Solution::two_sum(vec![1, 2, 3, 4, 5], 9));
//     }
// }


//给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
//
//
//
// 示例:
//
// 给定 nums = [2, 7, 11, 15], target = 9
//
//因为 nums[0] + nums[1] = 2 + 7 = 9
//所以返回 [0, 1]
//
// Related Topics 数组 哈希表
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len(){
                if target == nums[i] + nums[j] {
                    return vec![i as i32, j as i32]
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        assert_eq!(vec![3, 4], Solution::two_sum(vec![1, 2, 3, 4, 5], 9));
    }
}