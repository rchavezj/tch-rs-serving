# rust_graphql

This project will contain a back end stand with rust web framework Actix for fast request performance. 


docker-compose up -d
docker ps 

psql -h 127.0.0.1 -p 5432 -U actix actix 
psql -h 127.0.0.1 -p 5432 -U actix actix < database.sql

curl http://127.0.0.1:5432/
curl http://127.0.0.1:5432/todos
curl http://127.0.0.1:5432/todos | jq .

(0) cargo run
(1) sudo docker-compose up -d
(2) sudo docker ps 
(3) psql -h 127.0.0.1 -p 5432 -U actix actix