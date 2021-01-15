// You should implement the following function:

fn sum_of_multiples(number: i32, multiple1: i32, multiple2: i32) -> i32
{
    let mut sum: i32 = 0;
    for x in 0..number {
        if x % multiple1 == 0 || x % multiple2 == 0 {
            sum +=x;
        }
    }
    sum
}

fn main() {
    println!("{}", sum_of_multiples(1000, 5, 3));
}
