fn main() {
    let sentence=String::from("Hello World");
    let first_word = get_first_word(sentence);
    println!("First word is: {}",first_word);
    let a: i8=12;
    let b:i8=123;
    let sum = do_sum(a,b);
    println!("Sum of {} and {} is: {}",a,b,sum);
}
fn get_first_word(sentence:String)->String{
    let mut first_word = String::new();
    for c in sentence.chars(){
        if c == ' '{
            break;
        }
        first_word.push(c);
    }
    return first_word;
}
fn do_sum(a:i8, b:i8)->i8{
    return a+b;
}