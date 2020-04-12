fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    //imperative approach
    //declare accumulator variable
    let mut acc = 0;
    //iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        //square the number
        let n_squared = n * n;

        if n_squared >= upper {
            //break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            //accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    //functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) //all natural numbers squared
        .take_while(|&n_squared| n_squared < upper) //below upper limit
        .filter(|&n_squared| is_odd(n_squared)) //that are odd
        .sum(); //sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
