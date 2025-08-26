fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v2 = vec![1, 2, 3, 4];
    println!("v2: {:?}", v2);
    let second: &i32 = &v2[1];
    println!("The second element is {}", second);

    let second: Option<&i32> = v2.get(1);
    if let Some(&second) = second {
        println!("The second element is {}", second);
    }

    // let does_not_exist = &v2[100];
    let does_not_exist = v2.get(100);
    if let None = does_not_exist {
        println!("The does_not_exist is None.");
    }
}
