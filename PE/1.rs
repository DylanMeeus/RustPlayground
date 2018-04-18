

// first problem of PE
fn main(){
    let mut sum = 0;
    for number in 1..1000 {
        sum += if number % 3 == 0 || number % 5 == 0 {
            number
        } else {
            0
        }
    }
    println!("{}",sum);
}

