fn odd_man_out(array: &[i32]) ->i32 {
    let mut val: i32 = 0;
    for el in array{
        val = val ^ el;
    }
    return val;
}

fn main(){
    let array: [i32; 7 ] = [1, -7, 5, -7, -3, 5, 1];
    let val: i32 = odd_man_out(&array);
    println!("{}", val);
}