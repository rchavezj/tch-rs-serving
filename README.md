# Actix (Rust) + GraphQL = Juniper

This project will contain a back end stack with rust web framework Actix for fast request performance. 

Remember to remove './todo-actix/target' folder before every git commit. </br>

# Docker commands
sudo docker-compose up -d </br>
sudo docker ps </br>

# PSQL Commands
psql -h 127.0.0.1 -p 5432 -U actix actix </br>
psql -h 127.0.0.1 -p 5432 -U actix actix < database.sql </br>


# Set of commands to setup server and retrieve data back to rust handler
(0) cargo run </br>
## Switch terminals
(1) sudo docker-compose up -d </br> 
(2) sudo docker ps </br>
(3) psql -h 127.0.0.1 -p 5432 -U actix actix </br>
    &nbsp;&nbsp;&nbsp; (3a List of relations)  \d  </br>
    &nbsp;&nbsp;&nbsp; (3b) select * from todo_item; </br>
    &nbsp;&nbsp;&nbsp; (3c) select * from todo_list; </br>
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/storedProcedureDisplay.png)


# If the user wishes to display the todos list
## curl commandds when program (server) is running (cargo run)
curl http://localhost:8080/ </br>
curl http://localhost:8080/todos </br>
curl http://localhost:8080/todos | jq . </br>
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/todoList.png)


# If the user wishes to display the items 
curl http://localhost:8080/todos/1/items </br>
curl http://localhost:8080/todos/1/items -s | jq . </br>
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/todoItems.png)

# If the user wishes to create a new todo
curl http://localhost:8080/todos </br>
curl http://localhost:8080/todos | jq . </br>
curl -X POST -H "Content-Type: application/json" -d '{"title": "List 3"}' http://localhost:8080/todos </br>
curl http://localhost:8080/todos </br>
curl http://localhost:8080/todos | jq . </br>
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/insertNewTodoList.png)


# Update content for put commands
curl http://localhost:8080/todos/2/items -s | jq . </br>
curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq . --> (Success true) </br>
curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq . --> (Success false) </br>
curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq . --> (Success false) </br>
curl http://localhost:8080/todos/2/items -s | jq . </br>
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/putFunction.png)


# Performance Tests (Local host)
(1) cargo build --release </br>
(2) cargo run --release </br>
(3) ab -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(4) ab -n 100000 -k -c 30 -q http://localhost:8080/todos </br>
(5) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(6) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/todos </br>


# Put the app into a container and limit the resources. 
## The idea is to simulate a small 'BM' in a cloud provider (Performance Test)
(1) sudo docker-compose --compatibility up </br>
(2) ab -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(3) ab -n 100000 -k -c 30 -q http://localhost:8080/todos </br>
(4) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(5) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/todos </br>
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/dockerComposeCompatibilityPt1.png) 
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/dockerComposeCompatibilityPt2.png)

