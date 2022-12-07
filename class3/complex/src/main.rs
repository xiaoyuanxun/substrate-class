fn sort<T: PartialOrd + Copy>(array: &mut Vec<T>) -> &Vec<T> {
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
    array
}

fn main() {
    let mut array = vec![1, 34, 50, 200, 34, 51, 25, 100, 65];
    println!("{:?}",sort(&mut array));

    let mut array = vec!['D', 'e', 'A', 'C', 'a', 'W'];
    println!("{:?}",sort(&mut array));
}
