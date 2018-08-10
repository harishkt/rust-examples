fn is_prime(i: &u32) -> bool {
    let mut temp: u32 = 2;
    while i > &temp {
        if i % &temp == 0 {
            return false;
        }
        temp += 1;
    }
    true
}

pub fn nth(n: u32) -> Option<u32> {
    let mut init_vec: Vec<u32>  = Vec::new();
    let mut i: u32 = 2;
    let mut counter: u32 = 0;

    if n == 0 {
        return None;
    }
    loop {
        if is_prime(&i) {
            counter += 1;
            init_vec.push(i);
        }
        i += 1;
        if counter == n {
            break;
        }
    }
    init_vec.pop()
}
