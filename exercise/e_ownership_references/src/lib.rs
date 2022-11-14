pub fn inspect(s: &String){
    let status = if s.ends_with("s") {"plural"} else {"singular"};
    println!("{} is a {} word", s, status);
}

pub fn change(x: &mut String){
    if !x.ends_with("s"){
        x.push_str("s");
    }
}

pub fn eat(s: String)-> bool{
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s: &mut String){
    *s = "sparkely".to_string();
}