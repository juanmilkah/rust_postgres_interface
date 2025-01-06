# rust_postgres_interface

```bash
DATABASE_URL="pstgres://database_user:password@localhost/database_name"
```

## Create Users table
```sql
CREATE TABLE IF NOT EXISTS users( id SERIAL PRIMARY KEY, name TEXT, email TEXT);
```

## Licensing
Project is licensed under the [GNU GPL3](LICENSE). 
