# Actix (Rust) + GraphQL = Juniper

This project will contain a back end stand with rust web framework Actix for fast request performance. 

# Docker commands
docker-compose up -d </br>
docker ps </br>

# PSQL Commands
psql -h 127.0.0.1 -p 5432 -U actix actix </br>
psql -h 127.0.0.1 -p 5432 -U actix actix < database.sql </br>

# curl commandds when program (server) is running (cargo run)
curl http://127.0.0.1:8080/ </br>
curl http://127.0.0.1:8080/todos </br>
curl http://127.0.0.1:8080/todos | jq . </br>


# Set of commands to setup server and retrieve data back to rust handler
(0) cargo run </br>
(1) sudo docker-compose up -d </br> 
(2) sudo docker ps </br>
(3) psql -h 127.0.0.1 -p 5432 -U actix actix </br>
    &nbsp;&nbsp;&nbsp; (3a List of relations)  \d  </br>
    &nbsp;&nbsp;&nbsp; (3b) select * from todo_item; </br>
    &nbsp;&nbsp;&nbsp; (3c) select * from todo_list; </br>
(4) curl http://127.0.0.1:8080/todos </br>
(5) http://127.0.0.1:8080/todos | jq . </br>