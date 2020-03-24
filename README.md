# rust-practice
Some rust pracice


### road map:
#### practice
    1 get remote json data to mysql
        
        mysql-connector-c seems to be the homebrew package you are looking for. Libmysqlclient should also be installed when installing the mysql brew package.
        
        docker run -p 3306:3306 --name mysql1 -e MYSQL_ROOT_PASSWORD=secret -d mysql:latest --default-authentication-plugin=mysql_native_password
        
        test diesel
            
            diesel migration run , cli will build schema.cs automatically.
        
    2 web api apply to some biz flow
    3 cross platform compile
    4 web assembly
    
    best practice
    
        how to handle exception , use ? or expect()
        
            https://lotabout.me/2017/rust-error-handling/
        
        walk macro code of diesel
            connection pool
            macro design
        
        design of Async future package

#### concept:
    1 macro learning

