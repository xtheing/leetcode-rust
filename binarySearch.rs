fn binary_search(array: &[i32], target: &i32) -> isize{
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if array[mid] == *target{
            return mid as isize;
        } else if array[mid] < *target {
            left = mid +1;
        } else {
            right = mid -1;
        }
    }
    -1
}

fn main(){
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 9;
    let result = binary_search(&array,&target);
    println!("{:?}",result)
}
