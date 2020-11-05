# Actix (Rust) + GraphQL = Juniper

This project will contain a back end stack with rust web framework Actix for fast request performance. Alongside Juniper, an integration of Rust with GraphQL

Remember to remove './todo-actix/target' folder before every git commit. </br>

# Docker commands
sudo docker-compose up -d </br>
sudo docker ps </br>
sudo docker-compose up -d postgres </br>

# PSQL Commands
psql -h 127.0.0.1 -p 5432 -U actix actix </br>
psql -h 127.0.0.1 -p 5432 -U actix actix < database.sql </br>


# Set of commands to setup server and retrieve data back to rust handler
(0) cargo run </br>
### Switch terminals
(1) sudo docker-compose up -d </br> 
(2) sudo docker ps </br>
(3) psql -h 127.0.0.1 -p 5432 -U actix actix </br>
    &nbsp;&nbsp;&nbsp; (3a List of relations)  \d  </br>
    &nbsp;&nbsp;&nbsp; (3b) select * from todo_item; </br>
    &nbsp;&nbsp;&nbsp; (3c) select * from todo_list; </br>
![alt text](https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/storedProcedureDisplay.png)


# If the user wishes to display the todos list
### curl commandds when program (server) is running (cargo run)
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
<img src="https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/POSTNewTodoList.png" width="820" height="500" /> 


# Update content for put commands
curl http://localhost:8080/todos/2/items -s | jq . </br>
curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq . --> (Success true) </br>
curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq . --> (Success false) </br>
curl -X PUT http://localhost:8080/todos/2/items/3 -s | jq . --> (Success false) </br>
curl http://localhost:8080/todos/2/items -s | jq . </br>
<img src="https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/putFunction.png" width="820" height="500" /> 


# Performance Tests (Local host)
(1) cargo build --release </br>
(2) cargo run --release </br>
(3) ab -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(4) ab -n 100000 -k -c 30 -q http://localhost:8080/todos </br>
(5) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(6) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/todos </br>


# Put the app into a container and limit the resources. 
### The idea is to simulate a small 'BM' in a cloud provider (Performance Test)
(1) sudo docker-compose --compatibility up </br>
<img src="https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/dockerComposeCompatibility.png"/>   </br>
(2) ab -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(3) ab -n 100000 -k -c 30 -q http://localhost:8080/todos </br>
(4) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/ </br>
(5) ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/todos </br>
<img src="https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/dockerReleasePt1.png" width="820" height="400" />  </br>

<img src="https://github.com/rchavezj/rust_graphql/blob/main/todo-actix/img/dockerReleasePt2.png" width="820" height="400" />  </br>


# Error handling and Logging (Create appropriate msg to user)
### Series of commands to see why we need error handling
(0) cargo run --> Switch terminal tab </br>
(1) curl localhost:8080/todos -v | jq . </br>
(2) docker-compose up -d postgres </br>
(3) curl localhost:8080/todos -v | jq . </br>
(4) psql -h 127.0.0.1 -p 5432 -U actix actix </br>
(5) docker-compose stop </br>
(6) curl localhost:8080/todos -v | jq . --> curl (52) Empty reply from server </br>
### Switch terminal tabs back to restart cargo run but with trace env variable (RUST_BACKREACE=1) 
(7) RUST_BACKREACE=1 cargo run (Switch terminal tabs again) </br>
<<<<<<< HEAD
(8) curl localhost:8080/todos -v | jq . --> Uglier stack trace. Difficult finding the root to error.  </br>

=======
(8) curl localhost:8080/todos -v | jq . --> Uglier stack trace </br>
>>>>>>> 3facf1b01b30c84ba8622facd0c4cc7528871c40
