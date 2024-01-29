docker build -t gcr.hrz.tu-chemnitz.de/praktikum-scc/ws23-gruppe1/gruppe1/bucket_list:0.2.0 -f ./services/bucket_list/Dockerfile .
docker build -t gcr.hrz.tu-chemnitz.de/praktikum-scc/ws23-gruppe1/gruppe1/user:0.1.0 -f ./services/user/Dockerfile .
docker build -t gcr.hrz.tu-chemnitz.de/praktikum-scc/ws23-gruppe1/gruppe1/records:v1.f.a.5 -f ./services/records/Dockerfile .
