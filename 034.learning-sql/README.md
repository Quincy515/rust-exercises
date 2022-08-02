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
- [4.1 条件评估](#41-条件评估)
- [4.2 构建条件](#42-构建条件)
- [4.3 条件类型](#43-条件类型)
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

### 4.1 条件评估

***SQL 语句***

***SeaORM***

### 4.2 构建条件

***SQL 语句***

***SeaORM***

### 4.3 条件类型

***SQL 语句***

***SeaORM***

### 4.4 null:  4个字母的单词

***SQL 语句***

***SeaORM***
