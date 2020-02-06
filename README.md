# Rust App Boilerplate

1. support log4rs
2. support config file
3. support handlebars as template
4. support front and back-end separation(insert html/js in binary)


## dir/file structure
- config
     
      config files
      
- static

      template files
      
- public

      React/Vue/Angular files
     
- src

      rust files

- log

      log files
            
- Cargo.toml


## Diesel

diesel setup --database-url=mysql://app:app@localhost:3306/app

diesel migration generate create_users

diesel migration run --database-url=mysql://app:app@localhost:3306/app

diesel migration redo --database-url=mysql://app:app@localhost:3306/app
