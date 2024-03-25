use regex::Regex;
use crate::mods::read_gbk::read_gbk;

#[allow(dead_code)]
pub fn re_element_split(text: & str, node: & str) -> String {
    // 提取vct数据的属性内容
    let re = Regex::new(&("(".to_owned() + node + r"Begin)([\s\S]*)(" + node + "End)")).expect("匹配错误");
    if let Some(caps) = re.captures(text) {
        if caps.len() < 4 {
            return String::new();
        }
        let group = caps.get(2).map(|m| m.as_str().to_string());
        return match group {
            Some(s)=> s,
            None => String::new(),
        }
    }
    String::new()
}

#[test]
fn test_reelementsplit() {
    let path = "E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GX.vct";
    let res: String = read_gbk(path).expect("读取文件失败");
    let node = "Line";
    let result = re_element_split(&res, node);
    // assert_eq!(result,None);
    println!("{:?}", result)
}
