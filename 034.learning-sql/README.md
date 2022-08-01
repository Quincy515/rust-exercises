https://github.com/Liangdi/rust-demos

```bash
sea-orm-cli generate entity -v -u mysql://root:root1234@127.0.0.1:3306/sakila -o src/entity -t actor,category  --with-serde both
```

### The Sakila Database

```
sakila-schema.sql
sakila-data.sql
```

### 3.1 查询机制

#### SQL 语句

```bash
mysql> SELECT first_name, last_name
 	-> FROM customer
 	-> WHERE last_name = 'ZIEGLER';
Empty set (0.02 sec)
```

#### SeaORM

```rust
Customer::find()
    .select_only()
    .column(customer::Column::FirstName)
    .column(customer::Column::LastName)
    .filter(customer::Column::LastName.eq("ZIEGLER"))
    .into_model::<CustomerRes>()
    .all(db)
    .await?;
```

```sql
SELECT `customer`.`first_name`, `customer`.`last_name` FROM `customer` WHERE `customer`.`last_name` = 'ZIEGLER'
```

#### SQL 语句

```bash
mysql> SELECT *
 -> FROM category;
+-------------+-------------+---------------------+
| category_id | name | last_update |
+-------------+-------------+---------------------+
| 1 | Action | 2006-02-15 04:46:27 |
| 2 | Animation | 2006-02-15 04:46:27 |
| 3 | Children | 2006-02-15 04:46:27 |
| 4 | Classics | 2006-02-15 04:46:27 |
| 5 | Comedy | 2006-02-15 04:46:27 |
| 6 | Documentary | 2006-02-15 04:46:27 |
| 7 | Drama | 2006-02-15 04:46:27 |
| 8 | Family | 2006-02-15 04:46:27 |
| 9 | Foreign | 2006-02-15 04:46:27 |
| 10 | Games | 2006-02-15 04:46:27 |
| 11 | Horror | 2006-02-15 04:46:27 |
| 12 | Music | 2006-02-15 04:46:27 |
| 13 | New | 2006-02-15 04:46:27 |
| 14 | Sci-Fi | 2006-02-15 04:46:27 |
| 15 | Sports | 2006-02-15 04:46:27 |
| 16 | Travel | 2006-02-15 04:46:27 |
+-------------+-------------+---------------------+
16 rows in set (0.02 sec)
```



#### SeaORM

```rust
Category::find().all(db).await?;
```

```sql
SELECT `category`.`category_id`, `category`.`name`, `category`.`last_update` FROM `category`
```



### 3.3 Select ziju

#### SQL 语句

```bash
mysql> SELECT *
 -> FROM language;
+-------------+----------+---------------------+
| language_id | name | last_update |
+-------------+----------+---------------------+
| 1 | English | 2006-02-15 05:02:19 |
| 2 | Italian | 2006-02-15 05:02:19 |
| 3 | Japanese | 2006-02-15 05:02:19 |
| 4 | Mandarin | 2006-02-15 05:02:19 |
| 5 | French | 2006-02-15 05:02:19 |
| 6 | German | 2006-02-15 05:02:19 |
+-------------+----------+---------------------+
6 rows in set (0.03 sec)
```



#### SeaORM

```rust
Language::find().all(db).await?;
```

```sql
SELECT `language`.`language_id`, `language`.`name`, `language`.`last_update` FROM `language`
```



### 3.1 查询机制

#### SQL 语句

#### SeaORM





### 3.1 查询机制

#### SQL 语句

#### SeaORM



