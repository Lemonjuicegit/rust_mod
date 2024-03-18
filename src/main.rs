use std::time::Instant;
mod mods;
// use std::env;

fn main(){
    // 记录开始时间
    let start_time = Instant::now();
    // let file_path = "test.txt";
    let file_path = "E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GX.vct";
    let res = mods::read_gbk::read_gbk(file_path).expect("读取失败");
    println!("{}", res.len());
    // 记录结束时间
    let end_time = Instant::now();
    // 计算运行时间
    let elapsed_time = end_time - start_time;
    // 输出结果
    println!("代码运行时间: {:?}", elapsed_time);
}
