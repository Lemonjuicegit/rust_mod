use std::collections::HashMap;
use crate::mods::regsearch::re_element_split;
use crate::mods::read_gbk::read_gbk;
struct Head {
    /**
    * 获取头信息:
    *  DataMark:<数据标识>
       Version:<版本号>
       CoordinateSystemType:<坐标系统类型>
       Dim:<坐标维数>
       XAxisDirection:<X坐标轴方向>
       YAxisDirection:<Y坐标轴方向>
       XYUnit:<平面坐标单位>
       Spheroid:<参考椭球>
       PrimeMeridian:<首子午线>
       Projection:<投影类型>
       Parameters:<投影参数>
       VerticalDatum:<高程基准>
       TemporalReferenceSystem:<时间参照系>
       ExtentMin:<最小坐标>
       ExtentMax:<最大坐标>
       MapScale:<比例尺>
       Offset:<坐标偏移量>
       Date:<土地规划批准时间>
       Separator:<属性字段分割符>
    */
    data_mark: String,
    version: String,
    coordinate_system_type: String,
    dim: String,
    x_axis_direction: String,
    y_axis_direction: String,
    xy_unit: String,
    spheroid: String,
    prime_meridian: String,
    projection: String,
    parameters: String,
    vertical_datum: String,
    temporal_reference_system: String,
    extent_min: String,
    extent_max: String,
    map_mcale: String,
    offset: String,
    date: String,
    deparator: String,
}

struct Attribute {}
struct FeatureCode {
    feature_codet_list: Vec<HashMap<String, String>>,
    feature_names: Vec<String>,
}

enum FiedlType {
    CHAR,
    VARCHAR,
    INT,
    FLOAT,
    DATE,
}

struct Field {
    name: String,
    fiedl_type: FiedlType,
    precision: u32,
    length: u32,
}
struct TableStructure {
    field: Vec<Field>,
    field_names: Vec<String>,
}

struct Point {
    geo_type: String,
    x: f64,
    y: f64,
}

struct Line {
    geo_type: String,
    line_len: f64,
    xy: Vec<Point>,
}
struct Polygon {
    geo_type: String,
    area: f64,
    centroid: Vec<Line>,
    bounds: [f64; 4],
}
struct Annotation {}
struct Topology {}
struct Style {}

pub struct Vct {
    filePath: String,
    comment: String,
    head: Head,
    feature_code: FeatureCode,
    table_structure: TableStructure,
    point: Point,
    line: Line,
    annotation: Annotation,
    topology: Topology,
    attribute: Attribute,
    style: Style,
}

impl Vct {
    fn new(path: &str) {
        let res = read_gbk(path).expect("读取文件失败");
        let head = re_element_split(&res, "Head");
        println!("head:{:?}", head);
    }

}

#[test]
fn test_vct() {
    let path = "E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GX.vct";
    Vct::new(path);
}
