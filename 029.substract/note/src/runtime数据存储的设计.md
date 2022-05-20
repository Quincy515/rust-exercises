## 区块链存储的不同点和约束

区块链应用通常几个特点，

- 开源可审查，对等节点，引入延迟和随机来达到共识
- **链式**、**增量地存储数据**

区块链应用的节点软件依赖高效的键值对数据库：

- **LevelDB**
- **RocksDB**

 区块链作为业务的载体，存储相关的限制有：
- 大文件直接存储在链上的成本很高；
- 链式的区块存储结构**不利于对历史数据的索引**；
- 另外一个约束是，在进行数值运算时**不能使用浮点数**。

## Substrate 存储单元的类型

开发链上存储单元的特点：

- Rust原生数据类型的子集，定义在核心库和 alloc 库中
- 原生类型构成的映射类型
- 满足一定的编解码条件

| 单值     |
| -------- |
| 简单映射 |
| 双键映射 |

### 回顾  `storage ` 宏

```rust
#[pallet::storage]
#[pallet::getter(fn something)]
pub type Something<T> = StorageValue<_, u32>;
```

### 单值类型

存储某种单一类型的值，如布尔，数值，枚举，结构体等：

- 数值：`u8`, `i8`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`
- 大整数：`U128`, `U256`, `U512`
- 布尔：`bool`
- 集合：`Vec<T>`, `BTreeMap`, `BTreeSet`
- 定点小数：`Percent`, `Permill`, `Perbill`, `FixedU128`
- 定长哈希：`H128`, `H256`, `H512`
- 其它复杂类型：`Option<T>`, `tuple`, `enum`, `struct`
- 内置自定义类型：`Moment`, `AccountId`

数值类型 `u8` 的定义：

 ```rust
 #[pallet::storage]
 #[pallet::getter(fn my_unsigned_number)]
 pub type MyUnsignedNumber<T> = StorageValue<_, u8>;
 
 #[pallet::storage]
 #[pallet::getter(fn my_signed_number)]
 pub type MySignedNumber<T> = StorageValue<_, i8, ValueQuery>;
 ```

数值类型 `u8`, `i8`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128` 的使用：

- 增：`MyUnsignedNumber::<T>::put(number);`
- 查：`MyUnsignedNumber::<T>::get();`
- 改：`MyUnsignedNumber::<T>::mutate(|v| v + 1);`
- 删：`MyUnsignedNumber::<T>::kill();`

更多 API，请参考文档 https://crates.parity.io/frame_support/pallet_prelude/struct.StorageValue.html

数值类型 `u8`, `i8`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128` 的安全操作：

- 返回Result类型：`checked_add`, `checked_sub`, `checked_mul`, `checked_div`

```rust
// fail the transaction if error
my_unsigned_num.checked_add(10)?; 
```

- 溢出返回饱和值：`saturating_add`, `saturating_sub`, `saturating_mul`

```rust
// result is 255 for u8
my_unsigned_num.saturating_add(10000); 
```

大整数 `U256`, `U512` 类型定义：

 ```rust
 use sp_core::U256;
 
 #[pallet::storage]
 #[pallet::getter(fn my_big_integer)]
 pub type MyBigInteger<T> = StorageValue<_, U256>;
 ```

操作：`checked_add`, `overflowing_mul`... 

更多API，参考文档 https://crates.parity.io/sp_core/struct.U256.html

 `bool` 类型定义：

```rust
#[pallet::storage]
#[pallet::getter(fn my_bool)]
pub type MyBool<T> = StorageValue<_, bool>;
```

- If else 逻辑判断
- 对于 `ValueQuery`, 默认值为 false

 `Vec<T>` 类型定义：

 ```rust
 use sp_std::prelude::*;
 
 #[pallet::storage]
 #[pallet::getter(fn my_string)]
 pub type MyString<T> = StorageValue<_, Vec<u8>>;
 ```

- 操作：`push`, `pop`, `iter`… https://doc.rust-lang.org/alloc/vec/struct.Vec.html
- 对于 `ValueQuery`, 默认值为 `0x00`

`Percent`, `Permill`, `Perbill` 类型定义：

```rust
use sp_runtime::Permill;

#[pallet::storage]
#[pallet::getter(fn my_permill)]
pub type MyPermill<T> = StorageValue<_, Permill>;
```

`Percent`, `Permill`, `Perbill` 类型操作：

- 构造
  - `Permill::from_percent(value);`
  - `Permill::from_parts(value);`
  - `Permill::from_rational(p,q);`

- 计算
  - `permill_one.saturating_mul(permill_two);`
  - `my_permill * 20000 as u32`

API 文档 https://crates.parity.io/sp_runtime/struct.Permill.html

`Moment` 时间类型定义：

```rust
#[pallet::config]
pub trait Config: pallet_timestamp::Config: + frame_system::Config {
    // -- snippet --
}

#[pallet::storage]
#[pallet::getter(fn my_time)]
pub type MyTime<T: Config> = StorageValue<_, T::Moment>;
```

- `Moment` 是 `u64` 的类型别名 
- 获取链上时间：`pallet_timestamp::Pallet::<T>::get()`

`AccountId` 账户类型定义：

```rust
#[pallet::storage]
#[pallet::getter(fn my_account_id)]
pub type MyAccountId<T: Config> = StorageValue<_, T::AccountId>;
```

- 定义在 `frame_system` 中，通常是 Public key
- 获取 `AccountId`: `let sender = ensure_signed(origin)?`

 `struct` 类型定义：

 ```rust
 #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
 pub struct People {
     name: Vec<u8>,
     age: u8,
 }
 
 #[pallet::storage]
 #[pallet::getter(fn my_struct)]
 pub type MyStruct<T: Config> = StorageValue<_, People>;
 ```

`enum` 类似，还需要实现 Default 接口

- https://github.com/kaichaosun/play-substrate/blob/master/pallets/data-type/src/lib.rs#L97

### 简单映射类型

`StorageMap` 类型，用来保存键值对，单值类型都可以用作 key 或者 value。

```rust
#[pallet::storage]
#[pallet::getter(fn my_map)]
pub type MyMap<T> = StorageMap<
	_,
	Blake2_128Concat,
	u8,
	Vec<u8>,
>;
```

key 的哈希算法：`Blake2_128Concat`, `Twox64Concat`, `Identity

- 插入一个元素：`MyMap::<T>::insert(key, value);`
- 通过key获取value：`MyMap::<T>::get(key);`
- 删除某个key对应的元素：`MyMap::<T>::remove(key);`
- 覆盖或者修改某个 key 对应的元素
  - `MyMap::<T>::insert(key, new_value);`
  - `MyMap::<T>::mutate(key, |old_value| old_value+1);`

API 文档：https://crates.parity.io/frame_support/pallet_prelude/struct.StorageMap.html

https://crates.parity.io/frame_support/storage/trait.IterableStorageMap.html

### 双键映射类型 

`StorageDoubleMap` 类型，使用两个 key 来索引 value，用于快速删除 key1 对应的任意记录，也可以遍历 key1 对应的所有记录，定义：

```rust
#[pallet::storage]
#[pallet::getter(fn my_double_map)]
pub type MyDoubleMap<T: Config> = StorageDoubleMap<
	_,
	Blake2_128Concat,
	T::AccountId,
	Blake2_128Concat,
	u8,
	Vec<u8>,
>;
```

- 插入一个元素：`MyDoubleMap::<T>::insert(key1, key2, value);`
- 获取某一元素：`MyDoubleMap::<T>::get(key1, key2);`
- 删除某一元素：`MyDoubleMap::<T>::remove(key1, key2);`
- 删除 key1 对应的所有元素：`MyDoubleMap::<T>::remove_prefix(key1, None);`

API 文档 https://crates.parity.io/frame_support/pallet_prelude/struct.StorageDoubleMap.html

https://crates.parity.io/frame_support/storage/trait.IterableStorageDoubleMap.html

## 存储的初始化

创世区块的数据初始化：

```rust
#[pallet::genesis_config]
pub struct GenesisConfig {
    pub value: u8,
}

#[cfg(feature = "std")]
impl Default for GenesisConfig<T> {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}

#[pallet::genesis_build]
impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
        MyValue::<T>::put(&self.value);
    }
}
```

演示: https://github.com/paritytech/substrate/blob/master/frame/sudo/src/lib.rs

## 最佳实践

- 最小化链上存储
  - 哈希值
  - 设列表容量
- Verify First, Write Last
- 事务管理：Transactional macro

### 其它 Tips

- 可以通过 pub 关键字设置存储单元的可见范围
- `ValueQuery` 设置默认值，如: https://github.com/kaichaosun/play-substrate/blob/master/pallets/data-type/src/lib.rs#L42
- 在 frame 目录下查找对应的最新用法
- [pallet::storage](https://crates.parity.io/frame_support/attr.pallet.html#storage-palletstorage-optional) 宏的说明文档

 

