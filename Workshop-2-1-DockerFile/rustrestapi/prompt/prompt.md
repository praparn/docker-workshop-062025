Please provide rust code with Axum framework for high performance restful api server that connect to mariadb and redis cache with method below
1. Welcome page with method get on path /
    Return 200 with message: Welcome Page from Container Rust Lab
2. get user by id with method get on path /users/<uid>
    2.1 Connect to redis cache and check key of uid in redis
        2.1.1 if key exist in redis
            2.1.1.1 return name from redis and return 200 with message "########### Return from Redis ###########"
        2.1.2 if key not exist in redis
            2.1.1.2 get user from mariadb
                2.1.1.2.1 select name from users where id = uid
                2.1.1.2.2 if record not found return 404 back with message "########### Record not found ###########"
            2.1.1.3 save name to redis with expire key within 30s
            2.1.1.4 return name from redis and return 200 with message "########### Return from Redis ###########"
3. create user with method put on path /users/adduser
    3.1 verify user input on json format with 3 fields
        3.1.1 id
        3.1.2 name
        3.1.3 description
    3.2 insert user to mariadb
        3.2.1 connect to mariadb
        3.2.2 verify user id is already exist on database or not
        3.2.3 if user id not exist
            3.2.3.1 insert user to mariadb by sql insert into users (id, name, description) values (id, name, description) and return 200 with message "########### Record Added ###########"
        3.2.4 if user id exist
            3.2.4.1 return 409 back with message "########### Record Duplicate Already Exist ###########"
        
4. remove user with method delete on path /user/deluser/<uid>
    4.1 Connect to redis to check key is exist on redis or not ?
        4.1.1 if key exist, Delete key from redis and return message "########### Deleted from Cache ###########"
    4.2 Connect to mariadb to check user by uid
        4.2.1 if record is already exist, Delete record by uid and return 200 back with message "########### Deleted from Database ###########"
        4.2.2 if record is not found, return 404 back with message "########### Delete Record not found ###########"

5. initial database and table with method post on path /initial
    5.1 Connect to redis, purge all key on redis, return message "########### Initial Cache Done ###########"
    5.2 Connect to mariadb, drop database if exist and create database with table as detail below
        5.2.1 database name: accoutable
        5.2.2 table name: member
            5.2.2.1 field: id, type: int
            5.2.2.2 field: name, type: char(100)
            5.2.2.3 field: description, type: char(250)
    5.3 Return message "########### Initial Database Done ###########"

Remark:
    - MaridDB ServerName: MainDB, Username: memberadmin, Password: taleofdestiny, Connection Port: TCP-3306
    - Redis ServerName: CacheDB, Connection Port: TCP-6379
======================================================================================================
Notes
Environment Variables:
You can override the default connection strings by setting the environment variables MARIADB_URL and REDIS_URL before starting the application.

Database Initialization:
The /initial endpoint connects to MariaDB without a default database so that it can drop/create the accoutable database. After creating the database, it connects to it and creates the member table.

Error Handling:
For brevity, error messages are printed to standard error and a generic HTTP 500 is returned in most cases.

Compile and run the application (for example, with cargo run --release), and your API will listen on port 5000. Adjust hostnames (e.g. MainDB and CacheDB) as needed for your deployment environment.