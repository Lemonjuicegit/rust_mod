use crate::mods::read_gbk::read_gbk;
use crate::mods::regsearch::re_element_split;
use std::collections::HashMap;
use log::{info, trace, warn};
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
    feature_list: Vec<HashMap<String, String>>,
    feature_names: Vec<String>,
}

enum FieldType {
    CHAR,
    VARCHAR,
    INT,
    FLOAT,
    DATE,
}

struct Field {
    name: String,
    Field_type: FieldType,
    precision: u32,
    length: u32,
}
struct TableStructure {
    field: Vec<Field>,
    field_names: Vec<String>,
    field_vec: Vec<Vec<String>>,
}

struct Point {
    geo_type: String,
    x: f64,
    y: f64,
}

struct Line {
    geo_type: String,
    line_len: f64,
    xy: Vec<[f64; 2]>,
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

impl TableStructure {
    fn new(data: &String) {
        let tablestructure_vec = row_vec(&data, ",");
        let mut n = 0;

        loop {
            let stop = tablestructure_vec[n][1]
                .to_string()
                .parse::<usize>()
                .unwrap();
            for i in n+1..stop + n {

            }
            n += stop+1;
            if tablestructure_vec[n].len() == 2 {}
        }
    }
}

fn row_vec<'a>(data: &'a String, separator: &'a str) -> Vec<Vec<&'a str>> {
    let mut container = Vec::new();
    let res = data.split("\n").collect::<Vec<&str>>();
    for v in &res[1..res.len() - 1] {
        let split_str = v.split(separator).collect::<Vec<_>>();
        container.push(split_str);
    }
    container
}

impl Vct {
    fn new(path: &str) {
        let res: String = read_gbk(path).expect("读取文件失败");
        let head = re_element_split(&res, "Head");
        let featurecode_str = re_element_split(&res, "FeatureCode");
        let tablestructure_str = re_element_split(&res, "TableStructure");
        let line_str = re_element_split(&res, "Line");
        let polygon_str = re_element_split(&res, "Polygon");

        let mut headmap = HashMap::new();
        for v in row_vec(&head, ":") {
            headmap.insert(v[0].to_string(), v[1].to_string());
        }

        let featurecode_vec = row_vec(&featurecode_str, ",");
        let tablestructure_vec = row_vec(&tablestructure_str, ",");
        let line_vec = row_vec(&line_str, ",");
        let polygon_vec = row_vec(&polygon_str, ",");
        println!("{:?}", tablestructure_vec);
        // Vct {
        //     filePath: path.to_string(),
        //     comment: String::new(),
        //     head: Head {
        //         data_mark: String::new(),
        //         version: String::new(),
        //         coordinate_system_type: String::new(),
        //         dim: String::new(),
        //         x_axis_direction: String::new(),
        //         y_axis_direction: String::new(),
        //         xy_unit: String::new(),
        //         spheroid: String::new(),
        //         prime_meridian: String::new(),
        //         projection: String::new(),
        //         parameters: String::new(),
        //         vertical_datum: String::new(),
        //         temporal_reference_system: String::new(),
        //         extent_min: String::new(),
        //         extent_max: String::new(),
        //         map_mcale: String::new(),
        //         offset: String::new(),
        //         date: String::new(),
        //         deparator: String::new(),
        //     },
        //     feature_code: FeatureCode {
        //         feature_list: vec![HashMap::new()],
        //         feature_names: Vec::new(),
        //     },
        //     table_structure: TableStructure {
        //         field: vec![Field {
        //             name: "要素代码".to_string(),
        //             Field_type: FieldType::CHAR,
        //             precision: 0,
        //             length: 255,
        //         }],
        //         field_names: vec!["要素代码".to_string()],
        //     },
        //     point: Point {
        //         geo_type: "point".to_string(),
        //         x: 0.0,
        //         y: 0.0,
        //     },
        //     line: Line {
        //         geo_type: "line".to_string(),
        //         line_len: 0.0,
        //         xy: Vec::new(),
        //     },
        //     annotation: Annotation {},
        //     topology: Topology {},
        //     attribute: Attribute {},
        //     style: Style {},
        // }
    }
}

#[test]
fn test_vct() {
    let path = "E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GX.vct";
    Vct::new(path);
}
