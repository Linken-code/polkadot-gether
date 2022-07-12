# 在 pallet 中使用其它 pallet

本节，我们讲讲如何在自己的 pallet 中使用其它的 pallet。在自己的 pallet 中使用其它的 pallet 主要有以下几种情况：

- 在 pallet 的 config 中定义类型，然后 runtime 中使用时指定这个类型为 frame 中指定某个现成的 pallet；

- 在 pallet 的 config 中定义类型，然后 runtime 中使用时指定这个类型为 frame 中指定某个自定义的 pallet；

- 封装和扩展现有的 pallet。

本节主要介绍前两种方式，实际上第一种和第二种是同样的方式，但是我们这里分成两种情况介绍。

## 在 runtime 中直接指定某个类型为其它的 pallet

在上一节中我们讲解了 pallet 的 config 中定义类型，然后在 runtime 中指定具体的类型。此处讲的第一种使用其它 pallet 就是这种方式。这种方式比较常见的就是在 pallet 中定义 currency 类型，然后用指定 currency 类型为 balances pallet。详细的可以看 substrate 中 node 中的使用，在 pallet_assets 中使用了 pallet_balances，就是通过指定前者的 currency 类型为后者（详情见：https://github.com/paritytech/substrate/blob/master/bin/node/runtime/src/lib.rs#L1343）。

## pallet 中使用其它 pallet 的 storage

下面我们在自定义两个 pallet，分别叫做 pallet-use-other-pallet1 和 pallet-storage-provider，然后我们在前一个 pallet 中读取和存储后一个 pallet，下面我们看具体实现。整个部分完整的代码可以参考这里(本节对应的 pallet 是 storage-provider 和 use-other-pallet1)。

### pallet-storage-provider 实现

学过之前的知识我们知道，pallet 的 config 中的类型定义实际上一种 trait 约束，就是对应的类型需要实现冒号后的 trait。为了方便演示，我们定义如下 trait：

```
pub trait StorageInterface{
  type Value;
  fn get_param() -> Self::Value;
  fn set_param(v: Self::Value);
}
```

这个 trait 可以单独定义在某个包中。这里我们为了方便，直接放在 pallet-storage-provider 对应的文件夹中。下面我们再看看 pallet-storage-provider/src/lib.rs 的代码，如下：

```

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use traits::StorageInterface;

pub mod traits;

#[frame_support::pallet]
pub mod pallet {
  use codec::Codec;
  use frame_support::{
    pallet_prelude::*,
    sp_runtime::traits::AtLeast32BitUnsigned,
    sp_std::fmt::Debug,
  };
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>>
      + IsType<<Self as frame_system::Config>::Event>;
    type Value: Member
      + Parameter
      + AtLeast32BitUnsigned
      + Codec
      + From<u32>
      + Into<u32>
      + Copy
      + Debug
      + Default
      + MaxEncodedLen
      + MaybeSerializeDeserialize;
  }

  #[pallet::storage]
  pub type MyValue<T: Config> =
       StorageValue<_, T::Value, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    FunctionCall,
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(0)]
    pub fn my_function(
      origin: OriginFor<T>,
    ) -> DispatchResultWithPostInfo {
      ensure_signed(origin)?;
      log::info!(target: "storage provider", "my function!");
      Self::deposit_event(Event::FunctionCall);

      Ok(().into())
    }
  }
}

// 注意此处：我们为pallet实现了前面定义的trait StorageInterface.
impl<T: Config> StorageInterface for Pallet<T> {
  type Value = T::Value;

  fn get_param() -> Self::Value {
    MyValue::<T>::get()
  }

  fn set_param(v: Self::Value) {
    MyValue::<T>::put(v);
  }
}
```

## pallet-use-other-pallet1

下面我们再看 pallet-use-other-pallet1 的代码：

```

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use codec::Codec;
  use frame_support::{
    pallet_prelude::*,
    sp_runtime::traits::AtLeast32BitUnsigned,
    sp_std::fmt::Debug,
  };
  use frame_system::pallet_prelude::*;
  use pallet_storage_provider::traits::StorageInterface;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  // 3. Runtime Configuration Trait
  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>>
      + IsType<<Self as frame_system::Config>::Event>;
    type Value: Member
      + Parameter
      + AtLeast32BitUnsigned
      + Codec
      + From<u32>
      + Into<u32>
      + Copy
      + Debug
      + Default
      + MaxEncodedLen
      + MaybeSerializeDeserialize;

  //定义了MyStorage类型，要求其实现trait StorageInterface
    type MyStorage: StorageInterface<Value = Self::Value>;
  }

  // 5. Runtime Events
  // Can stringify event types to metadata.
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    StoreEvent,
  }

  // 7. Extrinsics
  // Functions that are callable from outside the runtime.
  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(0)]
    pub fn storage_value(
      origin: OriginFor<T>,
      value: T::Value,
    ) -> DispatchResultWithPostInfo {
      ensure_signed(origin)?;

      T::MyStorage::set_param(value);

   //使用trait StorageInterface中的函数
      let v = T::MyStorage::get_param();
      log::info!(target: "other-pallet",
           "Value get from storage is: {:?}", v);

      Self::deposit_event(Event::StoreEvent);

      Ok(().into())
    }
  }
}
```

## 在 runtime 中添加两个 pallet

下面就是在 runtime 中添加对应的 pallet。

首先当然添加依赖，在 runtime/Cargo.toml 中添加：

```

[dependencies]
...
pallet-storage-provider ={
      version = "4.0.0-dev",
      default-features = false,
      path = "../pallets/storage-provider" }
pallet-use-other-pallet1 = {
      version = "4.0.0-dev",
      default-features = false,
      path = "../pallets/use-other-pallet1" }
 ...

[features]
default = ["std"]
std = [
  "codec/std",
  "scale-info/std",
  "frame-executive/std",
  "frame-support/std",
  "frame-system-rpc-runtime-api/std",
  "frame-system/std",
  "pallet-aura/std",
  "pallet-balances/std",
  "pallet-grandpa/std",
  "pallet-randomness-collective-flip/std",
  "pallet-sudo/std",
  "pallet-template/std",
  "pallet-simple-pallet/std",
  "pallet-use-storage/std",
  "pallet-use-errors/std",
  "pallet-ext-example/std",
  "pallet-use-hooks/std",
  "pallet-use-rpc/std",
  "pallet-use-config1/std",
  "pallet-use-config2/std",
  "pallet-storage-provider/std",
  "pallet-use-other-pallet1/std",
...
  ]
```

然后在 runtime/src/lib.rs 中添加如下：

```
//添加下面4行
impl pallet_storage_provider::Config for Runtime {
  type Event = Event;
  type Value = u32;
}

//添加下面5行
impl pallet_use_other_pallet1::Config for Runtime {
  type Event = Event;
  type Value = u32;
  type MyStorage = StorageProvider;
}

// Create the runtime by composing the FRAME pallets
// that were previously configured.
construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    System: frame_system,
  ...
  //添加下面两行
    StorageProvider: pallet_storage_provider,
    UseOtherPallet1: pallet_use_other_pallet1,
  }
);
```
