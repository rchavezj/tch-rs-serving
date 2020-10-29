# Actix (Rust) + GraphQL = Juniper

This project will contain a back end stack with rust web framework Actix for fast request performance. 

# Docker commands
sudo docker-compose up -d </br>
sudo docker ps </br>

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

# If the user wishes to display the todos list
(4) curl http://127.0.0.1:8080/todos </br>
(5) curl http://127.0.0.1:8080/todos | jq . </br>


# If the user wishes to display the items 
curl http://127.0.0.1:8080/todos/1/items </br>
curl http://127.0.0.1:8080/todos/1/items -s | jq . </br>


# If the user wishes to create a new todo
curl -X POST -H "Content-Type: application/json" -d '{"title": "List 3"}' http://127.0.0.1:8080/todos </br>


# Update content for put commands
curl http://127.0.0.1:8080/todos/2/items -s | jq . </br>
curl -X PUT http://127.0.0.1:8080/todos/2/items/3 -s | jq . --> (Success true) </br>
curl -X PUT http://127.0.0.1:8080/todos/2/items/3 -s | jq . --> (Success false) </br>
curl -X PUT http://127.0.0.1:8080/todos/2/items/3 -s | jq . --> (Success false) </br>
curl http://127.0.0.1:8080/todos/2/items -s | jq . 


# Performance Tests
cargo build --release
ab -n 100000 -k -c 30 -q http://127.0.0.1:8080/