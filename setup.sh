cd ./services/bucket_list
docker build -t bucket_list:latest .
cd ../services/user
docker built -t user:latest .
cd ../services/records
docker built -t records:latest .