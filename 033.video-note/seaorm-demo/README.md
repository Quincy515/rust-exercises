- [Sqlite](#sqlite)
  - [创建单词表](#创建单词表)
  - [Insert语句](#insert语句)
  - [单词书](#单词书)
  - [听写表](#听写表)
  - [听写记录](#听写记录)
  - [错题本](#错题本)
- [步骤](#步骤)
  - [1. Generate a new migration file](#1-generate-a-new-migration-file)
  - [2. 生成实体](#2-生成实体)
- [增删改查](#增删改查)
  - [字典](#字典)
    - [1. 初始化字典 init_dict](#1-初始化字典-init_dict)
    - [2. 新增单词 create_word](#2-新增单词-create_word)
    - [3. 单词列表 get_word_list](#3-单词列表-get_word_list)
    - [4. 查询单词 find_word](#4-查询单词-find_word)
    - [5. 修改单词和释义 update_word](#5-修改单词和释义-update_word)
    - [6. 删除单词 delete_word](#6-删除单词-delete_word)
    - [7. 批量删除单词 delete_word_list](#7-批量删除单词-delete_word_list)
  - [听写](#听写)
    - [1. 新增听写 create_dictation](#1-新增听写-create_dictation)
    - [2. 新增记录 create_dictation_record](#2-新增记录-create_dictation_record)
    - [3. 订正记录 update_dictation_record](#3-订正记录-update_dictation_record)
    - [4. 错题本 create_wong_book](#4-错题本-create_wong_book)
    - [5. 结束听写 end_dictation](#5-结束听写-end_dictation)
    - [6. 新增听写时选择单词](#6-新增听写时选择单词)
- [参考](#参考)

## Sqlite
### 创建单词表
```sql
CREATE TABLE "word" (
  "word" text NOT NULL DEFAULT '',
  "translation" text NOT NULL DEFAULT NULL,
  PRIMARY KEY ("word")
);
```

### Insert语句
```sql
INSERT INTO `word` (`word`, `translation`) VALUES
('accelerate', 'v.加速,促进'),
('accelerated', 'adj.加速的');
```

### 单词书
```sql
CREATE TABLE "book" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "level" integer NOT NULL DEFAULT 0,
  "order" integer NOT NULL DEFAULT 0,
  "name" TEXT NOT NULL DEFAULT '',
  "num" INTEGER NOT NULL DEFAULT 0,
  "author" TEXT NOT NULL DEFAULT '',
  "book" TEXT NOT NULL DEFAULT '',
  "comment" TEXT NOT NULL DEFAULT '',
  "orgnization" TEXT NOT NULL DEFAULT '',
  "publisher" TEXT NOT NULL DEFAULT '',
  "version" TEXT NOT NULL DEFAULT '',
  "flag" TEXT NOT NULL DEFAULT ''
);
```

### 听写表
```sql
CREATE TABLE "dictation" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" text,
  "start_at" text,
  "end_at" text,
  "total" integer,
  "correct" TEXT,
  "wrong" integer
);
```

### 听写记录
```sql
CREATE TABLE "dictation_record" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "word" TEXT,
  "sentence" TEXT,
  "spell" TEXT,
  "flag" integer NOT NULL DEFAULT 0,
  "dictation_id" INTEGER,
);
```

### 错题本
```sql
CREATE TABLE "wong_book" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "word" TEXT,
  "sentence" TEXT,
  "frequency" integer NOT NULL DEFAULT 0,
  "dictation_id" INTEGER,
);
```

## 步骤
### 1. Generate a new migration file
```bash
export DATABASE_URL='sqlite://data.sqlite?mode=rwc'
sea-orm-cli migrate generate dictation
```

编辑文件 `migration/src/m202*****_000001_***.rs` 删除 `todo!()` 并替换表结构。

当修改好了自己的表结构，接下来，将运行 migrate up，将创建我们刚新建的表结构。

```bash
sea-orm-cli migrate up
```
### 2. 生成实体

```bash
sea-orm-cli generate entity -o entity/src
```
由于entity位于项目的根目录，我们将其转换为一个lib，这样我们就可以调用它。

将 `entity/src/mod.rs` 文件重命名为 `entity/src/lib.rs`。

## 增删改查
### 字典
#### 1. 初始化字典 init_dict 
#### 2. 新增单词 create_word
#### 3. 单词列表 get_word_list
#### 4. 查询单词 find_word
#### 5. 修改单词和释义 update_word
#### 6. 删除单词 delete_word
#### 7. 批量删除单词 delete_word_list
### 听写
#### 1. 新增听写 create_dictation
create 该次听写的名称，自动获取当前创建时间
#### 2. 新增记录 create_dictation_record
create 记录该次听写的id和听到的单词
#### 3. 订正记录 update_dictation_record
update 填写正确的单词，记录是否正确
#### 4. 错题本 create_wong_book
create 记录该次的听写编号，新增错误单词到错题本，并把错误次数加一
#### 5. 结束听写 end_dictation
update 更新该次听写的总个数，错误的个数，正确率
#### 6. 新增听写时选择单词
首页1.新增听写，获得该次听写的编号，再2.新增记录，但记录的是该次准备听写的单词

## 参考
学习自：https://mp.weixin.qq.com/s/3orOZZizPkDttE4jCh0dAg

翻译自：
https://dev.to/anshulxyz/guide-to-getting-started-with-seaorm-an-orm-for-rust-2fen

![](./note.png)