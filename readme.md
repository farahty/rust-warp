## Start Database 

```sh
# Start the database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql 
docker exec -it -u postgres pg psql
```


## Start dev mode
```sh
cargo watch -q -c -w src/ -x run

```