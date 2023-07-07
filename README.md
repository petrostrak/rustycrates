## Rust Web-API
An advanced Rocket REST API written in Rust that 
*  implements an authentication service
*  implements a cache database with Redis
*  tests API endpoints in an automated way
*  writes CLI commands with clap
*  manages database entities with complex fields and relationships using Diesel and PostgreSQL
*  sends HTML emails with lettre and tera
*  implements session handling and granular access management for different endpoints

Setup migrations via docker compose:
```bash
docker compose exec app diesel setup
```
Generate migrations via docker compose:
```bash
docker compose exec app diesel migration generate <table_name>
```
To execute migrations via docker compose:
```bash
docker compose exec app diesel migration run
```
To run the application:
```bash
docker compose exec app cargo run
```
