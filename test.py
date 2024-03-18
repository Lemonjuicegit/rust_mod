from target.release import rust_mod
from time import time

pathaa = "E:\\工作文档\\(500104)2023年度国土变更调查数据库更新成果\\更新数据包\\标准格式数据\\2001H2023500104GX.vct";



if __name__ == '__main__':
    t = time()
    # with open(pathaa,'r') as f:
    #     resul = f.read()
    # print(len(resul))
    res = rust_mod.read_rust(pathaa)
    print(res[:100])
    print(time() - t)