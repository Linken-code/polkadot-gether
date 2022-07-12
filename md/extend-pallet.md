# 封装和扩展现有的 pallet

本节我们讲解在 pallet 中使用其它 palllet 的另外一种情况，即在新的 pallet 中封装和扩展现有的 pallet。我们这里 substrate 提供的 contracts pallet，然后对其中的功能进行封装。在我们的封装中，将 contracts pallet 的 call 函数封装成 sudo_call，即需要 root 权限才能调用。同时，我们在 runtime 中加载 contracts 时，去掉直接调用 contracts 函数的方式。

整个方式我们分成两大步骤，如下：

- 编写 extend-pallet;-

- 在 runtime 配置 extend-pallet 和 contracts pallet。

## 编写 extend-pallet

首先我们编写封装 contracts pallet 的 pallet，取名叫做 extend-pallet，主要代码如下：

```

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, HasCompact};
use frame_support::traits::Currency;
use scale_info::TypeInfo;
use sp_core::crypto::UncheckedFrom;
use sp_runtime::traits::StaticLookup;
use sp_std::{fmt::Debug, prelude::*};

type BalanceOf<T> =
  <<T as pallet_contracts::Config>::Currency as Currency<
  <T as frame_system::Config>::AccountId,>>::Balance;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  // 重点关注1
  #[pallet::config]
  pub trait Config: pallet_contracts::Config
      + frame_system::Config {}

  // 重点关注2
  #[pallet::call]
  impl<T: Config> Pallet<T>
  where
    T::AccountId: UncheckedFrom<T::Hash>,
    T::AccountId: AsRef<[u8]>,
    <BalanceOf<T> as HasCompact>::Type: Clone
     + Eq + PartialEq + Debug + TypeInfo + Encode,
  {
    #[pallet::weight(0)]
    pub fn sudo_call(
      origin: OriginFor<T>,
      dest: <T::Lookup as StaticLookup>::Source,
      #[pallet::compact] value: BalanceOf<T>,
      #[pallet::compact] gas_limit: Weight,
      storage_deposit_limit: Option<<BalanceOf<T>
           as codec::HasCompact>::Type>,
      data: Vec<u8>,
    ) -> DispatchResultWithPostInfo {
      //添加下面这行，用于判断是否是root权限
      ensure_root(origin.clone())?;

      //直接调用pallet-contracts的call函数
      pallet_contracts::Pallet::<T>::call(
        origin,
        dest,
        value,
        gas_limit,
        storage_deposit_limit,
        data,
      )
    }
  }
}
```

在上面的 pallet 中，主要有两个部分需要重点关注，一个是 Config 部分，封装 pallet 的 Config 需要集成被封装 pallet 的 Config，如下：

```
// 重点关注1
  #[pallet::config]
  pub trait Config: pallet_contracts::Config
  + frame_system::Config {}
```

另外一个是上面的“重点关注 2”部分，对主要 pallet-contracts 的 call 函数进行封装，在这部分里面，我们添加判断 root 权限的语句，然后直接调用 pallet-contracts 的 call 函数。

## 在 runtime 中配置 pallet

将上面的封装 pallet 准备好了之后，我们就需要将 extend-pallet 和 contracts pallet 加载到 runtime 中，需要修改 runtime/src/lib.rs 如下：

```

// 1、配置Contracts pallet
use pallet_contracts::weights::WeightInfo;
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

pub mod currency {
  use node_primitives::Balance;

  pub const MILLICENTS: Balance = 1_000_000_000;
  // assume this is worth about a cent.
  pub const CENTS: Balance = 1_000 * MILLICENTS;
  pub const DOLLARS: Balance = 100 * CENTS;

  pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
  }
}

parameter_types! {
  pub const DepositPerItem: Balance = currency::deposit(1, 0);
  pub const DepositPerByte: Balance = currency::deposit(0, 1);
  pub const MaxValueSize: u32 = 16 * 1024;
  pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
    BlockWeights::get().max_block;
  pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
      <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
      <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
    )) / 5) as u32;
  pub Schedule: pallet_contracts::Schedule<Runtime> = Default::default();
}

impl pallet_contracts::Config for Runtime {
  type Time = Timestamp;
  type Randomness = RandomnessCollectiveFlip;
  type Currency = Balances;
  type Event = Event;
  type Call = Call;
  type CallFilter = Nothing;
  type DepositPerItem = DepositPerItem;
  type DepositPerByte = DepositPerByte;
  type CallStack = [pallet_contracts::Frame<Self>; 31];
  type WeightPrice = pallet_transaction_payment::Pallet<Self>;
  type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
  type ChainExtension = ();
  type DeletionQueueDepth = DeletionQueueDepth;
  type DeletionWeightLimit = DeletionWeightLimit;
  type Schedule = Schedule;
  type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
}

// 2、配置extends pallet
impl pallet_extend_pallet::Config for Runtime {}

// 3、在runtime中定义两个pallet
construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    System: frame_system,
          ...
    // 注意下面两行的区别: 一定要去掉Call
    // Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
    Contracts: pallet_contracts::{Pallet, Storage, Event<T>},
    ExtendContracts: pallet_extend_pallet,
  }
);
```

对于上面代码中 1 和 2 部分，其实就是为 runtime 配置两个 pallet。我们需要重点说明的是第 3 部分，定义 Contracts 和 ExtendContracts 如下：

```

Contracts: pallet_contracts::{Pallet, Storage, Event<T>},
ExtendContracts: pallet_extend_pallet,
```

对于 Contracts，我们将它的 Call 部分去掉了，这也就表示我们在 runtime 层面没有对外暴露 Contracts 的调度函数接口，这样用户只能使用 ExtendContracts 提供的 sudo_call 函数，而不能使用 Contracts 的调度函数。
