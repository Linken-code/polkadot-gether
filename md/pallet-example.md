# pallet 结构

一个 pallet 的功能主要包括 7 部分，分别是：

1. 依赖;
2. pallet 类型声明;
3. config trait;
4. 定义要使用的链上存储;
5. 事件;
6. 钩子函数;
7. 交易调用函数;

其中 1 和 2 基本上是固定的写法，而对于后面的 3-7 部分，则是根据实际需要写或者不写.下面是一个完整的 pallet 实例：

```
// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // 2. Declaration of the Pallet type
    // This is a placeholder to implement traits and methods.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 3. Runtime Configuration Trait
    // All types and constants go here.
    // Use #[pallet::constant] and #[pallet::extra_constants]
    // to pass in values to metadata.
    #[pallet::config]
    pub trait Config: frame_system::Config { ... }

    // 4. Runtime Storage
    // Use to declare storage items.
    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub MyStorage<T: Config> = StorageValue<_, u32>;

    // 5. Runtime Events
    // Can stringify event types to metadata.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> { ... }

    // 6. Hooks
    // Define some logic that should be executed
    // regularly in some context, for e.g. on_initialize.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

    // 7. Extrinsics
    // Functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T:Config> Pallet<T> { ... }

}
```

# pallet 结构分析

下面开始对 pallet 的各部分进行详细分析：

## 导出和依赖

```
// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    ...
}
```

学过 Rust 的都知道，Pub mod pallet{}就是将我们的 pallet 暴露出来，可以让外部使用（如果这部分不能理解的，建议参考文档）.

接下来我们说依赖，第一行 pub use pallet::\_;是可以使用 pallet 中的所有类型，函数，数据等。
还有

```
use frame_support::pallet_prelude::_;
use frame_system::pallet_prelude::\*;
```

这两行，引入了相关的依赖。我们在写自己的 pallet 时候，当 pallet 使用到什么依赖，可以在这里引入。

## Pallet 中的类型声明

```
// 2. Declaration of the Pallet type
    // This is a placeholder to implement traits and methods.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
```

接下来是 Pallet 类型声明，它是一系列 trait 和方法的拥有者，实际的作用类似于占位符。如果对这个还不理解的话，我们可以看如下 Rust 程序的例子：

```
trait MyTrait {
    fn info(&self);
}

struct PlaceHolder(); //本身并没有字段

impl PlaceHolder {
    fn method(&self) {
        println!("This is method.");
    }
}

impl MyTrait for PlaceHolder {
    fn info(&self) {
        println!("This is info method.");
    }
}

fn main() {
    let p = PlaceHolder();
    p.method();
    p.info();
}
```

所以在这部分中定义的 pub struct Pallet<T>(\_)就和上面的 Rust 例子中定义的 struct PlaceHolder();作用一样，是 method 和 info 方法的主体。

## Runtime 配置 trait

这部分是指定 Runtime 的配置 trait，Pallet 中使用的一些类型和常量在此 trait 中进行配置。通常的使用方式如下：

```
#[pallet::config]
    pub trait Config: frame_system::Config {
        type Id: Member
      + Parameter
      + AtLeast32BitUnsigned
      + Codec
      + Copy
      + Debug
      + Default
      + MaybeSerializeDeserialize;

        #[pallet::constant]
    type Limit: Get<u32>;
    }

```

例如我们这里定义了一个类型 Id 以及一个常量 Limit，定义的格式就是 type 类型名/常量名: trait 约束 ，不同的是常量名字上面会加上#[pallet::constant]。此处定义的类型以及常量，会在 runtime 中（就是代码 runtime/src/lib.rs 中）使用时，会指定具体的类型。

对于 Config 中的类型，我们可以在我们整个 Pallet 中使用，使用的方式就是 T::类型名/常量名。例如此处定义的 Id，我们使用时就是 T::Id。

## 存储

存储（Storage）允许我们在链上存储数据，使用它存储的数据可以通过 Runtime 进行访问。substrate 提供了四种存储方式，分别是：

- Storage Value

- Storage Map

- Storage Double Map

- Storage N Map

从字面意思，我们基本上也可以看出几种存储的区别。

1、 StorageValue 是存储单个的值

2、StorageMap 是以 map 的方式存储（key-value）

3、StorageDoubleMap 则是以双键的方式存储（就是两个 key 对应 value 的方式）

4、StorageNMap 则是 N 个 key 的方式

关于存储的介绍可以参考官方文档（https://docs.substrate.io/v3/runtime/storage/）

定义存储通常的方式如下：

```
#[pallet::storage]
pub type MyStorageValue<T: Config> =
        StorageValue<_, u32, ValueQuery>;

#[pallet::storage]
pub type MyStorageMap<T: Config> =
    StorageMap<_, Twox64Concat, u32, u32, ValueQuery>;
```

首先是需要添加#[pallet::storage]宏，然后使用 pub type 存储名 = 某种存储类型<...>。至于尖括号里面具体填的东西，可以看 Storage 的 Rust 文档。

如 StorageMap 就可以参考这里（https://docs.substrate.io/rustdocs/latest/frame_support/storage/types/struct.StorageMap.html）。

## 事件

当 pallet 需要把运行时上的更改或变化通知给外部主体时，就需要用到事件。事件是一个枚举类型，如下：

```
#[pallet::event]
#[pallet::metadata(u32 = "Metadata")]
pub enum Event<T: Config> {
    /// Set a value.
    ValueSet(u32, T::AccountId),
}
```

在区块链写交易函数的时候，一般分为三步，分别是判断条件、修改状态、发出事件。例如我们上一节定义了 pub enum Event<T: Config> {ClaimCreated(u32, u128) }事件，那么交易函数中就可以使用 Self::deposit_event(Event::ClaimCreated(id, claim));发出事件。

## 钩子函数

钩子函数，是在区块链运行过程中希望固定执行的函数，例如我们希望在每个区块构建之前、之后的时候执行某些逻辑等，就可以把这些逻辑放在钩子函数中。钩子函数一共有：

```
pub trait Hooks<BlockNumber> {
    fn on_finalize(_n: BlockNumber) { ... }
    fn on_idle(_n: BlockNumber, _remaining_weight: Weight)
       -> Weight { ... }
    fn on_initialize(_n: BlockNumber) -> Weight { ... }
    fn on_runtime_upgrade() -> Weight { ... }
    fn pre_upgrade() -> Result<(), &'static str> { ... }
    fn post_upgrade() -> Result<(), &'static str> { ... }
    fn offchain_worker(_n: BlockNumber) { ... }
    fn integrity_test() { ... }
}
```

从函数名字上，我们也基本上可以判断出这些钩子函数什么时候执行。

- on_finalize 是在区块 finalize 的时候执行
- on_idle 是在 on_finalize 之前执行
- on_initialize 是在准备打包区块之前执行
- on_runtime_upgrade 是升级的时候执行
- pre_upgrade 是在升级之前执行
- post_upgrade 是在升级之后执行
- offchain_worker 在每个区块同步的时候执行

## 交易

Extrinsic 则是可以从 runtime 外部可以调用的函数，也是 pallet 对外提供的逻辑功能。交易是放在如下部分：

```
 #[pallet::call]
    impl<T:Config> Pallet<T> { 各种交易函数 }
```

# 如何编写一个 pallet Examples

## 修改 config

```
  #[pallet::config]
  pub trait Config: frame_system::Config {
        type Event: From<Event<Self>>
        + IsType<<Self as frame_system::Config>::Event>;
  }
```

这里其实就是定义了一个关联类型，这个关联类型需要满足后面的类型约束（From<Event> + IsType<::Event>）。至于为什么是这样的约束，我们其实可以从字面意思进行理解，一个是可以转换成 Event，另外一个就是它是 frame_system::Config 的 Event 类型。对于大部分 pallet 来说, 如果需要使用到 Event，那么都需要在这个 Config 中进行定义，定义的方式基本一样.

## 修改存储

```
 #[pallet::storage]
    pub type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, u128>;
```

这部分就是在链上定义了一个存储，是一个 key-value 方式的存储结构，用于存储我们后面要使用的存证，key 是 u32 格式，value 也是 u128 格式。

## 添加事件

```
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(u32, u128),
    }
```

这里的 Event 是用来在我们具体的函数中做完动作之后发出的，一般用来通知前端做一些处理。这里我们在 Event 中定义了一个事件，就是创建存证

## 添加交易处理函数

```
 #[pallet::call]
    impl<T:Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>,
          id: u32,
          claim: u128) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            Proofs::<T>::insert(
                &id,
                &claim,
            );

            Self::deposit_event(Event::ClaimCreated(id, claim));
            Ok(().into())
        }
    }
```

# 将 pallet 添加到 runtime 中

如果用开发一个程序来类别的话，上面写完我们的 pallet 就类似于我们开发好了一个库（或者说模块），但是这个库还没有真正的用在我们的程序中（链）。接下来就是要在链上使用，就要将 pallet 添加到 runtime 中。添加的过程也比较简单，这里我们分两步进行，分别是修改 Cargo.toml 中和 runtime/src/lib.rs 中。

## 修改 Cargo.toml

要在 runtime 中使用我们上面编写的 pallet，需要修改 substrate-node-template/runtime/Cargo.toml，在其中添加依赖如下：

```
...
[dependencies]
...
pallet-simple-pallet = {
  version = "4.0.0-dev",
  default-features = false,
  path = "../pallets/simple-pallet" } #我们上面编写的pallet
...

[features]
default = ["std"]
std = [
  ...
  "pallet-template/std",
  "pallet-simple-pallet/std", #我们上面编写的pallet
  ...
]
```

## 修改 runtime/src/lib.rs

在 runtime/src/lib.rs 中来使用 pallet。首先我们需要添加 pallet 的配置，其实就是指定 pallet 中 Config 中的关联类型，所以在 substrate-node-template/runtime/src/lib.rs 中添加如下代码：

```
impl pallet_simple_pallet::Config for Runtime {

  type Event = Event;  //我们上面的定义中只有一个关联类型Event，
                      //在此处进行指定，等号右边的Event实际上是
                      //frame system中的Event，此处不需要深究，
                    //可以理解为在runtime中已经定义好的一种具体的类型。
}
```

接下来就是把 simple_pallet 加入到 runtime 中，修改如下代码：

```
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
    TemplateModule: pallet_template,
    //添加下面这一行，这里可以看出，
    //实际上我们前面实现的simple-pallet可以理解为一种类型，
    //然后这里在runtime中定义了一个变量，
    //该变量是这个pallet_simple_pallet类型
    SimplePallet: pallet_simple_pallet,
  }
);
```

至此，我们就将 pallet 加入到我们的 runtime 中了。
