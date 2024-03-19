use regex::Regex;

#[allow(dead_code)]
pub fn re_element_split<'a, 'b>(text: &'a str, node: &'b str) -> Option<String> {
    // 提取vct数据的属性内容
    let re = Regex::new(&("(".to_owned() + node + r"Begin)([\s\S]*)(" + node + "End)")).expect("");
    if let Some(caps) = re.captures(text) {
        println!("{:?}", caps);
        if caps.len() < 4 {
            return None;
        }
        let group = caps.get(2).map(|m| m.as_str().to_string());
        return group;
    }
    None
}

#[test]
fn test_reelementsplit() {
    let text = "123Beginsdadsa123End";
    let node = "123";
    let result = re_element_split(text, node);
    // assert_eq!(result,None);
    println!("{:?}", result)
}
