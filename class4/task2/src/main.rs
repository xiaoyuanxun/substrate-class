fn get_sum(x: &[u32]) -> Option<u32> {
    let mut ans: u32 = 0u32;
    for i in x {
        let (num, check) = ans.overflowing_add(*i);
        if check {
            return None;
        };
        ans = num;
    }
    Some(ans)
}

fn main() {
    let array = vec![1, 2, 3];
    let ans = get_sum(&array);
    match ans {
        Some(ans) => println!("sum: {}", ans),
        None => println!("overflow"),
    }

    let array = vec![1, 2, 3, u32::MAX];
    let ans = get_sum(&array);
    match ans {
        Some(ans) => println!("sum: {}", ans),
        None => println!("overflow"),
    }
}
