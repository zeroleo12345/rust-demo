fn sum(numbers: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for num in numbers.iter() {
        match total.checked_add(*num) {
            Some(value) => {
                total = value;
            }
            None => {
                return None;
            }
        };
    }
    return Some(total);
}

fn main() {
    let numbers1: [u32; 5] = [1, 2, 3, 4, 5];
    match sum(&numbers1) {
        Some(value) => {
            println!("sum {value}");
        }
        None => {
            println!("overflow");
        }
    }
    let numbers2: [u32; 5] = [1, 2, 3, 4, 4294967294];
    match sum(&numbers2) {
        Some(value) => {
            println!("sum {value}");
        }
        None => {
            println!("overflow");
        }
    }
}
