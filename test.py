from target.release import rust_mod
from time import time
import geopandas as gpd
import json
pathaa = "E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GXGC.vct";
aa = json.loads(rust_mod.fnvct(pathaa))

print(aa)
 


if __name__ == '__main__':
    # t = time()
    # with open(pathaa,'r') as f:
    #     resul = f.read()
    # print(len(resul))
    # res = rust_mod.read_rust(pathaa)
    # print(res[:100])
    # print(time() - t)
    gdf = gpd.read_file(r"E:\工作文档\大足所有权\大足-古龙镇\数据库\JZXshp.shp")
    crs = gdf.crs
    pass