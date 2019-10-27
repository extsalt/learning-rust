fn main() {
 let s = String::from("hello world");

//since s is being moved, s1 can access only through reference.
 let s1 = &s ;

 println!("{}",s );
}
