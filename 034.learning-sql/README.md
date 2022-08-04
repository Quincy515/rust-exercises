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
- [5.1.2 内链接](#512-内链接)
- [5.2 连接 3 个或以上的数据表](#52-连接-3-个或以上的数据表)
- [6.3.1 `UNION` 运算符](#631-union-运算符)
- [8.1 分组概念 `GROUP_BY()` `HAVING()`](#81-分组概念-group_by-having)
- [8.2 聚合函数 `MAX()` `MIN()` `SUM()` `COUNT()`](#82-聚合函数-max-min-sum-count)
  - [8.2.1 显示分组](#821-显示分组)
  - [8.2.2　统计不同的值 `COUNT(DISTINCT customer_id) num_customers`](#822统计不同的值-countdistinct-customer_id-num_customers)
  - [8.2.3　使用表达式 `datediff(return_date,rental_date)`](#823使用表达式-datediffreturn_daterental_date)
- [8.3　生成分组](#83生成分组)
  - [8.3.1　单列分组](#831单列分组)
  - [8.3.2　多列分组](#832多列分组)
  - [8.3.3　通过表达式分组 `extract(YEAR FROM rental_date)`](#833通过表达式分组-extractyear-from-rental_date)
  - [8.3.4　生成汇总 `with rollup`](#834生成汇总-with-rollup)
- [8.4　分组过滤条件](#84分组过滤条件)

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

要求： **查询返回Cate McQueen和Cuba Birch两人共同参演的所有电影**

> 为此，要在film数据表中查找符合以下条件的所有行：对应于film_actor数据表中的两行。
>
> - 其中一行与Cate McQueen关联，
> - 另一行与Cuba Birch关联。
>
> 因此，需要包含数据表film_actor和actor两次，每次使用不同的别名，以便服务器知道在不同的子句中引用的是哪个数据表

***SQL 语句***

```bash
mysql> SELECT f.title
 -> FROM film f
 -> INNER JOIN film_actor fa1
 -> ON f.film_id = fa1.film_id
 -> INNER JOIN actor a1
 -> ON fa1.actor_id = a1.actor_id
 -> INNER JOIN film_actor fa2
 -> ON f.film_id = fa2.film_id
 -> INNER JOIN actor a2
 -> ON fa2.actor_id = a2.actor_id
 -> WHERE (a1.first_name = 'CATE' AND a1.last_name = 'MCQUEEN')
 -> AND (a2.first_name = 'CUBA' AND a2.last_name = 'BIRCH');
+------------------+
| title |
+------------------+
Joining Three or More Tables | 97
| BLOOD ARGONAUTS |
| TOWERS HURRICANE |
+------------------+
2 rows in set (0.00 sec)
```

***SeaORM***

```rust

```

```sql

```

> **Note**

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

### 8.1 分组概念 `GROUP_BY()` `HAVING()`

要求： **每位客户租借了多少部电影**

***SQL 语句***

```bash
mysql> SELECT customer_id, count(*)
 -> FROM rental
 -> GROUP BY customer_id;
+-------------+----------+
| customer_id | count(*) |
+-------------+----------+
| 1 | 32 |
...
| 599 | 19 |
+-------------+----------+
599 rows in set (0.01 sec)
```

***SeaORM***

```sql
SELECT 
  `rental`.`customer_id`, 
  COUNT(*) AS `count` 
FROM 
  `rental` 
GROUP BY 
  `rental`.`customer_id`
```

```rust
#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect};

#[derive(Debug, FromQueryResult)]
struct CustomerGropuByRes {
    customer_id: u16,
    count: i32,
}
pub async fn group_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(rental::Column::CustomerId)
        .into_model::<CustomerGropuByRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

> **Note**
>
> - `COUNT(*)`: `Expr::asterisk().count()`
> - `GROUP_BY()`

要求： **确定哪位客户借的电影最多**

***SQL 语句***

```bash
mysql> SELECT customer_id, count(*)
 -> FROM rental
 -> GROUP BY customer_id
 -> ORDER BY 2 DESC;
+-------------+----------+
| customer_id | count(*) |
+-------------+----------+
| 148 | 46 |
...
| 318 | 12 |
+-------------+----------+
599 rows in set (0.01 sec)
```

***SeaORM***

```rust

/// 要求： **确定哪位客户借的电影最多**
/// ```
/// SELECT
///   `rental`.`customer_id`,
///   COUNT(*) AS `count`
/// FROM
///   `rental`
/// GROUP BY
///   `rental`.`customer_id`
/// ORDER BY
///   COUNT(*) DESC
/// ```
pub async fn group_by_order_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(rental::Column::CustomerId)
        .order_by_desc(Expr::asterisk().count())
        .into_model::<CustomerGropuByRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

> **Note**
>
> - `ORDER_BY()` : `order_by_desc(Expr::asterisk().count())`



要求： **租借电影数量为40或多于40部的客户**

***SQL 语句***

```bash
mysql> SELECT customer_id, count(*)
 -> FROM rental
 -> GROUP BY customer_id
 -> HAVING count(*) >= 40;
+-------------+----------+
| customer_id | count(*) |
+-------------+----------+
| 75 | 41 |
| 144 | 42 |
| 148 | 46 |
| 197 | 40 |
| 236 | 42 |
| 469 | 40 |
| 526 | 45 |
+-------------+----------+
7 rows in set (0.01 sec)
```

***SeaORM***

```rust
/// 要求： 租借电影数量为40或多于40部的客户
/// ```
/// SELECT 
///   `rental`.`customer_id`, 
///   COUNT(*) AS `count` 
/// FROM 
///   `rental` 
/// GROUP BY 
///   `rental`.`customer_id` 
/// HAVING 
///   COUNT(*) >= 40
/// ```
pub async fn group_by_order_by_having(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(rental::Column::CustomerId)
        .having(Expr::expr(Expr::asterisk().count()).gte(40))
        .into_model::<CustomerGropuByRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

> **Note**
>
> - `HAVING()`

### 8.2 聚合函数 `MAX()` `MIN()` `SUM()` `COUNT()`

要求： **常见的聚合函数来分析电影租借付款数据**

***SQL 语句***

```bash
mysql> SELECT MAX(amount) max_amt,
 -> MIN(amount) min_amt,
 -> AVG(amount) avg_amt,
 -> SUM(amount) tot_amt,
 -> COUNT(*) num_payments
 -> FROM payment;
+---------+---------+----------+----------+--------------+
| max_amt | min_amt | avg_amt | tot_amt | num_payments |
+---------+---------+----------+----------+--------------+
| 11.99 | 0.00 | 4.200667 | 67416.51 | 16049 |
+---------+---------+----------+----------+--------------+
1 row in set (0.09 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{payment, prelude::*};
use anyhow::Result;
use sea_orm::{
    prelude::Decimal, sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait,
    FromQueryResult, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct PaymentRes {
    max_amt: Decimal,
    min_amt: Decimal,
    tot_amt: Decimal,
    num_payments: i32,
}

/// 要求：常见的聚合函数来分析电影租借付款数据
/// ```
/// SELECT
///   MAX(`payment`.`amount`) AS `max_amt`,
///   MIN(`payment`.`amount`) AS `min_amt`,
///   SUM(`payment`.`amount`) AS `tot_amt`,
///   COUNT(*) AS `num_payments`
/// FROM
///   `payment`
/// ```
pub async fn aggregate_functions(db: &DatabaseConnection) -> Result<()> {
    let payment = Payment::find()
        .select_only()
        .column_as(payment::Column::Amount.max(), "max_amt")
        .column_as(payment::Column::Amount.min(), "min_amt")
        .column_as(payment::Column::Amount.sum(), "tot_amt")
        .column_as(Expr::asterisk().count(), "num_payments")
        .into_model::<PaymentRes>()
        .all(db)
        .await?;
    println!("{:?}", payment);
    Ok(())
}
```

> **Note**
>
> - `MAX()` `MIN()` `SUM()` `COUNT()`
>
> **Warning**
>
> - 没有 `AVG()`

#### 8.2.1 显示分组

要求： **每位顾客的电影租借付款数据**

***SQL 语句***

```bash
mysql> SELECT customer_id,
 -> MAX(amount) max_amt,
 -> MIN(amount) min_amt,
 -> AVG(amount) avg_amt,
 -> SUM(amount) tot_amt,
 -> COUNT(*) num_payments
 -> FROM payment
 -> GROUP BY customer_id;
+-------------+---------+---------+----------+---------+--------------+
| customer_id | max_amt | min_amt | avg_amt | tot_amt | num_payments |
+-------------+---------+---------+----------+---------+--------------+
| 1 | 9.99 | 0.99 | 3.708750 | 118.68 | 32 |
...
| 599 | 9.99 | 0.99 | 4.411053 | 83.81 | 19 |
+-------------+---------+---------+----------+---------+--------------+
599 rows in set (0.04 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{payment, prelude::*};
use anyhow::Result;
use sea_orm::{
    prelude::Decimal, sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait,
    FromQueryResult, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct CustomerPayment {
    customer_id: u16,
    max_amt: Decimal,
    min_amt: Decimal,
    tot_amt: Decimal,
    num_payments: i32,
}

/// 要求： 每位顾客的电影租借付款数据
/// ```
/// SELECT 
///   `payment`.`customer_id`, 
///   MAX(`payment`.`amount`) AS `max_amt`, 
///   MIN(`payment`.`amount`) AS `min_amt`, 
///   SUM(`payment`.`amount`) AS `tot_amt`, 
///   COUNT(*) AS `num_payments` 
/// FROM 
///   `payment` 
/// GROUP BY 
///   `payment`.`customer_id`
/// ```
pub async fn aggregate_functions_customer(db: &DatabaseConnection) -> Result<()> {
    let payment = Payment::find()
        .select_only()
        .column(payment::Column::CustomerId)
        .column_as(payment::Column::Amount.max(), "max_amt")
        .column_as(payment::Column::Amount.min(), "min_amt")
        .column_as(payment::Column::Amount.sum(), "tot_amt")
        .column_as(Expr::asterisk().count(), "num_payments")
        .group_by(payment::Column::CustomerId)
        .into_model::<CustomerPayment>()
        .all(db)
        .await?;
    println!("{:?}", payment);
    Ok(())
}
```

> **Warning**
>
> - 怎么嵌套结构体

#### 8.2.2　统计不同的值 `COUNT(DISTINCT customer_id) num_customers`

要求： **count()函数检查分组中每个成员的列值，以便查找和删除重复项，而不是简单地计算分组中值的数量。**

***SQL 语句***

```bash
mysql> SELECT COUNT(customer_id) num_rows,
 -> COUNT(DISTINCT customer_id) num_customers
 -> FROM payment;
+----------+---------------+
| num_rows | num_customers |
+----------+---------------+
| 16049 | 599 |
+----------+---------------+
1 row in set (0.01 sec)

```

***SeaORM***

```rust
/// 在 `sea_orm` 中结合使用 `sea_query` 方法
/// ```
/// SELECT
///   COUNT(customer_id) num_rows,
///   COUNT(DISTINCT customer_id) num_customers
/// FROM
///   payment;
/// ```
async fn counting_distinct_values(db: &DatabaseConnection) -> Result<()> {
    let count = Payment::find()
        .select_only()
        .column_as(payment::Column::CustomerId.count(), "num_rows")
        .column_as(
            Expr::col(payment::Column::CustomerId).count(),
            "num_customers",
        )
        .all(db)
        .await?;
    println!("{:?}", count);
    Ok(())
}

```

> **Warning**
>
> 不知如何在 `sea_orm` 中结合使用 `sea_query`

#### 8.2.3　使用表达式 `datediff(return_date,rental_date)`

要求： **找出一部电影从被租借到后来归还之间相隔的最大天数**

***SQL 语句***

```bash
mysql> SELECT MAX(datediff(return_date,rental_date))
 -> FROM rental;
+----------------------------------------+
| MAX(datediff(return_date,rental_date)) |
+----------------------------------------+
| 33 |
+----------------------------------------+
1 row in set (0.01 sec)
```

***SeaORM***

```rust
#[derive(Debug, FromQueryResult)]
struct UsingExpressionsRes {
    max: i32,
}

struct Datediff;

impl Iden for Datediff {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "datediff").unwrap();
    }
}

/// 要求： **找出一部电影从被租借到后来归还之间相隔的最大天数**
/// ```
/// SELECT
///   MAX(
///      datediff(`return_date`, `rental_date`)
///   ) AS `max`
/// FROM
///   `rental`
/// ```
pub async fn using_expressions(db: &DatabaseConnection) -> Result<()> {
    let rental_date = Rental::find()
        .select_only()
        .column_as(
            Expr::expr(Func::cust(Datediff).args(vec![
                Expr::col(rental::Column::ReturnDate),
                Expr::col(rental::Column::RentalDate),
            ]))
            .max(),
            "max",
        )
        .into_model::<UsingExpressionsRes>()
        .all(db)
        .await?;
    println!("{:?}", rental_date);
    Ok(())
}
```

> **Note**
>
> 使用表达式
>
> `Expr::expr(Func::cust(Datediff).args(vec![
>                 Expr::col(rental::Column::ReturnDate),
>                 Expr::col(rental::Column::RentalDate),
>             ]))`

### 8.3　生成分组

#### 8.3.1　单列分组

要求： **查找某位演员参演的电影数量**

***SQL 语句***

```bash
mysql> SELECT actor_id, count(*)
 -> FROM film_actor
 -> GROUP BY actor_id;
+----------+----------+
| actor_id | count(*) |
+----------+----------+
| 1 | 19 |
...
| 200 | 20 |
+----------+----------+
200 rows in set (0.11 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{film_actor, prelude::*};
use anyhow::Result;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect};

#[derive(Debug, FromQueryResult)]
struct SingleColumnGroupingRes {
    actor_id: u16,
    count: i32,
}

/// 要求： **查找某位演员参演的电影数量**
/// ```
/// SELECT
///   `film_actor`.`actor_id`,
///   COUNT(*) AS `count`
/// FROM
///   `film_actor`
/// GROUP BY
///   `film_actor`.`actor_id`
/// ```
pub async fn single_column_grouping(db: &DatabaseConnection) -> Result<()> {
    let res = FilmActor::find()
        .select_only()
        .column(film_actor::Column::ActorId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(film_actor::Column::ActorId)
        .into_model::<SingleColumnGroupingRes>()
        .all(db)
        .await?;

    println!("{:?}", res);
    Ok(())
}
```

#### 8.3.2　多列分组

要求： **找出每位演员参演的各种分级电影（G、PG...）的数量**

***SQL 语句***

```bash
mysql> SELECT fa.actor_id, f.rating, count(*)
 -> FROM film_actor fa
 -> INNER JOIN film f
 -> ON fa.film_id = f.film_id
 -> GROUP BY fa.actor_id, f.rating
 -> ORDER BY 1,2;
+----------+--------+----------+
| actor_id | rating | count(*) |
+----------+--------+----------+
| 1 | G | 4 |
...
| 200 | NC-17 | 4 |
+----------+--------+----------+
996 rows in set (0.01 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{film, film_actor, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QueryOrder, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MulticolumnGroupingRes {
    actor_id: u16,
    count: i32,
}

/// 要求： **找出每位演员参演的各种分级电影（G、PG...）的数量**
/// ```
/// SELECT
///   `film_actor`.`actor_id`,
///   `film`.`rating`,
///   COUNT(*) AS `count`
/// FROM
///   `film_actor`
///   INNER JOIN `film` ON `film_actor`.`film_id` = `film`.`film_id`
/// GROUP BY
///   `film_actor`.`actor_id`,
///   `film`.`rating`
/// ORDER BY
///   `film_actor`.`actor_id` ASC,
///   `film`.`rating` ASC
/// ```
pub async fn multicolumn_grouping(db: &DatabaseConnection) -> Result<()> {
    let res = FilmActor::find()
        .select_only()
        .column(film_actor::Column::ActorId)
        .column(film::Column::Rating)
        .column_as(Expr::asterisk().count(), "count")
        .inner_join(Film)
        .group_by(film_actor::Column::ActorId)
        .group_by(film::Column::Rating)
        .order_by_asc(film_actor::Column::ActorId)
        .order_by_asc(film::Column::Rating)
        .into_model::<MulticolumnGroupingRes>()
        .all(db)
        .await?;

    println!("{:?}", res);
    Ok(())
}
```

#### 8.3.3　通过表达式分组 `extract(YEAR FROM rental_date)`

要求： **查询按年份对租借数据进行分组**

***SQL 语句***

```bash
mysql> SELECT extract(YEAR FROM rental_date) year,
 -> COUNT(*) how_many
 -> FROM rental
 -> GROUP BY extract(YEAR FROM rental_date);
+------+----------+
| year | how_many |
+------+----------+
| 2005 | 15862 |
| 2006 | 182 |
+------+----------+
2 rows in set (0.01 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    sea_query::{Expr, Func, Iden},
    DatabaseConnection, EntityTrait, FromQueryResult, QueryOrder, QuerySelect,
};

struct Extract;

impl Iden for Extract {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "extract").unwrap();
    }
}

#[derive(Debug, FromQueryResult)]
struct GroupingViaExpressionsRes {
    year: i32,
    how_many: i32,
}
/// 要求： **查询按年份对租借数据进行分组**
/// ```
/// SELECT
///   extract(
///       YEAR
///       FROM
///     `rental_date`
///   ) AS `year`,
///   COUNT(*) AS `how_many`
/// FROM
///   `rental`
/// GROUP BY
///   extract(
///      YEAR
///      FROM FROM
///     `rental_date`
///   )
/// ```
pub async fn grouping_via_expressions(db: &DatabaseConnection) -> Result<()> {
    let res = Rental::find()
        .select_only()
        .column_as(
            Func::cust(Extract).args(vec![
                Expr::expr(Expr::cust("YEAR FROM `rental_date`")),
                // Expr::col(rental::Column::RentalDate),
            ]),
            "year",
        )
        .column_as(Expr::asterisk().count(), "how_many")
        .group_by(Func::cust(Extract).args(vec![
            Expr::expr(Expr::cust("YEAR FROM `rental_date`")),
            // Expr::col(rental::Column::RentalDate),
        ]))
        .into_model::<GroupingViaExpressionsRes>()
        .all(db)
        .await?;

    println!("{:?}", res);
    Ok(())
}
```

> **Note**
>
> - `extract(YEAR FROM rental_date)`
>
> **Warning**
>
> 不会构造复杂的函数

#### 8.3.4　生成汇总 `with rollup`

要求： **在 `group by`子句中使用了`with rollup`选项，来生成汇总**

***SQL 语句***

```bash
mysql> SELECT fa.actor_id, f.rating, count(*)
 -> FROM film_actor fa
 -> INNER JOIN film f
 -> ON fa.film_id = f.film_id
 -> GROUP BY fa.actor_id, f.rating WITH ROLLUP
 -> ORDER BY 1,2;
+----------+--------+----------+
| actor_id | rating | count(*) |
+----------+--------+----------+
| NULL | NULL | 5462 |
| 1 | NULL | 19 |
| 1 | G | 4 |
| 1 | PG | 6 |
| 1 | PG-13 | 1 |
| 1 | R | 3 |
| 1 | NC-17 | 5 |
| 2 | NULL | 25 |
| 2 | G | 7 |
Generating Groups | 157
| 2 | PG | 6 |
| 2 | PG-13 | 2 |
| 2 | R | 2 |
| 2 | NC-17 | 8 |
...
| 199 | NULL | 15 |
| 199 | G | 3 |
| 199 | PG | 4 |
| 199 | PG-13 | 4 |
| 199 | R | 2 |
| 199 | NC-17 | 2 |
| 200 | NULL | 20 |
| 200 | G | 5 |
| 200 | PG | 3 |
| 200 | PG-13 | 2 |
| 200 | R | 6 |
| 200 | NC-17 | 4 |
+----------+--------+----------+
1197 rows in set (0.07 sec)
```

***SeaORM***

```rust

```

> **Warning**
>
> 在 `group by`子句中使用了`with rollup`选项，来生成汇总。
> 没有找到在 `sea-orm` 中如何使用

### 8.4　分组过滤条件

要求： **前者过滤掉评级不为G或PG的电影，后者过滤掉参演电影数少于10部的演员。**

***SQL 语句***

```bash
mysql> SELECT fa.actor_id, f.rating, count(*)
 -> FROM film_actor fa
 -> INNER JOIN film f
 -> ON fa.film_id = f.film_id
 -> WHERE f.rating IN ('G','PG')
 -> GROUP BY fa.actor_id, f.rating
 -> HAVING count(*) > 9;
+----------+--------+----------+
| actor_id | rating | count(*) |
+----------+--------+----------+
| 137 | PG | 10 |
| 37 | PG | 12 |
| 180 | PG | 12 |
| 7 | G | 10 |
| 83 | G | 14 |
| 129 | G | 12 |
| 111 | PG | 15 |
| 44 | PG | 12 |
| 26 | PG | 11 |
| 92 | PG | 12 |
| 17 | G | 12 |
| 158 | PG | 10 |
| 147 | PG | 10 |
| 14 | G | 10 |
| 102 | PG | 11 |
| 133 | PG | 10 |
+----------+--------+----------+
16 rows in set (0.01 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{film, film_actor, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct GroupFilterConditionsRes {
    actor_id: u16,
    rating: String,
    count: i32,
}
/// 要求： 评级为G或PG的电影并且参演电影数大于等于10部的演员。
/// ```
/// SELECT
///   `film_actor`.`actor_id`,
///   `film`.`rating`,
///   COUNT(*) AS `count`
/// FROM
///   `film_actor`
///   INNER JOIN `film` ON `film_actor`.`film_id` = `film`.`film_id`
/// WHERE
///   `film`.`rating` IN ('G', 'PG')
/// GROUP BY
///   `film_actor`.`actor_id`,
///   `film`.`rating`
/// HAVING
///   COUNT(*) > 9
/// ```
pub async fn group_filter_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = FilmActor::find()
        .select_only()
        .column(film_actor::Column::ActorId)
        .column(film::Column::Rating)
        .column_as(Expr::asterisk().count(), "count")
        .inner_join(Film)
        .filter(film::Column::Rating.is_in(["G", "PG"]))
        .group_by(film_actor::Column::ActorId)
        .group_by(film::Column::Rating)
        .having(Expr::expr(Expr::asterisk().count()).gt(9))
        .into_model::<GroupFilterConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
```

> **Note**
>
> - `is_in()`

### 9.1　什么是子查询

要求： **在单个查询中检索ID值最大的客户信息**

***SQL 语句***

```bash
mysql> SELECT customer_id, first_name, last_name
 -> FROM customer
 -> WHERE customer_id = (SELECT MAX(customer_id) FROM customer);
+-------------+------------+-----------+
| customer_id | first_name | last_name |
+-------------+------------+-----------+
| 599 | AUSTIN | CINTRON |
+-------------+------------+-----------+
1 row in set (0.27 sec)

```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Query, ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult,
    QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
pub struct SelectResult {
    customer_id: u16,
    first_name: String,
    last_name: String,
}

/// 要求： **在单个查询中检索ID值最大的客户信息**
/// ```
/// SELECT
///   `customer`.`customer_id`,
///   `customer`.`first_name`,
///   `customer`.`last_name`
/// FROM
///   `customer`
/// WHERE
///   `customer`.`customer_id` IN (
///     SELECT
///         MAX(`customer`.`customer_id`)
///     FROM
///         `customer`
///   )
///
/// ```
pub async fn what_is_a_subquery(db: &DatabaseConnection) -> Result<()> {
    let res = Customer::find()
        .select_only()
        .column(customer::Column::CustomerId)
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .filter(
            customer::Column::CustomerId.in_subquery(
                Query::select()
                    .expr(customer::Column::CustomerId.max())
                    .from(Customer)
                    .to_owned(),
            ),
        )
        .into_model::<SelectResult>()
        .all(db)
        .await?;
    println!("{:?}", res);
    Ok(())
}
```

> **Note**
>
> `WHERE` 中 `=` 后或 `IN` 后跟 `SELECT` 子查询 `sub_query`
>
> `.filter(
>
> ​            customer::Column::CustomerId.in_subquery(
>
> ​                Query::select()
>
> ​                    .expr(customer::Column::CustomerId.max())
>
> ​                    .from(Customer)
>
> ​                    .to_owned(),
>
> ​            ),
>
> ​        )`

### 9.3　非关联子查询

要求： **查询返回所有不在印度的城市**

***SQL 语句***

```bash
mysql> SELECT city_id, city
 -> FROM city
 -> WHERE country_id <> 
 -> (SELECT country_id FROM country WHERE country = 'India');
+---------+----------------------------+
| city_id | city |
+---------+----------------------------+
| 1 | A Corua (La Corua) |
...
| 600 | Ziguinchor |
+---------+----------------------------+
540 rows in set (0.02 sec)

```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{city, country, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Query, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
pub struct NoncorrelatedSubqueriesRes {
    city_id: u16,
    city: String,
}

/// 要求： **查询返回所有不在印度的城市**
/// ```
/// SELECT
///   `city`.`city_id`,
///   `city`.`city`
/// FROM
///   `city`
/// WHERE
///   `city`.`country_id` NOT IN (
///     SELECT
///         `country_id`
///     FROM
///         `country`
///     WHERE
///         `country`.`country` = 'India'
///   )
/// ```
pub async fn noncorrelated_subqueries(db: &DatabaseConnection) -> Result<()> {
    let res = City::find()
        .select_only()
        .column(city::Column::CityId)
        .column(city::Column::City)
        .filter(
            city::Column::CountryId.not_in_subquery(
                Query::select()
                    .column(country::Column::CountryId)
                    .from(Country)
                    .and_where(country::Column::Country.eq("India"))
                    .to_owned(),
            ),
        )
        .into_model::<NoncorrelatedSubqueriesRes>()
        .all(db)
        .await?;
    println!("{:?}", res);
    Ok(())
}
```

#### 9.3.1　多行单列子查询

**1.`IN` `NOT IN` 运算符**

>  `=` `<>` 只能和单个值进行比较。
>
> `IN` `NOT IN` 可以与一组值进行相等不想等比较

要求： **查找位于Canada或Mexico的所有城市**

***SQL 语句***

```bash
mysql> SELECT city_id, city
 -> FROM city
 -> WHERE country_id IN
 -> (SELECT country_id
 -> FROM country
 -> WHERE country IN ('Canada','Mexico'));
Noncorrelated Subqueries | 165
+---------+----------------------------+
| city_id | city |
+---------+----------------------------+
| 179 | Gatineau |
...
| 595 | Zapopan |
+---------+----------------------------+
37 rows in set (0.00 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{city, country, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Query, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
pub struct MultipleRowSingleColumnSubqueriesRes {
    city_id: u16,
    city: String,
}

/// 要求： **查找位于Canada或Mexico的所有城市**
/// ```
/// SELECT
///   `city`.`city_id`,
///   `city`.`city`
/// FROM
///   `city`
/// WHERE
///   `city`.`country_id` IN (
///     SELECT
///       `country_id`
///     FROM
///       `country`
///     WHERE
///       `country`.`country` IN ('Canada', 'Mexico')
///   )
/// ```
pub async fn multiple_row_single_column_subqueries(db: &DatabaseConnection) -> Result<()> {
    let res = City::find()
        .select_only()
        .column(city::Column::CityId)
        .column(city::Column::City)
        .filter(
            city::Column::CountryId.in_subquery(
                Query::select()
                    .column(country::Column::CountryId)
                    .from(Country)
                    .and_where(country::Column::Country.is_in(["Canada", "Mexico"]))
                    .to_owned(),
            ),
        )
        .into_model::<MultipleRowSingleColumnSubqueriesRes>()
        .all(db)
        .await?;
    println!("{:?}", res);
    Ok(())
}
```

**2．all运算符**

要求： **查询搜索所有从未获得过免费电影租借的客户**

***SQL 语句***

```bash
mysql> SELECT first_name, last_name
 -> FROM customer
 -> WHERE customer_id <> ALL
 -> (SELECT customer_id
 -> FROM payment
 -> WHERE amount = 0);
+-------------+--------------+
| first_name | last_name |
+-------------+--------------+
| MARY | SMITH |
...
| AUSTIN | CINTRON |
+-------------+--------------+
576 rows in set (0.01 sec)
```

***SeaORM***

```rust
#![allow(dead_code)]
use crate::entity::{city, country, customer, payment, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Query, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct AllOperatorRes {
    first_name: String,
    last_name: String,
}

/// 要求： **查询搜索所有从未获得过免费电影租借的客户**
/// ```
/// SELECT
///   `customer`.`first_name`,
///   `customer`.`last_name`
/// FROM
///   `customer`
/// WHERE
///   `customer`.`customer_id` NOT IN (
///     SELECT
///       `customer_id`
///     FROM
///       `payment`
///     WHERE
///       `payment`.`amount` = 0
///   )
/// ```
pub async fn all_operator(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .filter(
            customer::Column::CustomerId.not_in_subquery(
                Query::select()
                    .column(payment::Column::CustomerId)
                    .from(Payment)
                    .and_where(payment::Column::Amount.eq(0))
                    .to_owned(),
            ),
        )
        .into_model::<AllOperatorRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}
```

> **Note**
>
> `in`运算符可用于查看能否在一个表达式集合中找到某个表达式，`all`运算符则用于将某个值与集合中的所有值进行比较。构建这种条件时需要将比较运算符（`=`、`<>`、`<`、`>`等）与`all`运算符配合使用
>
> 大多数人更喜欢换一种方式编写查询，而避免使用all运算符。下列查询使用`not in`运算符，生成的结果和上一个示例一模一样。`not in` 的版本更易于理解



要求： **查询返回所有北美洲客户租借的电影总数，包含查询返回电影租借总数超过任何北美洲客户的全部客户**

***SQL 语句***

```bash
mysql> SELECT customer_id, count(*)
 -> FROM rental
 -> GROUP BY customer_id
 -> HAVING count(*) > ALL
 -> (SELECT count(*)
 -> FROM rental r
 -> INNER JOIN customer c
 -> ON r.customer_id = c.customer_id
 -> INNER JOIN address a
 -> ON c.address_id = a.address_id
 -> INNER JOIN city ct
 -> ON a.city_id = ct.city_id
 -> INNER JOIN country co
 -> ON ct.country_id = co.country_id
 -> WHERE co.country IN ('United States','Mexico','Canada')
 -> GROUP BY r.customer_id
 -> );
+-------------+----------+
| customer_id | count(*) |
+-------------+----------+
| 148 | 46 |
+-------------+----------+
1 row in set (0.01 sec)
```

***SeaORM***

```rust

```

> **Note**



