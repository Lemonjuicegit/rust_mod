use regex::Regex;

#[allow(dead_code)]
pub fn re_element_split<'a,'b>(text:&'a str,node: &'b str,)->&'a str{
    let re = Regex::new(&("(".to_owned() + node + r"Begin)([\s\S]*)(" + node + "End)")).expect("");
    let captures: Vec<_> = re.find_iter(text).map(|mat| mat.as_str()).collect();
    captures[0]
}

#[test]
fn test_reelementsplit() {
    let text = "123Beginewrwe123End";
    let node = "123";
    let result = re_element_split(text,node);
    println!("{result}")
}