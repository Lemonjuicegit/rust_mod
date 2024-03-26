use crate::mods::read_gbk::read_gbk;
use crate::mods::regsearch::{re, re_element_split};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
struct Attribute {
    tabel_name: String,
    data: Vec<Vec<String>>,
}

impl Attribute {
    fn new(Attdata: &String) -> Self {
        let Attribute_data = Attdata.split("\n").collect::<Vec<&str>>();
        Self {
            tabel_name: String::new(),
            data: Vec::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct FeatureCode {
    feature_list: Vec<HashMap<String, String>>,
    feature_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum FieldType {
    CHAR,
    VARCHAR,
    INT,
    FLOAT,
    DATE,
}

#[derive(Debug, Serialize, Deserialize)]
struct Field {
    name: String,
    Field_type: FieldType,
    precision: Option<u32>,
    length: Option<u32>,
}
#[derive(Debug, Serialize, Deserialize)]
struct TableStructure {
    field: HashMap<String, Vec<Field>>,
    table_names: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Point {
    geo_type: String,
    x: f64,
    y: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Line {
    geo_type: String,
    line_len: f64,
    bsm: String,      // 标识码
    ysdm: String,     // 要素代码
    txbxmb: String,   // 图形表现编码
    tzlx: String,     // 线的特征类型
    count: u32,       // 线段条数
    xdlx: String,     // 线段类型
    point_count: u32, // 点数
    xy: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Polygon {
    geo_type: String,
    area: f64,
    centroid: Vec<Line>,
    bsm: String,
    ysdm: String,
    txbxbm: String,
    tzlx: String,
    x: f64,
    y: f64,
    jjzbmgclx: String,
    count: u32,
    dxbsm: Vec<String>,
    // bounds: [f64; 4],
}
#[derive(Serialize, Deserialize)]
struct Annotation {}
#[derive(Serialize, Deserialize)]
struct Topology {}
#[derive(Serialize, Deserialize)]
struct Style {}

#[derive(Serialize, Deserialize)]
pub struct Vct {
    filePath: String,
    comment: String,
    head: Head,
    feature_code: FeatureCode,
    table_structure: TableStructure,
    point: Point,
    line: Vec<Line>,
    polygon: Vec<Polygon>,
    annotation: Annotation,
    topology: Topology,
    attribute: Attribute,
    style: Style,
}

impl TableStructure {
    fn new(data: &String) -> Self {
        let tablestructure_vec = row_vec(&data, ",");
        let mut n = 0;
        let mut tablestructure = TableStructure {
            field: HashMap::new(),
            table_names: Vec::new(),
        };

        while n <= tablestructure_vec.len() - 1 {
            let mut fields = Vec::new();
            let stop = tablestructure_vec[n][1]
                .to_string()
                .parse::<usize>()
                .unwrap();
            let tablename = tablestructure_vec[n][0];
            tablestructure
                .table_names
                .push(tablestructure_vec[n][0].to_string());
            for i in n + 1..stop + n + 1 {
                let field = match tablestructure_vec[i].len() {
                    2 => Field {
                        name: tablestructure_vec[i][0].to_string(),
                        Field_type: FieldType::VARCHAR,
                        precision: None,
                        length: None,
                    },
                    3 => Field {
                        name: tablestructure_vec[i][0].to_string(),
                        Field_type: match tablestructure_vec[i][1] {
                            "Char" => FieldType::CHAR,
                            "Int" => FieldType::INT,
                            "Date" => FieldType::DATE,
                            _ => FieldType::CHAR,
                        },
                        precision: None,
                        length: Some(
                            u32::from_str_radix(tablestructure_vec[i][2], 10)
                                .expect("字段长度设置错误"),
                        ),
                    },
                    4 => Field {
                        name: tablestructure_vec[i][0].to_string(),
                        Field_type: FieldType::VARCHAR,
                        precision: Some(
                            u32::from_str_radix(tablestructure_vec[i][2], 10)
                                .expect("字段精度设置错误"),
                        ),
                        length: Some(
                            u32::from_str_radix(tablestructure_vec[i][3], 10)
                                .expect("字段长度设置错误"),
                        ),
                    },
                    _ => Field {
                        name: String::new(),
                        Field_type: FieldType::CHAR,
                        precision: None,
                        length: None,
                    },
                };
                fields.push(field);
            }
            tablestructure
                .table_names
                .push(tablestructure_vec[n][0].to_string());
            tablestructure.field.insert(tablename.to_string(), fields);
            n += stop + 2;
        }
        tablestructure
    }
}

impl Line {
    fn new(data: &[&str]) -> Self {
        let mut line = Line {
            geo_type: "line".to_string(),
            line_len: 0.0,
            bsm: data[0].to_string(),
            ysdm: data[1].to_string(),
            txbxmb: data[2].to_string(),
            tzlx: data[3].to_string(),
            count: u32::from_str_radix(data[4], 10).unwrap(),
            xdlx: data[5].to_string(),
            point_count: u32::from_str_radix(data[6], 10).unwrap(),
            xy: Vec::new(),
        };
        for i in 7..line.point_count + 6 {
            let xy = data[i as usize]
                .split(",")
                .map(|v| v.parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            line.xy.push(xy);
        }
        line
    }
}

fn row_vec<'a>(data: &'a String, separator: &'a str) -> Vec<Vec<&'a str>> {
    let mut container = Vec::new();
    let res = data.split("\n").filter(|&v| v != "").collect::<Vec<&str>>();
    for v in &res[0..res.len() - 1] {
        let split_str = v.split(separator).collect::<Vec<_>>();
        container.push(split_str);
    }
    container
}

impl Vct {
    pub fn new(path: &str) -> Self {
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
        let tablestructure = TableStructure::new(&tablestructure_str);

        let mut vct = Self {
            filePath: path.to_string(),
            comment: String::new(),
            head: Head {
                data_mark: String::new(),
                version: String::new(),
                coordinate_system_type: String::new(),
                dim: String::new(),
                x_axis_direction: String::new(),
                y_axis_direction: String::new(),
                xy_unit: String::new(),
                spheroid: String::new(),
                prime_meridian: String::new(),
                projection: String::new(),
                parameters: String::new(),
                vertical_datum: String::new(),
                temporal_reference_system: String::new(),
                extent_min: String::new(),
                extent_max: String::new(),
                map_mcale: String::new(),
                offset: String::new(),
                date: String::new(),
                deparator: String::new(),
            },
            feature_code: FeatureCode {
                feature_list: vec![HashMap::new()],
                feature_names: Vec::new(),
            },
            table_structure: tablestructure,
            point: Point {
                geo_type: "point".to_string(),
                x: 0.0,
                y: 0.0,
            },
            line: Vec::new(),
            polygon: Vec::new(),
            annotation: Annotation {},
            topology: Topology {},
            attribute: Attribute::new(&String::new()),
            style: Style {},
        };
        vct.set_line(&line_str);
        vct.set_polygon(&polygon_str);
        vct
    }
    fn set_line(&mut self, line_str: &String) {
        let mut lines = Vec::new();
        let res = line_str
            .split("\n")
            .filter(|v| *v != "")
            .collect::<Vec<&str>>();
        let mut n = 0;

        while n < res.len() {
            let stop = res[n + 6].parse::<usize>().unwrap();
            lines.push(Line::new(&res[n..n + stop + 6]));
            n += stop + 8;
        }
        self.line = lines;
    }
    fn set_polygon(&mut self, polygon_str: &String) {
        let polygon_vec = polygon_str
            .split("\n")
            .filter(|v| *v != "")
            .collect::<Vec<&str>>();

        let mut n = 0;
        while n < polygon_vec.len() {
            let polygon_vec_slice = Vec::from(&polygon_vec[n..n + 7]);
            n += 7;
            let line_bsm = polygon_vec[n];
            let mut jx_bsm = polygon_vec[n].split(",0,").collect::<Vec<&str>>();
            while polygon_vec[n + 1] != "0" {
                n += 1;
                polygon_vec[n].split(",0,").for_each(|v| jx_bsm.push(v));
            }
            n += 2;
            let xy_split = polygon_vec_slice[4].split(",").collect::<Vec<_>>();
            self.polygon.push(Polygon {
                geo_type: "polygon".to_string(),
                area: 0.0,
                centroid: self
                    .line
                    .iter()
                    // .filter(|v|&v.bsm == line_bsm)
                    .filter(|v| jx_bsm.contains(&v.bsm.as_str()))
                    .cloned()
                    .collect::<Vec<Line>>(),
                bsm: polygon_vec_slice[0].to_string(),
                ysdm: polygon_vec_slice[1].to_string(),
                txbxbm: polygon_vec_slice[2].to_string(),
                tzlx: polygon_vec_slice[3].to_string(),
                x: xy_split[0].parse::<f64>().unwrap(),
                y: xy_split[1].parse::<f64>().unwrap(),
                jjzbmgclx: polygon_vec_slice[5].to_string(),
                count: polygon_vec_slice[6].parse::<u32>().unwrap(),
                dxbsm: jx_bsm.iter().map(|v| String::from(*v)).collect(),
            });
        }
    }

    pub fn json_str(&self) -> String {
        let jsondata = serde_json::to_string(self).unwrap();
        jsondata
    }
}

#[test]
fn test_vct() {
    let path = "E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GXGC - 副本.vct";
    let mut vct = Vct::new(path);
    let aa = &vct.polygon[100];
    println!("{:?}", aa);
}
