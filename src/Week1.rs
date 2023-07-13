fn concatenate_strings(string1:&str, string2:&str) -> String{
    let mut result = "";
    result.push_str(string1);
    result.push_str(string2);
    result;
}


fn main() {
    let string1 = "icey";
    let string2 = "spicey";
    let concatenated_string = concatenate_strings(string1, string2);
    println!("{}",concatenated_strings);
}

