## substrate-node-template 

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

- node 生成程序最后的二进制文件，在本机运行，控制程序的启动、网络连接、数据库等底层的交互。
- runtime 执行所有链上的逻辑，runtime 会编译成 wasm，存放在链上，当链上触发 function dispatch 时候会去 node 中执行。
- pallets 不同的功能是不同的 pallet，所有 pallet 组合起来是 runtime 的执行逻辑，有利于不同业务之间的模块化和隔离。

对于 substrate 和其他链会有一些区别，substrate 主要有三部分，第一个是 node 生成的二进制程序在本机执行，第二个部分是 runtime 生成的 wasm 代码，pallet 复制各种具体业务。第三层是用户层的链上逻辑，通过智能合约来实现，substrate-node-template 项目中没有包含智能合约部分的代码。这是因为对于 substrate 来说，很多链上逻辑可以包含在 pallet 的链本身，然后，用户所定义的一些逻辑和 runtime 最大的一个区别就是说。contract 其实是任何一个人都可以在链上去部署。它的安全性其实是有开发这种 Dapp 或者 contract 的人来保障。但对于链来说，他其实是在是开发这条区块链。不管是 substrate 底层实现开发者，他来保证这个链的安全性，所以它的升级等都是会通过，一些链上的治理或者是比如在 sudo 没有移除之前由 sudo 来做这些事情，但是对于 contract，他其实是相当于任何人都可以去部署、管理。这就是 substrate 与其他链最大的一个区别。

## node

node 其实就是要把所有区块链的这些组件给它启动起来，它最基本的像数据库、网络连接、线程的管理，包括这个交易池的管理，这些都是需要在 node 这部分初始化，然后启动起来。main 函数只是把运行 command。

### `main.rs`

```rust
//! node/src/main.rs
//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
```

### `command.rs`

`command.rs` 中有一些除了运行链启动之外的 subcommand

```rust
//! node/src/command.rs
// [...]
Some(Subcommand::PurgeChain(cmd)) => {
    let runner = cli.create_runner(cmd)?;
    runner.sync_run(|config| cmd.run(config.database))
},
// [...]
```

比如 `PurgeChain` 删除链上之前的状态和数据等。

### `chain_spec.rs`

`chain_spec.rs` 里主要定义链启动时的参数，比如说有一些什么账号，每个账号初始的 balance 是多少等。local 的环境是单节点的启动。

```rust
//! node/src/chain_spec.rs
// [...]
pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	// [...]
}
// [...]
```

### `service.rs`

最主要的逻辑代码是放在 `service.rs` 。`service` 就是如何来组装整个 node 节点的执行代码首先。

```rust
type FullClient =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub fn new_partial(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			sc_finality_grandpa::GrandpaBlockImport<
				FullBackend,
				Block,
				FullClient,
				FullSelectChain,
			>,
			sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other("Remote Keystores are not supported.".into()))
	}

	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			&config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

	let import_queue =
		sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _, _>(ImportQueueParams {
			block_import: grandpa_block_import.clone(),
			justification_import: Some(Box::new(grandpa_block_import.clone())),
			client: client.clone(),
			create_inherent_data_providers: move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
					sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
						*timestamp,
						slot_duration,
					);

				Ok((timestamp, slot))
			},
			spawner: &task_manager.spawn_essential_handle(),
			can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(
				client.executor().clone(),
			),
			registry: config.prometheus_registry(),
			check_for_equivocation: Default::default(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		})?;

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (grandpa_block_import, grandpa_link, telemetry),
	})
}
```

它有一个函数，`new_partial` 这个里面其实是把 `FullClient`，后端的这个节点 `FullBackend`，`FullSelectChain` ，共识的机制，比如这里选择的是最长的这个链 `type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;`，对于每一个交易的 `DefaultImportQueue` ，还有这个交易池 `FullPool<Block, FullClient>`，就是如何在内存里去存放交易。还有 `grandpa` 用于在 `finalize` 每一个 `block`的。就是运行 `grandpa` 的节点会对一个区块进行投票，当有 2/3 的节点投某一个区块的时候，那么这个区块就会被 `finalize` 下来，以后该区块就不能 `revert`。

```rust
pub fn new_partial(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			sc_finality_grandpa::GrandpaBlockImport<
				FullBackend,
				Block,
				FullClient,
				FullSelectChain,
			>,
			sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {}
```

在 **init partial** 后端的功能的时候。首先 `keystore_remote`，是存放我们的一些有用的公私钥对。

```rust
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other("Remote Keystores are not supported.".into()))
	}
```

`telemetry` 是用于统计。主要是通过 web 界面可以把网络里面每一个节点运行情况给展现出来，比如每个节点它同步到了哪一个区块，有没有出块，等等。这些可以有利于监控对整个网络的情况。

```rust
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;
```

`executor`  链上的运行其实是有两种模式。一种就是 **native**，另一种就是 **wasm**，**native** 就是直接执行机器代码，**wasm** 执行的是 **runtime** 所编译出来的 **wasm** 代码。它是选择用哪种方式来运行链上的逻辑。

```rust
	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);
```

如果想要更加详细的了解，可以到 **substrate** 代码库去查看。这里只是了解一下基本的流程。

```rust
	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			&config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
```

`task_manager` 是做不同线程的管理。当这些东西都初始化好了之后，他最后会把所需要的这些 PartialComponents 都返回回去。

```rust
	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (grandpa_block_import, grandpa_link, telemetry),
	})
```

`new_full` 里面首先会去调用上面的 `new_partial`，获取初始化好的资源，然后后面主要就是对 **network** 的启动。

```rust
pub fn new_full(mut config: Configuration) -> Result<TaskManager, ServiceError> {
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		mut keystore_container,
		select_chain,
		transaction_pool,
		other: (block_import, grandpa_link, mut telemetry),
	} = new_partial(&config)?;
    // [...]
    network_starter.start_network();
	Ok(task_manager)
}

```

比如说在启动网络之前会把 `grandpa` 节点的 **peer to teer** 连接。

```rust
	config
		.network
		.extra_sets
		.push(sc_finality_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));
	let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		grandpa_link.shared_authority_set().clone(),
		Vec::default(),
	));
```

然后当 `build_network` 之后，他会得到 **network** 句柄。

```rust
	let (network, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync: Some(warp_sync),
		})?;
```

然后在 substrate 里面，和其他链比较大的优势，他有 `offchain_worker` 这个功能。 `offchain_worker`就是链上面的数据、交易等可以得到存储和更新之外。还有，和链下的一些东西去进行交互。比如说我一个链想取得某个时候的天气情况，或者是某个时间的价格指数等，这个时候往往是不能在链上能够完成的。比如说还有一些比较费时的计算。有可能它会比一个出块的时间还长，比如说，它需要半分钟或者一分钟才能够完成。这些事情都可以交由 `offchain_worker` 来做。当 `offchain_worker` **enable** 的时候，他也会把  `offchain_worker` 这个 **thread** 启动起来。

```rust
	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}
```

后面还有 **rpc** 的启动。就是一个节点他会向外面提供一些 **rpc** 的服务，这样的话你可以跟他进行交互，得到链上的一些状态，或者是通过 **rpc** 去提交一些交易，这个都是由 **rpc** 来做的，然后它也是通过 **spwan** 一个新的 **thread** 来做这些事情。

```rust
	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();

		Box::new(move |deny_unsafe, _| {
			let deps =
				crate::rpc::FullDeps { client: client.clone(), pool: pool.clone(), deny_unsafe };

			Ok(crate::rpc::create_full(deps))
		})
	};

	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		rpc_extensions_builder,
		backend,
		system_rpc_tx,
		config,
		telemetry: telemetry.as_mut(),
	})?;
```

然后，如果它是一个**authority** 的一个节点的话，那他就需要进行打包，目前在这个 node-template 里使用的是 `Aura` 协议， `Aura` 是最简单的共识算法。就是说在很多节点组成的一个 **pool** 里面，按照 **index** 轮流的出块。嗯，当然，我们在实际的产品当中可能不会用这种，用的更多的可能像是 Beb 做一些选择，它是会有一些链上的随机数来保证这个选择是更加安全，也不会被攻击的。然后 **spawn**  **AURA** 这个 **thread**。

```rust
	if role.is_authority() {
        // [...]
		let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _, _>(
			// [...]
		)?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager
			.spawn_essential_handle()
			.spawn_blocking("aura", Some("block-authoring"), aura);
	// [...]
```

`keystore` 做公私钥对的存储

```rust
	let keystore =
		if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };
```

还有 `grandpa` 是最后对块进行确认的。因为在 substrate 里面其实是一个两层的共识逻辑，第一个就是我们用 Beb 去选择哪个节点出块，然后这个块会被广播到整个网络里面去，然后，但他出了块之后，另外一个算法就是 `grandpa` 它会去对这些块进行投票，最后来确认哪些块，是会被 `finalize`。当这个节点参与到 `grandpa` 共识的话，他也会把相应的线程给启动起来。

```rust
	if enable_grandpa {
		let grandpa_config = sc_finality_grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network,
			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state: SharedVoterState::empty(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
		);
	}
```

然后最后启动的就是一个网络。

```rust
	network_starter.start_network();
```

对于一个 node 节点来说，只有启动网络开始跟其他的节点进行同步的时候，上面所有这些逻辑包括交易池、**rpc**服务，或者是链上状态的改变，这些东西才可以运行起来。这就是 node 这一部分代码所做的一些事情。node 对于 substrate 来说是最底层的。然后基于 node 就会运行一个 runtime，这个 runtime 就是所有的链上的逻辑。

## runtime

runtime 是基于 node 节点，可以对 wasm 进行 distpetch 和 executor 的环境，runtime，其实做的最主要的工作就是一些参数的设定。对常数和 **triat** 的绑定，还有就是把所有 pallet 的组和起来。**pallet** 就是链上的业务逻辑的模块。

比如可以把很多参数放在这里来定义。

```rust
pub type BlockNumber = u32;
pub type Balance = u128;
```

定义 runtime 的版本，比如 spec 的版本，impl 的版本，还有其他一些比如 API 的版本

```rust
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("node-template"),
    impl_name: create_runtime_str!("node-template"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
    //   the compatible custom types.
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
```

在升级的时候，应该记得对一些参数进行调整。这样的话。当 node 节点去比较这些版本的时候时候就知道哪个版本是最新的，它会去做一些 upgrade。

比如说，还有其他的一些参数，像一个块的时间，这里定义的六秒钟。

```rust
pub const MILLISECS_PER_BLOCK: u64 = 6000;
```

以 **frame** 开头的 pallet，它其实是系统级的一个pallet。它里面包含了很多最重要的一些参数的设定，比如 AccountId 是什么 type，比如说 block 的长度等等。这些最重要的参数都是在 system 里面进行绑定的。

```rust
impl frame_system::Config for Runtime {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    // [...]
```

参与共识的两个模块，aura 和 grandpa，然后相对应的他们也会有一些数据的存储在链上。

```rust
impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
}

impl pallet_grandpa::Config for Runtime {
    // [...]
}
```

`timestamp` 是提供系统的时间。

```rust
impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}
```

然后 `balance` 就是最重要的，他会去存储每一个用户账户金额。

```rust
impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}
```

`sudo` 是最高权限管理，一般来说我们会在一个网络到达成熟的阶段之后，会把 `sudo` 移除掉，然后是交由社区来进行管理。`[democracy](https://github.com/paritytech/substrate/tree/master/frame/democracy)`这种模块来,来做一些有最高权限的一些工作，比如像升级 runtime 或者是做一些链上的操作，都是用其他的 pallet 来做共识。而不是简单的使用 `sudo`，当然在一开始是非常方便去做一些链上的处理。因为投票，包括提案，往往需要的时间很长。

```rust
impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}
```

在 `construct_runtime!` 这个宏里面，其实他就是把上面使用的 pallet 给全部放放进来了。在 polkadot.js 访问链上的数据时，会使用这里定义的变量。

```rust
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

这个就是整个runtime的逻辑，他所起到的作用就是把所有的 pallet，所有在链上所需要进行的存储啊，所需要 dispatch 的这些函数。全部都放到这个里面来，然后最终编译出来的就是一个 wasm。

还有对外提供的一些 api 服务，像比如说得到这个runtime的版本，或者是其他这些。

```rust
impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}
// [...]
```

## pallet

对于 pallet 来说，也是我们在以后开发独有的区块链接触到最多的一个。因为大部分的链来说去修改它的，比如说共识机制、交易池，或者是 p2p 的网络连接，都是相对来说会少一点。但是对于业务逻辑的定制化，其实是做的最多的。

这个 pallet 其实是非常有意思一个概念，因为最早的时候不叫这个名字，后来因为 substrate 设计者希望把整个构建过程像做一个艺术画作一样，来进行的，所以他最后把这个模块的名字叫做 pallet（调色板）。其实从软件的角度来说，它就是 runtime 的一个mod。大家能够这么去理解就好了。

pallet 是怎么样来定义的？首先所有的 pallet 都会放到 `pub mod pallet{}` 里面。然后他所最依赖的 pallet，就是最主要的是两个 system pallet。一个 support一个 system。

```rust
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// [...]
```

config 其实就是可以来参数化的东西，比如说像 event ，在 web 界面上，或者是程序的后端能够以统一的方式看到 **event** 的定义和数据。

```rust
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}
```

如果你的 pallet 要存储一些数据在链上的话，那首先的话要定义 storage。这是整个 storage 的定义，但是对应某一个 storage 一个子项目，比如说这里的 something，它里面定义的是一个 u32 的值，然后在后面代码就可以看到怎么样去update 或者是取得这个 something。

```rust
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;
```

**event** 也是定义的比较简单，如果 something 从外面 update的，或者是存到的链上，它就会抛出这么一个事件。

```rust
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}
```

然后还有一些程序在运行过程中一些 error 的信息。

```rust
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}
```

最后再看一下，用户从外界可以调用的一些接口，一般来说就是通过 **transaction** 的形式来调用它。这个就是dosomething。这个就是在外面会传进一个新的值。这个值，他会去取代以前的那个值。首先来验证这个签名，然后就是从  **transaction**  里面取出 **account** 。之后它主要作用就是在 event 里面把这个 account 给抛出去，这样的话你就知道。从这个 event 就知道是谁调用了 **transaction**  。

```rust
#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
        // [...]
```

