简单看一下 pallet，有个印象，
- 了解 substrate 的架构？
- pallet 的层次？

substraet-node-template 的目录命名方式和 substrate 的分层概念很贴近。
```
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── docker-compose.yml
├── docs
├── node
├── pallets
├── runtime
├── rustfmt.toml
├── scripts
├── shell.nix
└── target
```

- node: node 层里可以填写一些配置，控制生成最后的可执行二进制文件
- pallets: runtime 的最小单元，目前 pallets 里面提供了一个 template 示例，演示如何写 pallet
- runtime: 所有的 pallet 组装成 runtime 的逻辑代码。

利用 substrate 定制自己的区块链，写业务逻辑最主要就是写不同的 pallet。

首先可以看到 pallet 里面有各种各样的宏

```rust
//! pallets/template/src/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Something<T>>::put(something);
			Self::deposit_event(Event::SomethingStored(something, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			match <Something<T>>::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
```
注意看这里
```rust
//! pallets/template/src/lib.rs
#[frame_support::pallet]
pub mod pallet {
    // 1. 导入
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    
    // 2. 配置 config
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    
    // 必须写的
    #[pallet::pallet]
    // https://github.com/paritytech/substrate/issues/10652
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    // 3. 声明存储 storage，对于每个存储到需要 storage 宏修饰
    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;
    
    // 4. 声明事件 event
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {}
    
    // 5. 自定义 error，public function 调用失败的返回
    #[pallet::error]
    pub enum Error<T> {}
    
    // 6. public function (user-callabled) / private function 
    // 用户可调用的公共方法或用户不可调用的私有方法
    // public function = Dispatch function return DispatchResult
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // weight 调用方法的手续费
        #[pallet::weight(0)]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {}
    }
}
```
其中每一个组件都需要用一个主要的宏进行声明。可以使用命令

```sh
cargo expand -p pallet-template > template-expand.rs
```

`pallet-template` 为 `Cargo.toml` 中定义的 `name`，展开宏的代码，使用宏可以大大减少代码的编写，只关注于业务逻辑，宏的主要作用，是给我们写的 pallet 实现各种 Trait 来方便与底层交互。

runtime 就是把不同的 pallet 组装成最终的状态转换函数。把需要组装的 pallet 放入下面

```rust
//! runtime/src/lib.rs
// [...]

/// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
    type Event = Event;
}

impl pallet_poe::Config for Runtime {
    type Event = Event;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		// Include the custom logic from the pallet-template in the runtime.
		TemplateModule: pallet_template,
		Poe: pallet_poe,
	}
);
// [...]
```

在 pallet 中定义的抽象类型，如

```rust
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
```

实际在 runtime 中进行赋具体的类型

```rust
//! runtime/src/lib.rs
// [...]
impl frame_system::Config for Runtime {
    // [...]
}
impl pallet_randomness_collective_flip::Config for Runtime {}
impl pallet_aura::Config for Runtime {
	// [...]
}
impl pallet_aura::Config for Runtime {
	// [...]
}
impl pallet_timestamp::Config for Runtime {
	// [...]
}
impl pallet_balances::Config for Runtime {
	// [...]
}
impl pallet_transaction_payment::Config for Runtime {
	// [...]
}
impl pallet_sudo::Config for Runtime {
    // [...]
}
impl pallet_template::Config for Runtime {
    type Event = Event;
}
impl pallet_poe::Config for Runtime {
    type Event = Event;
}
// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		// Include the custom logic from the pallet-template in the runtime.
		TemplateModule: pallet_template,
		Poe: pallet_poe,
	}
);
```

通过 `construct_runtime!` 把 pallet 组装成 runtime，以一个整体与 node 或者底层进行交互。

从通用工程理解 `Config`，比如现在需要往链上保存数据，是需要成本的。所以使用 `pallet-balances` ，它是控制个人 token 余额。

第一步添加依赖到 `Cargo.toml`

```toml
#! pallets/template/Cargo.toml
# [...]
[dependencies]
pallet-balances = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.18" }
# [...]
# rust 编译标签进行条件编译
[features]
default = ["std"]
std = [
	"codec/std",
	"scale-info/std",
	"frame-support/std",
	"frame-system/std",
	"frame-benchmarking/std",
	# new
	"pallet-balabces/std"
]
# [...]
```

在 `call` 函数中使用

```rust
//! pallets/template/src/lib.rs
#[frame_support::pallet]
pub mod pallet {
    // 1. 导入
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    
	// 2. 修改 config
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_balances::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    
    // [...]
    
	#[pallet::call]
    impl<T: Config> Pallet<T> {
        // weight 调用方法的手续费
        #[pallet::weight(0)]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // updating free. pallet-balances
            pallet_balances::pallet::Pallet::<T>::decrease_balance(&sender, 1u32.into());
            // [...]
        }
    }
    // [...]
}
```

为了解决因为引入各种依赖，而需要处理的问题，利用 substrate 强大的抽象能力，

```rust
//! pallets/template/src/lib.rs
#[frame_support::pallet]
pub mod pallet {
    // 1. 导入
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use frame_support::traits::Currency;
    
	// 2. 修改 config
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // 声明通用类型 Currency，它符合 Currency trait 的约束
        type Currency: Currency<Self::AccountId>;
    }
    
    // [...]
    
	#[pallet::call]
    impl<T: Config> Pallet<T> {
        // weight 调用方法的手续费
        #[pallet::weight(0)]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // updating free. pallet-balances
            T::Currency::slash(&sender, 1u32.into());
            // [...]
        }
    }
    // [...]
}
```

甚至可以把 `Cargo.toml` 中关于 `pallet-balances` 依赖删除，然后在 `runtime` 中才具体指定类型

```rust
//! runtime/src/lib.rs
// [...]
impl pallet_template::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
}
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances, // 和这里对应
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		// Include the custom logic from the pallet-template in the runtime.
		TemplateModule: pallet_template,
		Poe: pallet_poe,
	}
);
// [...]
```

这样就不需要在 pallet 中手动引入依赖，从而避免 pallet 和 pallet 之间的相互调用。这样可以做到 substrate 很灵活，可维护性非常强。