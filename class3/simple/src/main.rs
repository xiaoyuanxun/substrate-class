use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) =>{
            panic!("请输入数组大小数字");
        }
    };

    let mut array = String::new();
    io::stdin().read_line(&mut array).unwrap();
    let mut array: Vec<i32> = array.trim().split(' ').map(|x| x.parse().unwrap()).collect();

    if u32::try_from(array.len()).unwrap() != n {
        panic!("请确保数组大小正确");
    }

    let mut flag = true;
    while flag {
        flag = false;
        for i in 0..array.len() - 1 {
            if array[i] > array[i+1] {
                array.swap(i, i+1);
                flag = true;
            }
        }
    }

    println!("排序后的数组为：");
    println!("{:?}", array);
}