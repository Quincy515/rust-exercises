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
#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
}

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

>**Note**
>
>- `eq`
>- `select_only` `column`
>- `into_model::<CustomerRes>()`

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



### 3.3 `SELECT` 子句

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

#### SQL 语句

```bash
mysql> SELECT language_id, name, last_update
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
6 rows in set (0.00 sec)
```

#### SeaORM

```rust

#[derive(Debug, FromQueryResult)]
struct LanguageRes {
    language_id: u8,
    name: String,
    last_update: DateTimeUtc,
}

pub async fn get_language_column(db: &DatabaseConnection) -> Result<()> {
    let language = Language::find()
        .select_only()
        .column(language::Column::LanguageId)
        .column(language::Column::Name)
        .column(language::Column::LastUpdate)
        .into_model::<LanguageRes>()
        .all(db)
        .await?;
    println!("{:?}", language);
    Ok(())
}
```

```sql
SELECT `language`.`language_id`, `language`.`name`, 
`language`.`last_update` FROM `language`
```

### 3.3.1 列的别名

#### SQL 语句

```bash
mysql> SELECT language_id,
 -> 'COMMON' language_usage,
 -> language_id * 3.1415927 lang_pi_value,
 -> upper(name) language_name
 -> FROM language;
+-------------+----------------+---------------+---------------+
| language_id | language_usage | lang_pi_value | language_name |
+-------------+----------------+---------------+---------------+
| 1 | COMMON | 3.1415927 | ENGLISH |
| 2 | COMMON | 6.2831854 | ITALIAN |
| 3 | COMMON | 9.4247781 | JAPANESE |
| 4 | COMMON | 12.5663708 | MANDARIN |
| 5 | COMMON | 15.7079635 | FRENCH |
| 6 | COMMON | 18.8495562 | GERMAN |
+-------------+----------------+---------------+---------------+
6 rows in set (0.04 sec)
```

#### SeaORM

```rust
 #[derive(Debug, FromQueryResult)]
struct LanguageAsRes {
    language_id: u8,
    lang_pi_value: u8,
    language_name: String,
}
pub async fn get_language_column_as(db: &DatabaseConnection) -> Result<()> {
    let language = Language::find()
        .select_only()
        .column(language::Column::LanguageId)
        .column_as(language::Column::Name, "language_name")
        .column_as(
            Expr::col(language::Column::LanguageId).mul(3),
            "lang_pi_value",
        )
        .into_model::<LanguageAsRes>()
        .all(db)
        .await?;
    println!("{:?}", language);
    Ok(())
}
```

```sql
SELECT `language`.`language_id`, `language`.`name` AS `language_name`, `language_id` * 3 AS `lang_pi_value` FROM `language`
```

> **Note**
>
> - `Expr::col(language::Column::LanguageId).mul(3)`
> - `select_only`
> - `column_as`
>
> **Warning** 
>
> - u8 * 3.1415927 
> - upper() 函数

### 3.3.2 移除重复数据

#### SQL 语句

```bash
mysql> SELECT actor_id FROM film_actor ORDER BY actor_id;
+----------+
| actor_id |
+----------+
| 1 |
| 1 |
| 1 |
| 1 |
| 1 |
| 1 |
| 1 |
| 1 |
| 1 |
| 1 |
...
| 200 |
| 200 |
| 200 |
| 200 |
| 200 |
| 200 |
| 200 |
| 200 |
| 200 |
+----------+
5462 rows in set (0.01 sec)
```

在 `SELECT` 后面使用 `DISTINCT` 关键字来移除重复数据

```bash
mysql> SELECT DISTINCT actor_id FROM film_actor ORDER BY actor_id;
+----------+
| actor_id |
+----------+
| 1 |
| 2 |
| 3 |
| 4 |
| 5 |
| 6 |
| 7 |
| 8 |
| 9 |
| 10 |
...
| 192 |
| 193 |
| 194 |
| 195 |
| 196 |
| 197 |
| 198 |
| 199 |
| 200 |
+----------+
200 rows in set (0.01 sec)
```

#### SeaORM

```rust
#[derive(Debug, FromQueryResult)]
struct ActorRes {
    actor_id: u16,
}
pub async fn get_actor_id(db: &DatabaseConnection) -> Result<()> {
    let actor = Actor::find()
        .select_only()
        .column(actor::Column::ActorId)
        .order_by(actor::Column::ActorId, Order::Desc)
        .into_model::<ActorRes>()
        .all(db)
        .await?;
    println!("{:?}", actor);
    Ok(())
}
```

```sql
SELECT `actor`.`actor_id` FROM `actor` ORDER BY `actor`.`actor_id` DESC
```

> **Note**
>
> - `order_by`
>
> **Warning** 
>
> 在 `SELECT` 后面使用 `DISTINCT` 关键字来移除重复数据

### 3.4 `FROM` 子句

### 3.4.1 数据表

#### SQL 语句

```bash
mysql> SELECT concat(cust.last_name, ', ', cust.first_name) full_name
 -> FROM
 -> (SELECT first_name, last_name, email
 -> FROM customer
 -> WHERE first_name = 'JESSIE'
 -> ) cust;
+---------------+
| full_name |
+---------------+
| BANKS, JESSIE |
| MILAM, JESSIE |
+---------------+
2 rows in set (0.00 sec)
```

#### SeaORM

> **Warning**
>
> - `concat()`
> - subquery

### 3.4.2 数据表链接

#### SQL 语句

```bash
mysql> SELECT customer.first_name, customer.last_name,
 -> time(rental.rental_date) rental_time
 -> FROM customer
 -> INNER JOIN rental
 -> ON customer.customer_id = rental.customer_id
 -> WHERE date(rental.rental_date) = '2005-06-14';
+------------+-----------+-------------+
| first_name | last_name | rental_time |
+------------+-----------+-------------+
| JEFFERY | PINSON | 22:53:33 |
| ELMER | NOE | 22:55:13 |
| MINNIE | ROMERO | 23:00:34 |
| MIRIAM | MCKINNEY | 23:07:08 |
| DANIEL | CABRAL | 23:09:38 |
| TERRANCE | ROUSH | 23:12:46 |
| JOYCE | EDWARDS | 23:16:26 |
| GWENDOLYN | MAY | 23:16:27 |
| CATHERINE | CAMPBELL | 23:17:03 |
| MATTHEW | MAHAN | 23:25:58 |
| HERMAN | DEVORE | 23:35:09 |
| AMBER | DIXON | 23:42:56 |
| TERRENCE | GUNDERSON | 23:47:35 |
| SONIA | GREGORY | 23:50:11 |
| CHARLES | KOWALSKI | 23:54:34 |
| JEANETTE | GREENE | 23:54:46 |
+------------+-----------+-------------+
16 rows in set (0.01 sec)
```

#### SeaORM

```rust
#[derive(Debug, FromQueryResult)]
struct CustomerTableRes {
    first_name: String,
    last_name: String,
    rental_time: DateTime,
}
pub async fn subquery_customer_table_link(db: &DatabaseConnection) -> Result<()> {
    let customer =
        Customer::find()
            .select_only()
            .column(customer::Column::FirstName)
            .column(customer::Column::LastName)
            .column_as(
                Expr::tbl(Alias::new("rental"), rental::Column::RentalDate).into_simple_expr(),
                "rental_time",
            )
            .inner_join(Rental)
            .filter(Condition::all().add(
                Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-16"),
            ))
            // .build(DbBackend::MySql)
            // .to_string();
            .into_model::<CustomerTableRes>()
            .all(db)
            .await?;
    println!("{:?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `customer`.`first_name`, 
  `customer`.`last_name`, 
  `rental`.`rental_date` AS `rental_time` 
FROM 
  `customer` 
  INNER JOIN `rental` ON `customer`.`customer_id` = `rental`.`customer_id` 
WHERE 
  `rental_date` BETWEEN '2005-06-14' 
  AND '2005-06-16'
```

> **Note**
>
> - `Expr::tbl(Alias::new("rental"), rental::Column::RentalDate).into_simple_expr(),`
> - `inner_join`
> - `Condition::all().add()`
> - `Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-16")`
>
> **Wraning**
>
> - Built in function `date()` `time()` `upper()`
> - `columns([])`
> - `from()` `in_subquery()`

### 3.5 `WHERE` 子句

#### SQL 语句

```bash
mysql> SELECT title
 -> FROM film
 -> WHERE rating = 'G' AND rental_duration >= 7;
+-------------------------+
| title |
+-------------------------+
| BLANKET BEVERLY |
| BORROWERS BEDAZZLED |
| BRIDE INTRIGUE |
| CATCH AMISTAD |
| CITIZEN SHREK |
| COLDBLOODED DARLING |
| CONTROL ANTHEM |
| CRUELTY UNFORGIVEN |
| DARN FORRESTER |
| DESPERATE TRAINSPOTTING |
| DIARY PANIC |
| DRACULA CRYSTAL |
| EMPIRE MALKOVICH |
| FIREHOUSE VIETNAM |
| GILBERT PELICAN |
| GRADUATE LORD |
| GREASE YOUTH |
| GUN BONNIE |
| HOOK CHARIOTS |
| MARRIED GO |
| MENAGERIE RUSHMORE |
| MUSCLE BRIGHT |
| OPERATION OPERATION |
| PRIMARY GLASS |
| REBEL AIRPORT |
| SPIKING ELEMENT |
| TRUMAN CRAZY |
| WAKE JAWS |
| WAR NOTTING |
+-------------------------+
29 rows in set (0.00 sec)
```



#### SeaORM

```rust
#![allow(dead_code)]
use crate::entity::{film, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct RatingFilmRes {
    title: String,
}

pub async fn get_rating_film_where(db: &DatabaseConnection) -> Result<()> {
    let rating = Film::find()
        .select_only()
        .column(film::Column::Title)
        .filter(
            Condition::all()
                .add(Expr::col(film::Column::Rating).eq("G"))
                .add(Expr::col(film::Column::RentalDuration).gte(7)),
        )
        .into_model::<RatingFilmRes>()
        .all(db)
        .await?;
    println!("{:?}", rating);
    Ok(())
}
```

`AND` `Condition::all()`

```sql
SELECT 
  `film`.`title` 
FROM 
  `film` 
WHERE 
  `rating` = 'G' 
  AND `rental_duration` >= 7
```

`OR` `Condition::any()`

```bash
mysql> SELECT title
 -> FROM film
 -> WHERE rating = 'G' OR rental_duration >= 7;
+---------------------------+
| title |
+---------------------------+
| ACE GOLDFINGER |
| ADAPTATION HOLES |
| AFFAIR PREJUDICE |
| AFRICAN EGG |
| ALAMO VIDEOTAPE |
| AMISTAD MIDSUMMER |
| ANGELS LIFE |
| ANNIE IDENTITY |
|... |
| WATERSHIP FRONTIER |
| WEREWOLF LOLA |
| WEST LION |
| WESTWARD SEABISCUIT |
| WOLVES DESIRE |
| WON DARES |
| WORKER TARZAN |
| YOUNG LANGUAGE |
+---------------------------+
340 rows in set (0.00 sec)
```

多个查询条件，比如在 `where` 子句中同时使用运算符 `and` 和 `or`

#### SQL 语句

```bash
mysql> SELECT title, rating, rental_duration
 -> FROM film
 -> WHERE (rating = 'G' AND rental_duration >= 7)
 -> OR (rating = 'PG-13' AND rental_duration < 4);
The where Clause | 59
+-------------------------+--------+-----------------+
| title | rating | rental_duration |
+-------------------------+--------+-----------------+
| ALABAMA DEVIL | PG-13 | 3 |
| BACKLASH UNDEFEATED | PG-13 | 3 |
| BILKO ANONYMOUS | PG-13 | 3 |
| BLANKET BEVERLY | G | 7 |
| BORROWERS BEDAZZLED | G | 7 |
| BRIDE INTRIGUE | G | 7 |
| CASPER DRAGONFLY | PG-13 | 3 |
| CATCH AMISTAD | G | 7 |
| CITIZEN SHREK | G | 7 |
| COLDBLOODED DARLING | G | 7 |
|... |
| TREASURE COMMAND | PG-13 | 3 |
| TRUMAN CRAZY | G | 7 |
| WAIT CIDER | PG-13 | 3 |
| WAKE JAWS | G | 7 |
| WAR NOTTING | G | 7 |
| WORLD LEATHERNECKS | PG-13 | 3 |
+-------------------------+--------+-----------------+
68 rows in set (0.00 sec)
```

#### 

#### SeaORM

```rust
#[derive(Debug, FromQueryResult)]
struct RatingFilmAndOrRes {
    title: String,
    rating: Option<Rating>,
    rental_duration: u8,
}

pub async fn get_rating_film_mul_where(db: &DatabaseConnection) -> Result<()> {
    let rating = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .column(film::Column::RentalDuration)
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(Expr::col(film::Column::Rating).eq("G"))
                        .add(Expr::col(film::Column::RentalDuration).gte(7)),
                )
                .add(
                    Condition::all()
                        .add(Expr::col(film::Column::Rating).eq("PG-13"))
                        .add(Expr::col(film::Column::RentalDuration).lt(4)),
                ),
        )
        .into_model::<RatingFilmAndOrRes>()
        .all(db)
        .await?;
    println!("{:?}", rating);
    Ok(())
}

```

```sql
SELECT 
  `film`.`title`, 
  `film`.`rating`, 
  `film`.`rental_duration` 
FROM 
  `film` 
WHERE 
  (
    `rating` = 'G' 
    AND `rental_duration` >= 7
  ) 
  OR (
    `rating` = 'PG-13' 
    AND `rental_duration` < 4
  )
```



### 3.1 查询机制

#### SQL 语句

#### SeaORM



### 3.1 查询机制

#### SQL 语句

#### SeaORM





### 3.1 查询机制

#### SQL 语句

#### SeaORM



