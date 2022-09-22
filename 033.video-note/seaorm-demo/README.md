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

### 听写结果
```sql
CREATE TABLE "dictation_result" (
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

### 3. 增删改查


## 参考
学习自：https://mp.weixin.qq.com/s/3orOZZizPkDttE4jCh0dAg

翻译自：
https://dev.to/anshulxyz/guide-to-getting-started-with-seaorm-an-orm-for-rust-2fen

![](./note.png)