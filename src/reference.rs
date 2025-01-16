fn main(){
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}",s2);
    println!("{}",s1); // This is valid because of reference, The first pointer wasn't invalidated
}