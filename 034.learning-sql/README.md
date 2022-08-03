- [The Sakila Database](#the-sakila-database)
- [3.1 查询机制](#31-查询机制)
- [3.3 `SELECT` 子句](#33-select-子句)
  - [3.3.1 列的别名](#331-列的别名)
  - [3.3.2 移除重复数据](#332-移除重复数据)
- [3.4 `FROM` 子句](#34-from-子句)
  - [3.4.1 数据表](#341-数据表)
  - [3.4.2 数据表链接](#342-数据表链接)
- [3.5 `WHERE` 子句](#35-where-子句)
- [3.6 `GROUP BY` 和 `HAVING` 子句](#36-group-by-和-having-子句)
- [3.7 `ORDER BY` 子句](#37-order-by-子句)
- [4.3 条件类型](#43-条件类型)
  - [4.3.1 相等条件](#431-相等条件)
    - [数据库内置函数 `date()` `time()`](#数据库内置函数-date-time)
  - [4.3.2 范围条件](#432-范围条件)
  - [4.3.3 成员条件](#433-成员条件)
    - [1. 使用子查询](#1-使用子查询)
  - [4.3.4 匹配条件](#434-匹配条件)
    - [`starts_with` `between` `like` `contains` `is_in`](#starts_with-between-like-contains-is_in)
    - [正则表达式](#正则表达式)
- [4.4 null:  4个字母的单词](#44-null--4个字母的单词)

```bash
sea-orm-cli generate entity -v -u mysql://root:root1234@127.0.0.1:3306/sakila -o src/entity -t actor,category  --with-serde both
```

### The Sakila Database

```
sakila-schema.sql
sakila-data.sql
```

### 3.1 查询机制

***SQL 语句***

```bash
mysql> SELECT first_name, last_name
 	-> FROM customer
 	-> WHERE last_name = 'ZIEGLER';
Empty set (0.02 sec)
```

***SeaORM***

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

***SQL 语句***

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

***SeaORM***

```rust
Category::find().all(db).await?;
```

```sql
SELECT `category`.`category_id`, `category`.`name`, `category`.`last_update` FROM `category`
```



### 3.3 `SELECT` 子句

***SQL 语句***

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

***SeaORM***

```rust
Language::find().all(db).await?;
```

```sql
SELECT `language`.`language_id`, `language`.`name`, `language`.`last_update` FROM `language`
```

***SQL 语句***

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

***SeaORM***

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

#### 3.3.1 列的别名

***SQL 语句***

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

***SeaORM***

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

#### 3.3.2 移除重复数据

***SQL 语句***

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

***SeaORM***

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

#### 3.4.1 数据表

***SQL 语句***

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

***SeaORM***

> **Warning**
>
> - `concat()`
> - subquery

#### 3.4.2 数据表链接

***SQL 语句***

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

***SeaORM***

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

***SQL 语句***

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

***SeaORM***

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

***SQL 语句***

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

***SeaORM***

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



### 3.6 `GROUP BY` 和 `HAVING` 子句

- `GROUP BY` 根据列值对数据进行分组
- `HAVING` 过滤数据

要求：**找出所有租借过 40 部或更多电影的客户**

***SQL 语句***

```bash
mysql> SELECT c.first_name, c.last_name, count(*)
 -> FROM customer c
 -> INNER JOIN rental r
 -> ON c.customer_id = r.customer_id
 -> GROUP BY c.first_name, c.last_name
 -> HAVING count(*) >= 40;
+------------+-----------+----------+
| first_name | last_name | count(*) |
+------------+-----------+----------+
| TAMMY | SANDERS | 41 |
| CLARA | SHAW | 42 |
| ELEANOR | HUNT | 46 |
| SUE | PETERS | 40 |
| MARCIA | DEAN | 42 |
| WESLEY | BULL | 40 |
| KARL | SEAL | 45 |
+------------+-----------+----------+
7 rows in set (0.03 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect};

#[derive(Debug, FromQueryResult)]
struct CustomerGroupBy {
    first_name: String,
    last_name: String,
    count: i32,
}
pub async fn get_customer_group_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column_as(Expr::asterisk().count(), "count")
        .inner_join(Rental)
        .group_by(customer::Column::FirstName)
        .group_by(customer::Column::LastName)
        .having(Expr::expr(Expr::asterisk().count()).gte(40))
        // .build(DbBackend::MySql)
        // .to_string();
        .into_model::<CustomerGroupBy>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `customer`.`first_name`, 
  `customer`.`last_name`, 
  COUNT(*) AS `count` 
FROM 
  `customer` 
  INNER JOIN `rental` ON `customer`.`customer_id` = `rental`.`customer_id` 
GROUP BY 
  `customer`.`first_name`, 
  `customer`.`last_name` 
HAVING 
  COUNT(*) >= 40
```

> **Note**
>
> - `COUNT(*)` : `.column_as(Expr::asterisk().count(), "count")`
> - `inner_join`
> - `group_by`
> - `having`
> - `COUNT(*) >= 40` : `Expr::expr(Expr::asterisk().count()).gte(40)`

### 3.7 `ORDER BY` 子句

要求：**查询可以返回在 2005 年 6 月 14 日租借电影的所有客户**

***SQL 语句***

```bash
mysql> SELECT c.first_name, c.last_name,
 -> time(r.rental_date) rental_time
 -> FROM customer c
 -> INNER JOIN rental r
 -> ON c.customer_id = r.customer_id
 -> WHERE date(r.rental_date) = '2005-06-14';
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
The order by Clause | 61
| TERRENCE | GUNDERSON | 23:47:35 |
| SONIA | GREGORY | 23:50:11 |
| CHARLES | KOWALSKI | 23:54:34 |
| JEANETTE | GREENE | 23:54:46 |
+------------+-----------+-------------+
16 rows in set (0.01 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]

use crate::entity::{customer, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    entity::prelude::*,
    sea_query::{Alias, Expr},
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};

#[derive(Debug, FromQueryResult)]
struct CustomerOrderBy {
    first_name: String,
    last_name: String,
    rental_time: DateTime,
}

pub async fn get_customer_order_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column_as(
            Expr::tbl(Alias::new("rental"), rental::Column::RentalDate).into_simple_expr(),
            "rental_time",
        )
        .join_rev(JoinType::InnerJoin, rental::Relation::Customer.def())
        .filter(Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-15"))
        .into_model::<CustomerOrderBy>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
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
  INNER JOIN `rental` ON `rental`.`customer_id` = `customer`.`customer_id` 
WHERE 
  `rental_date` BETWEEN '2005-06-14' 
  AND '2005-06-15'
```

> **Note**
>
> - `inner_join`
>   - `.inner_join(Rental)`
>   - `join_rev(JoinType::InnerJoin, rental::Relation::Customer.def())`
>
> - `date(r.rental_date) = '2005-06-14';`
>
>   -  `rental_date BETWEEN '2005-06-14' 
>       AND '2005-06-15'`
>
>   - `Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-15")`

要求：**查询可以返回在 2005 年 6 月 14 日租借电影的所有客户，并希望结果按姓氏的字母顺序排序**

***SQL 语句***

```bash

mysql> SELECT c.first_name, c.last_name,
 -> time(r.rental_date) rental_time
 -> FROM customer c
 -> INNER JOIN rental r
 -> ON c.customer_id = r.customer_id
 -> WHERE date(r.rental_date) = '2005-06-14'
 -> ORDER BY c.last_name;
+------------+-----------+-------------+
| first_name | last_name | rental_time |
+------------+-----------+-------------+
| DANIEL | CABRAL | 23:09:38 |
| CATHERINE | CAMPBELL | 23:17:03 |
| HERMAN | DEVORE | 23:35:09 |
| AMBER | DIXON | 23:42:56 |
| JOYCE | EDWARDS | 23:16:26 |
| JEANETTE | GREENE | 23:54:46 |
| SONIA | GREGORY | 23:50:11 |
| TERRENCE | GUNDERSON | 23:47:35 |
| CHARLES | KOWALSKI | 23:54:34 |
| MATTHEW | MAHAN | 23:25:58 |
| GWENDOLYN | MAY | 23:16:27 |
| MIRIAM | MCKINNEY | 23:07:08 |
| ELMER | NOE | 22:55:13 |
| JEFFERY | PINSON | 22:53:33 |
| MINNIE | ROMERO | 23:00:34 |
| TERRANCE | ROUSH | 23:12:46 |
+------------+-----------+-------------+
16 rows in set (0.01 sec)
```

***SeaORM***

```rust
let customer = Customer::find()
    .select_only()
    .column(customer::Column::FirstName)
    .column(customer::Column::LastName)
    .column_as(
        Expr::tbl(Alias::new("rental"), rental::Column::RentalDate).into_simple_expr(),
        "rental_time",
    )
    .join_rev(JoinType::InnerJoin, rental::Relation::Customer.def())
    .filter(Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-15"))
    .order_by_asc(customer::Column::LastName)
    .into_model::<CustomerOrderBy>()
    .all(db)
    .await?;
```

```sql
SELECT 
  `customer`.`first_name`, 
  `customer`.`last_name`, 
  `rental`.`rental_date` AS `rental_time` 
FROM 
  `customer` 
  INNER JOIN `rental` ON `rental`.`customer_id` = `customer`.`customer_id` 
WHERE 
  `rental_date` BETWEEN '2005-06-14' 
  AND '2005-06-15' 
ORDER BY 
  `customer`.`last_name` ASC
```

列在 `ORDER BY` 子句中出现的顺序不同，结果不同。

```rust
let customer = Customer::find()
    .select_only()
    .column(customer::Column::FirstName)
    .column(customer::Column::LastName)
    .column_as(
        Expr::tbl(Alias::new("rental"), rental::Column::RentalDate).into_simple_expr(),
        "rental_time",
    )
    .join_rev(JoinType::InnerJoin, rental::Relation::Customer.def())
    .filter(Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-15"))
    .order_by_asc(customer::Column::LastName)
    .order_by(customer::Column::FirstName, Order::Asc)
    .into_model::<CustomerOrderBy>()
    .all(db)
    .await?;
```

```sql
SELECT 
  `customer`.`first_name`, 
  `customer`.`last_name`, 
  `rental`.`rental_date` AS `rental_time` 
FROM 
  `customer` 
  INNER JOIN `rental` ON `rental`.`customer_id` = `customer`.`customer_id` 
WHERE 
  `rental_date` BETWEEN '2005-06-14' 
  AND '2005-06-15' 
ORDER BY 
  `customer`.`last_name` ASC, 
  `customer`.`first_name` ASC
```

在排序时默认为按照升序排序，如果希望按照降序排序，需要使用 `desc` 。

> **Note**
>
> - `order_by_asc(customer::Column::LastName)`
> - `order_by(customer::Column::FirstName, Order::Asc)`

### 4.3 条件类型

#### 4.3.1 相等条件

要求： **在 2005 年 6 月 14 日租借电影的所有客户的电子邮件地址**

***SQL 语句***

```bash
mysql> SELECT c.email
 -> FROM customer c
 -> INNER JOIN rental r
 -> ON c.customer_id = r.customer_id
 -> WHERE date(r.rental_date) = '2005-06-14';
+---------------------------------------+
| email |
+---------------------------------------+
| CATHERINE.CAMPBELL@sakilacustomer.org |
| JOYCE.EDWARDS@sakilacustomer.org |
| AMBER.DIXON@sakilacustomer.org |
| JEANETTE.GREENE@sakilacustomer.org |
| MINNIE.ROMERO@sakilacustomer.org |
| GWENDOLYN.MAY@sakilacustomer.org |
| SONIA.GREGORY@sakilacustomer.org |
| MIRIAM.MCKINNEY@sakilacustomer.org |
| CHARLES.KOWALSKI@sakilacustomer.org |
| DANIEL.CABRAL@sakilacustomer.org |
| MATTHEW.MAHAN@sakilacustomer.org |
| JEFFERY.PINSON@sakilacustomer.org |
| HERMAN.DEVORE@sakilacustomer.org |
| ELMER.NOE@sakilacustomer.org |
| TERRANCE.ROUSH@sakilacustomer.org |
| TERRENCE.GUNDERSON@sakilacustomer.org |
+---------------------------------------+
16 rows in set (0.03 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{customer, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct EqualityConditionsRes {
    email: String,
}

pub async fn equality_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::Email)
        .inner_join(Rental)
        .filter(rental::Column::RentalDate.between("2005-06-14", "2005-06-15"))
        .into_model::<EqualityConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `customer`.`email` 
FROM 
  `customer` 
  INNER JOIN `rental` ON `customer`.`customer_id` = `rental`.`customer_id` 
WHERE 
  `rental`.`rental_date` BETWEEN '2005-06-14' 
  AND '2005-06-15'
```

> **Note**
>
> - `.filter(rental::Column::RentalDate.between("2005-06-14", "2005-06-15"))`

##### 数据库内置函数 `date()` `time()` 

***SQL 语句***

```bash
mysql> SELECT c.email
 -> FROM customer c
 -> INNER JOIN rental r
 -> ON c.customer_id = r.customer_id
 -> WHERE date(r.rental_date) = '2005-06-14';
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{customer, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    entity::prelude::*,
    sea_query::{Expr, Func, Iden},
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct EqualityConditionsRes {
    email: String,
}

struct Date;

impl Iden for Date {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "date").unwrap();
    }
}

pub async fn equality_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::Email)
        .inner_join(Rental)
        .filter(
            Func::cust(Date)
                .args(vec![Expr::col(rental::Column::RentalDate)])
                .equals(Expr::val("2005-06-14")),
        )
        // .filter(rental::Column::RentalDate.between("2005-06-14", "2005-06-15"))
        .into_model::<EqualityConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `customer`.`email` 
FROM 
  `customer` 
  INNER JOIN `rental` ON `customer`.`customer_id` = `rental`.`customer_id` 
WHERE 
  date(`rental_date`) = '2005-06-14'
```

> **Note**
>
> 使用数据库内置函数
>
> **pub fn [cust](https://docs.rs/sea-query/0.26.2/sea_query/func/struct.Func.html#method.cust)<T>(func: T) -> [Expr](https://docs.rs/sea-query/0.26.2/sea_query/expr/struct.Expr.html) where   T: [IntoIden](https://docs.rs/sea-query/0.26.2/sea_query/types/trait.IntoIden.html), **

#### 4.3.2 范围条件

要求： **搜索在 2005 年 5 月 25 日之前租借的电影**

***SQL 语句***

```bash
mysql> SELECT customer_id, rental_date
 -> FROM rental
 -> WHERE rental_date < '2005-05-25';
+-------------+---------------------+
| customer_id | rental_date |
+-------------+---------------------+
| 130 | 2005-05-24 22:53:30 |
| 459 | 2005-05-24 22:54:33 |
| 408 | 2005-05-24 23:03:39 |
| 333 | 2005-05-24 23:04:41 |
| 222 | 2005-05-24 23:05:21 |
| 549 | 2005-05-24 23:08:07 |
| 269 | 2005-05-24 23:11:53 |
| 239 | 2005-05-24 23:31:46 |
+-------------+---------------------+
8 rows in set (0.00 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    entity::prelude::*, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct RangeConditionsRes {
    customer_id: u16,
    rental_date: DateTime,
}

pub async fn range_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column(rental::Column::RentalDate)
        .filter(rental::Column::RentalDate.lt("2005-05-25"))
        .into_model::<RangeConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `rental`.`customer_id`, 
  `rental`.`rental_date` 
FROM 
  `rental` 
WHERE 
  `rental`.`rental_date` < '2005-05-25'
```



#### 4.3.3 成员条件

要求: **找出评级为'G'或'PG'的所有电影**

***SQL 语句***

```bash
mysql> SELECT title, rating
 -> FROM film
 -> WHERE rating = 'G' OR rating = 'PG';
+---------------------------+--------+
| title | rating |
+---------------------------+--------+
| ACADEMY DINOSAUR | PG |
| ACE GOLDFINGER | G |
...
| YOUNG LANGUAGE | G |
+---------------------------+--------+
372 rows in set (0.00 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{film, prelude::*, sea_orm_active_enums::Rating};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MembershipConditionsRes {
    title: String,
    rating: Option<Rating>,
}

pub async fn membership_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .filter(
            Condition::any()
                .add(film::Column::Rating.eq("G"))
                .add(film::Column::Rating.eq("PG")),
        )
        .into_model::<MembershipConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `film`.`title`, 
  `film`.`rating` 
FROM 
  `film` 
WHERE 
  `film`.`rating` = 'G' 
  OR `film`.`rating` = 'PG'
```

> **Note**
>
> ```rust
> Condition::any()
>     .add(film::Column::Rating.eq("G"))
>     .add(film::Column::Rating.eq("PG")),
> ```

##### 1. 使用子查询

要求： **只要是影片名包含字符串'PET'的电影都适合家庭成员共同观看，就可以对 film 数据表执行子查询，县检索与这些电影相关的评级，然后检索和这些评级对应的所有电影**

***SQL 语句***

```bash
mysql> SELECT title, rating
 -> FROM film
 -> WHERE rating IN (SELECT rating FROM film WHERE title LIKE '%PET%');
+---------------------------+--------+
| title | rating |
+---------------------------+--------+
| ACADEMY DINOSAUR | PG |
| ACE GOLDFINGER | G |
...
| WORST BANGER | PG |
| YOUNG LANGUAGE | G |
+---------------------------+--------+
372 rows in set (0.00 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{film, prelude::*, sea_orm_active_enums::Rating};
use anyhow::Result;
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MembershipConditionsRes {
    title: String,
    rating: Option<Rating>,
}

pub async fn membership_conditions_in(db: &DatabaseConnection) -> Result<()> {
    let customer = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .filter(
            Condition::any().add(
                film::Column::Rating.in_subquery(
                    Query::select()
                        .column(film::Column::Rating)
                        .and_where(Expr::col(film::Column::Title).like("%PET%"))
                        .from(Film)
                        .to_owned(),
                ),
            ),
        )
        .into_model::<MembershipConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `film`.`title`, 
  `film`.`rating` 
FROM 
  `film` 
WHERE 
  `film`.`rating` IN (
    SELECT 
      `rating` 
    FROM 
      `film` 
    WHERE 
      `title` LIKE '%PET%'
  )
```

> **Note**
>
> - `IN` 后跟 `SELECT` 语句
>
>   - ```rust
>     .filter(
>         Condition::any().add(
>             film::Column::Rating.in_subquery(
>                 Query::select()
>                 .column(film::Column::Rating)
>                 .and_where(Expr::col(film::Column::Title).like("%PET%"))
>                 .from(Film)
>                 .to_owned(),
>             ),
>         ),
>     )
>     ```

要求： **有时候需要知道特定表达式是否存在于某个表达式集合中，而有时候又需要知道特定表达式是否不存在于某个表达式集合中。对此，可以使用 `NOT IN` 运算符**

***SQL 语句***

```bash
SELECT title, rating
FROM film
WHERE rating NOT IN ('PG-13','R', 'NC-17');
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{film, prelude::*, sea_orm_active_enums::Rating};
use anyhow::Result;
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MembershipConditionsRes {
    title: String,
    rating: Option<Rating>,
}

pub async fn membership_conditions_not_in(db: &DatabaseConnection) -> Result<()> {
    let customer = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .filter(film::Column::Rating.is_not_in(["PG-13", "R", "NC-17"]))
        .into_model::<MembershipConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `film`.`title`, 
  `film`.`rating` 
FROM 
  `film` 
WHERE 
  `film`.`rating` NOT IN ('PG-13', 'R', 'NC-17')
```

> **Note**
>
> `IN` 和 `NOT IN` 后跟字符串
>
> `.filter(film::Column::Rating.is_not_in(["PG-13", "R", "NC-17"]))`



#### 4.3.4 匹配条件

##### `starts_with` `between` `like` `contains` `is_in`

要求： **查找姓氏以 Q 开头的所有客户**

***SQL 语句***

```bash
mysql> SELECT last_name, first_name
 -> FROM customer
 -> WHERE left(last_name, 1) = 'Q';
+-------------+------------+
| last_name | first_name |
+-------------+------------+
| QUALLS | STEPHEN |
| QUINTANILLA | ROGER |
| QUIGLEY | TROY |
+-------------+------------+
3 rows in set (0.00 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MatchingConditionsRes {
    last_name: String,
    first_name: String,
}

pub async fn matching_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::LastName)
        .column(customer::Column::FirstName)
        .filter(customer::Column::LastName.starts_with("Q"))
        .into_model::<MatchingConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `customer`.`last_name`, 
  `customer`.`first_name` 
FROM 
  `customer` 
WHERE 
  `customer`.`last_name` LIKE 'Q%'
```

> **Note**
>
> - `use sea_orm::ColumnTrait`
> - `.filter(customer::Column::LastName.starts_with("Q"))`
> - [ColumnTrait in sea_orm::entity::prelude - Rust (docs.rs)](https://docs.rs/sea-orm/0.9.1/sea_orm/entity/prelude/trait.ColumnTrait.html#method.starts_with)

要求： **使用多个搜索表达式，查询搜索姓氏以 Q 或 Y 开头的所有客户**

***SQL 语句***

```bash
mysql> SELECT last_name, first_name
 -> FROM customer
 -> WHERE last_name LIKE 'Q%' OR last_name LIKE 'Y%';
+-------------+------------+
| last_name | first_name |
+-------------+------------+
| QUALLS | STEPHEN |
| QUIGLEY | TROY |
| QUINTANILLA | ROGER |
| YANEZ | LUIS |
| YEE | MARVIN |
| YOUNG | CYNTHIA |
+-------------+------------+
6 rows in set (0.00 sec)
```

***SeaORM***

```rust
let customer = Customer::find()
    .select_only()
    .column(customer::Column::LastName)
    .column(customer::Column::FirstName)
    .filter(
        Condition::any()
            .add(customer::Column::LastName.starts_with("Q"))
            .add(customer::Column::LastName.starts_with("Y")),
    )
    .into_model::<MatchingConditionsRes>()
    .all(db)
    .await?;
```

```sql
SELECT 
  `customer`.`last_name`, 
  `customer`.`first_name` 
FROM 
  `customer` 
WHERE 
  `customer`.`last_name` LIKE 'Q%' 
  OR `customer`.`last_name` LIKE 'Y%'
```

> **Note**
>
> `.filter(
>         Condition::any()
>             .add(customer::Column::LastName.starts_with("Q"))
>             .add(customer::Column::LastName.starts_with("Y")),
>     )`

##### 正则表达式

要求： **如果通配符无法提供足够的灵活性，可以使用正则表达式来构建搜索表达式。**

**搜索姓氏以 Q 或 Y 开头的所有客户**

***SQL 语句***

```bash
mysql> SELECT last_name, first_name
 -> FROM customer
 -> WHERE last_name REGEXP '^[QY]';
+-------------+------------+
| last_name | first_name |
+-------------+------------+
| YOUNG | CYNTHIA |
| QUALLS | STEPHEN |
| QUINTANILLA | ROGER |
| YANEZ | LUIS |
| YEE | MARVIN |
| QUIGLEY | TROY |
+-------------+------------+
6 rows in set (0.16 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult,
    QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MatchingConditionsRes {
    last_name: String,
    first_name: String,
}

pub async fn matching_conditions_regexp(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::LastName)
        .column(customer::Column::FirstName)
        .filter(Expr::cust(r#"`last_name` REGEXP '^[QY]'"#))
        .into_model::<MatchingConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
```

```sql
SELECT 
  `customer`.`last_name`, 
  `customer`.`first_name` 
FROM 
  `customer` 
WHERE 
  `last_name` REGEXP '^[QY]'
```

> **Note**
>
> 正则表达式：`.filter(Expr::cust(r#"last_name REGEXP '^[QY]'"#))`
>
> **pub fn [cust](https://docs.rs/sea-query/latest/sea_query/expr/struct.Expr.html#method.cust)(s: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [SimpleExpr](https://docs.rs/sea-query/latest/sea_query/expr/enum.SimpleExpr.html)**



### 4.4 null:  4个字母的单词

要求： **查询租借后从未归还的所有电影**

***SQL 语句***

```bash
mysql> SELECT rental_id, customer_id
 -> FROM rental
 -> WHERE return_date IS NULL;
+-----------+-------------+
| rental_id | customer_id |
+-----------+-------------+
| 11496 | 155 |
| 11541 | 335 |
| 11563 | 83 |
| 11577 | 219 |
| 11593 | 99 |
...
| 15867 | 505 |
| 15875 | 41 |
| 15894 | 168 |
| 15966 | 374 |
+-----------+-------------+
183 rows in set (0.01 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct NeverEqualNullRes {
    rental_id: i32,
    customer_id: u16,
}
pub async fn never_equal_null(db: &DatabaseConnection) -> Result<()> {
    let rental = Rental::find()
        .select_only()
        .column(rental::Column::RentalId)
        .column(rental::Column::CustomerId)
        .filter(rental::Column::ReturnDate.is_null())
        .into_model::<NeverEqualNullRes>()
        .all(db)
        .await?;

    println!("{:?}", rental);
    Ok(())
}
```

```sql
SELECT 
  `rental`.`rental_id`, 
  `rental`.`customer_id` 
FROM 
  `rental` 
WHERE 
  `rental`.`return_date` IS NULL
```

> **Note**
>
> - 表达式可以为 `null` ，但是不能等于 `= null`
> - 两个 `null` 值不想等

要求： **查询返回 租借后已经归还的所有电影记录**

***SQL 语句***

```bash
mysql> SELECT rental_id, customer_id, return_date
 -> FROM rental
 -> WHERE return_date IS NOT NULL;
+-----------+-------------+---------------------+
| rental_id | customer_id | return_date |
+-----------+-------------+---------------------+
| 1 | 130 | 2005-05-26 22:04:30 |
...
| 16049 | 393 | 2005-08-30 01:01:12 |
+-----------+-------------+---------------------+
15861 rows in set (0.02 sec)
```

***SeaORM***

```sql
SELECT 
  `rental`.`rental_id`, 
  `rental`.`customer_id` 
FROM 
  `rental` 
WHERE 
  `rental`.`return_date` IS NOT NULL
```

```rust
Rental::find()
    .select_only()
    .column(rental::Column::RentalId)
    .column(rental::Column::CustomerId)
    .filter(rental::Column::ReturnDate.is_not_null())
    .into_model::<NeverEqualNullRes>()
    .all(db)
    .await?;
```

> **Note**

要求： **查找 2015 年 5 月至 8 月期间所有未归还电影的记录**

- 62 条记录是在 5月至 8 月期间之外归还的
- 还有 183 部从未归还的电影记录

***SQL 语句***

```bash
mysql> SELECT rental_id, customer_id, return_date
 -> FROM rental
 -> WHERE return_date IS NULL
 -> OR return_date NOT BETWEEN '2005-05-01' AND '2005-09-01';
+-----------+-------------+---------------------+
| rental_id | customer_id | return_date |
+-----------+-------------+---------------------+
| 11496 | 155 | NULL |
...
| 15942 | 210 | 2005-09-01 18:39:40 |
| 15966 | 374 | NULL |
| 16037 | 45 | 2005-09-01 02:48:04 |
| 16040 | 195 | 2005-09-02 02:19:33 |
+-----------+-------------+---------------------+
245 rows in set (0.01 sec)
```

***SeaORM***

```sql
SELECT 
  `rental`.`rental_id`, 
  `rental`.`customer_id` 
FROM 
  `rental` 
WHERE 
  `rental`.`return_date` IS NULL 
  OR (
    `rental`.`return_date` NOT BETWEEN '2005-05-01' 
    AND '2005-09-01'
  )
```

```rust
#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct NeverEqualNullRes {
    rental_id: i32,
    customer_id: u16,
}
pub async fn never_equal_null_or(db: &DatabaseConnection) -> Result<()> {
    let rental = Rental::find()
        .select_only()
        .column(rental::Column::RentalId)
        .column(rental::Column::CustomerId)
        .filter(
            Condition::any()
                .add(rental::Column::ReturnDate.is_null())
                .add(rental::Column::ReturnDate.not_between("2005-05-01", "2005-09-01")),
        )
        .into_model::<NeverEqualNullRes>()
        .all(db)
        .await?;

    println!("{:?}", rental);
    Ok(())
}
```

> **Note**
>
> - `is_null()`
> - `not_between()`

### 5.1.2 内链接

要求： **查询检索客户的姓氏和名字，以及居住的街道地址**

***SQL 语句***

```bash
mysql> SELECT c.first_name, c.last_name, a.address
 -> FROM customer c JOIN address a
 -> ON c.address_id = a.address_id;
+-------------+--------------+----------------------------------------+
| first_name | last_name | address |
+-------------+--------------+----------------------------------------+
| MARY | SMITH | 1913 Hanoi Way |
...
| AUSTIN | CINTRON | 1325 Fukuyama Street |
+-------------+--------------+----------------------------------------+
599 rows in set (0.00 sec)
```

***SeaORM***

```sql
SELECT 
  `customer`.`first_name`, 
  `customer`.`last_name`, 
  `address`.`address` AS `address` 
FROM 
  `customer` 
  INNER JOIN `address` ON `customer`.`address_id` = `address`.`address_id`
```

```rust
#![allow(dead_code)]
use crate::entity::{address, customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::{Alias, Expr},
    DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
    address: String,
}

pub async fn inner_join(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column_as(
            Expr::tbl(Alias::new("address"), address::Column::Address).into_simple_expr(),
            "address",
        )
        .inner_join(Address)
        .into_model::<CustomerRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}
```

> **Note**
>
> SQL92 内链接语法与之前旧的连接语法
>
> ```sql
> SELECT c.first_name, c.last_name, a.address
> FROM customer c
>          INNER JOIN address a
>                     ON c.address_id = a.address_id;
> ```
>
> ```sql
> SELECT c.first_name, c.last_name, a.address
> FROM customer c,
>      address a
> WHERE c.address_id = a.address_id;
> ```

SQL92 语法的优势在于更易于识别同时包含连接和过滤条件的复杂查询

要求： **查询返回邮政编码为 52137 的客户**

***SQL 语句***

```bash
mysql> SELECT c.first_name, c.last_name, a.address
 -> FROM customer c, address a
 -> WHERE c.address_id = a.address_id
 -> AND a.postal_code = 52137;
+------------+-----------+------------------------+
| first_name | last_name | address |
+------------+-----------+------------------------+
| JAMES | GANNON | 1635 Kuwana Boulevard |
| FREDDIE | DUGGAN | 1103 Quilmes Boulevard |
+------------+-----------+------------------------+
2 rows in set (0.01 sec)
```

```bash
mysql> SELECT c.first_name, c.last_name, a.address
 -> FROM customer c INNER JOIN address a
 -> ON c.address_id = a.address_id
 -> WHERE a.postal_code = 52137;
+------------+-----------+------------------------+
| first_name | last_name | address |
+------------+-----------+------------------------+
| JAMES | GANNON | 1635 Kuwana Boulevard |
| FREDDIE | DUGGAN | 1103 Quilmes Boulevard |
+------------+-----------+------------------------+
2 rows in set (0.00 sec)
```

***SeaORM***

```sql
SELECT `customer`.`first_name`, `customer`.`last_name`, `address`.`address`
FROM `customer`
         INNER JOIN `address` ON `customer`.`address_id` = `address`.`address_id`
WHERE `address`.`postal_code` = 52137;
```

```rust
pub async fn inner_join_sql92(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column(address::Column::Address)
        // .column_as(
        //     Expr::tbl(Alias::new("address"), address::Column::Address).into_simple_expr(),
        //     "address",
        // )
        .inner_join(Address)
        .filter(address::Column::PostalCode.eq(52137))
        .into_model::<CustomerRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}
```

### 5.2 连接 3 个或以上的数据表

要求： **查询返回客户所在的城市，而不再返回街道地址**

***SQL 语句***

```bash
mysql> SELECT c.first_name, c.last_name, ct.city
 -> FROM customer c
 -> INNER JOIN address a
 -> ON c.address_id = a.address_id
 -> INNER JOIN city ct
 -> ON a.city_id = ct.city_id;
+-------------+--------------+----------------------------+
| first_name | last_name | city |
+-------------+--------------+----------------------------+
| JULIE | SANCHEZ | A Corua (La Corua) |
...
| RONNIE | RICKETTS | Ziguinchor |
+-------------+--------------+----------------------------+
599 rows in set (0.03 sec)
```

***SeaORM***

```sql
SELECT `customer`.`first_name`, `customer`.`last_name`, `address`.`address`
FROM `customer`
         INNER JOIN `address` ON `address`.`address_id` = `customer`.`address_id`
         INNER JOIN `city` ON `city`.`city_id` = `address`.`city_id`;
```

```rust
#![allow(dead_code)]
use crate::entity::{address, city, customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QuerySelect, RelationTrait,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
    address: String,
}

pub async fn joining_three_or_more_tables(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column(address::Column::Address)
        .join_rev(JoinType::InnerJoin, address::Relation::Customer.def())
        .join_rev(JoinType::InnerJoin, city::Relation::Address.def())
        .into_model::<CustomerRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

> **Note**
>
> - `inner_join()` `join_rev()` `join()`

```sql
SELECT `customer`.`first_name`, `customer`.`last_name`, `address`.`address`
FROM `customer`
         INNER JOIN `address` ON `customer`.`address_id` = `address`.`address_id`
         INNER JOIN `city` ON `address`.`city_id` = `city`.`city_id`;
```

```rust
Customer::find()
    .select_only()
    .column(customer::Column::FirstName)
    .column(customer::Column::LastName)
    .column(address::Column::Address)
    .inner_join(Address)
    .join(JoinType::InnerJoin, address::Relation::City.def())
    .into_model::<CustomerRes>()
    .all(db)
    .await?;
```

### 6.3.1 `UNION` 运算符

要求： **从 customer 和 actor 表中获取所有名字和姓氏的集合**

***SQL 语句***

```bash
mysql> SELECT 'CUST' typ, c.first_name, c.last_name
 -> FROM customer c
 -> UNION ALL
 -> SELECT 'ACTR' typ, a.first_name, a.last_name
 -> FROM actor a;
+------+------------+-------------+
| typ | first_name | last_name |
+------+------------+-------------+
| CUST | MARY | SMITH |
...
| ACTR | THORA | TEMPLE |
+------+------------+-------------+
799 rows in set (0.00 sec)
```

***SeaORM***

```sql
SELECT `customer`.`first_name`, `customer`.`last_name`
FROM `customer`
UNION ALL
SELECT `first_name`, `last_name`
FROM `actor`;
```

```rust
#![allow(dead_code)]
use crate::entity::{actor, customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::{MysqlQueryBuilder, Query, UnionType},
    DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect, QueryTrait,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
}
pub async fn union_all(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .into_query()
        .union(
            UnionType::All,
            Query::select()
                .columns([actor::Column::FirstName, actor::Column::LastName])
                .from(Actor)
                .to_owned(),
        )
        .to_owned()
        .to_string(MysqlQueryBuilder);
    // .into_model::<CustomerRes>()
    // .all(db)
    // .await?;
    println!("{:?}", customer);
    Ok(())
}
```

***还不会使用***

> **Warning**
>
> `sea-orm` 和 `sea_query` 的集合使用 
>
> [SelectStatement in sea_query::query - Rust (docs.rs)](https://docs.rs/sea-query/0.26.2/sea_query/query/struct.SelectStatement.html#examples-15)
>
> [SelectStatement in sea_query::query - Rust (docs.rs)](https://docs.rs/sea-query/0.26.2/sea_query/query/struct.SelectStatement.html#method.union)



