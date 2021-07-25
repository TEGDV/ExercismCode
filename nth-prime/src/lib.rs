pub fn nth(n: u32) -> u32 {
    let mut nth_list = Vec::from([2u32, 3]);
    let mut number = nth_list.last().unwrap().clone() + 2;
    let mut is_prime: bool;
    while nth_list.len() as u32 <= n {
        is_prime = true;

        for nth in nth_list.iter() {
            if nth * nth > number {
                break;
            }
            if number % nth == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            nth_list.push(number);
        }
        number += 2;
    }
    println!("{:?}", nth_list);
    nth_list[n as usize]
}
