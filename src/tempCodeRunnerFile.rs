#[test]
fn test() {
    struct Res {
        data: str,
        data1: &str,
        data2: String,
    }
    let res = Res {
        data: "str",
        data1: "&str",
        data2: String::from("String"),
    };
}