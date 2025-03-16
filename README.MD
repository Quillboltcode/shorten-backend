Dump in plain SQL format:

```sh
docker exec -t <container_id_or_name> pg_dump -U <your_db_user> -d <your_db_name> -f /tmp/db_backup.sql
```

Copy the SQL file to your local machine:

```sh
docker cp <container_id_or_name>:/tmp/db_backup.sql ./db_backup.sql
```
Import into local PostgreSQL:

```sh
psql -U <your_local_db_user> -d <your_local_db_name> -f db_backup.sql
```

## Command to rebuild and watch
```sh
docker compose up -d --no-deps shortener-service
```