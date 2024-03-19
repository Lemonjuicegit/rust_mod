#[test]
fn test() {
    struct Test{
        a: String,
        c:String,
    }
    let t = Test {
        a: "hello".to_string(),
        c: "rust".to_string(),
    };
    println!("{}",t.a)
}