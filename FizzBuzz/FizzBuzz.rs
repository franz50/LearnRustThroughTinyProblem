fn main(){
    for index in 1 .. 100 {

        let div_by3 = (index % 3) == 0;
        let div_by5 = (index % 5) == 0;
        let div_by_both = div_by3 && div_by5;

        if div_by_both {
            println!("FizzBuzz");
        }
        else if div_by3{
            println!("Fizz");
        }
        else if div_by5{
            println!("Buzz");
        }
        else{
            println!("{}", index);
        }
        
    }
}