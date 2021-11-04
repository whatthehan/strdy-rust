fn main() {
    // let mut tuple: (u32, bool) = (1, true);
    // tuple.0 = 2;
    //
    // println!("variable is {};", tuple.0);

    let nums = [1, 2, 3, 4, 5];
    for item in nums.iter() {
        println!("item is {}", item)
    }
}
