fn main() {
    let mut a = String::from("qwe");
    let b = &a;
    let d = &a;
    let c = &mut a;


    println!("result: {}", a);
    println!("result: {}", a);
}

fn test(s: &String) -> String {
    let a = &(*s);
    return (*a);
}