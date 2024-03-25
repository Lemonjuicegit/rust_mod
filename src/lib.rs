use pyo3::prelude::*;
mod mods;
mod vct;
pub mod test;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_rust(path:&str) -> PyResult<String> {
    let res = mods::read_gbk::read_gbk(path)?;
    Ok(res)
}
#[pyfunction]
fn fnvct(path:&str) -> PyResult<String> {
    let res = vct::parsevct::Vct::new(path);
    Ok(res.json_str())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_rust, m)?)?;
    m.add_function(wrap_pyfunction!(fnvct, m)?)?;
    Ok(())
}

#[test]
fn test(){
    let res = read_rust("E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GX.vct").expect("读取失败");
    println!("{}",res.len())
}