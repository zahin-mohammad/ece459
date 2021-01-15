use std::collections::HashMap;

fn fibonacci_number(n: u32) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 1,);

    // We do hm: &mut here because we want to modify the values of the original hm
    // If we do mut hm: .. we just make hm re-assignable which we do not want
    fn f(num: u32,  hm: &mut HashMap<u32, u32>) -> u32 {
        let ret: u32;
        match hm.get(&num) {
            Some(p) => {ret = *p}
            None => {
                // We should each number printed only once
                println!("Calling fib for {}", num);
                ret = f(num -1, hm) + f(num -2 , hm);
                hm.insert(num, ret);
            }
        }
        ret
    }
    return f(n, &mut map);
}

fn main() {
    println!("{}", fibonacci_number(10));
}
