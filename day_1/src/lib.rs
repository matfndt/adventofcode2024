use std::collections::HashMap;

// fn quickselect(arr: &mut [i32], k: usize) -> i32 {
//     if arr.len() == 1 {
//         return arr[0];
//     }

//     let pivot_index = arr.len() / 2;
//     let pivot = arr[pivot_index];

//     let (mut left, mut right) = (0, arr.len() - 1);

//     while left <= right {
//         while arr[left] < pivot {
//             left += 1;
//         }
//         while arr[right] > pivot {
//             right -= 1;
//         }
//         if left <= right {
//             arr.swap(left, right);
//             left += 1;
//             if right > 0 {
//                 right -= 1;
//             }
//         }
//     }

//     if k <= right {
//         quickselect(&mut arr[0..=right], k)
//     } else if k >= left {
//         quickselect(&mut arr[left..], k - left)
//     } else {
//         pivot
//     }
// }

// k select works here but is not efficient since it needs to be called n times

// pub fn calc_sum_of_differences(array1: &mut [i32], array2: &mut [i32]) -> i32 {
//     let mut sum: i32 = 0;
//     let n = array1.len();

//     for k in 0..n {
//         let num1 = quickselect(array1, k);
//         let num2 = quickselect(array2, k);
//         // println!("{}: {} {}", k, num1, num2);
//         sum += (num1 - num2).abs();
//         // println!("sum: {}", sum);
//     }
//     sum
// }

fn radix_sort(arr: &mut [i32]) {
    let max_value = *arr.iter().max().unwrap_or(&0);
    let mut exp = 1;

    let mut buffer = vec![0; arr.len()];

    while max_value / exp > 0 {
        let mut count = [0; 10];

        for &num in arr.iter() {
            let digit = ((num / exp) % 10) as usize;
            count[digit] += 1;
        }

        for i in 1..10 {
            count[i] += count[i - 1];
        }

        for &num in arr.iter().rev() {
            let digit = ((num / exp) % 10) as usize;
            count[digit] -= 1;
            buffer[count[digit]] = num;
        }

        arr.copy_from_slice(&buffer);

        exp *= 10;
    }
}

pub fn calc_sum_of_differences(array1: &mut [i32], array2: &mut [i32]) -> i32 {
    radix_sort(array1);
    radix_sort(array2);

    array1
        .iter()
        .zip(array2.iter())
        .map(|(&num1, &num2)| (num1 - num2).abs())
        .sum()
}

// Part 2:
pub fn calc_weighted_sum(array1: &[i32], array2: &[i32]) -> i32 {
    let mut freq_map = HashMap::new();
    for &num in array2 {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    array1
        .iter()
        .map(|&num| num * freq_map.get(&num).unwrap_or(&0))
        .sum()
}
