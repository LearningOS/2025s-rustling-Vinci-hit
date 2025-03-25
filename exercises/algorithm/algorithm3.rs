/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T:std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug>(arr: &mut [T]){
    if arr.len() > 1 {
        quick_sort_range(arr, 0, arr.len() - 1);
    }
}
fn quick_sort_range<T:std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug>(arr:&mut [T],lo:usize,hi:usize){
    if lo < hi {
        let p = partition(arr, lo, hi);
        if p>0{
            quick_sort_range(arr, lo, p - 1);
        }
        quick_sort_range(arr, p + 1, hi);
    }
}
fn partition<T:std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug>(arr:&mut [T],lo:usize,hi:usize) -> usize{
	let pivot = lo;
    let temp = arr[pivot].clone();
    let (mut left, mut right) = (lo, hi);
    while left < right{
        while left < right && arr[right] >= temp{
            right -= 1;
        }
        if left != right{
            arr.swap(left, right);
        }
        while left < right && arr[left] <= temp{
            left += 1;
        }

        if left != right{
            arr.swap(left, right);
        }
    }
    arr[left] = temp;
    left
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}