# 调度函数

调度函数在 substrate 官方文档里面叫做 Extrinsics（外部调用），详细的 Extrinsics 介绍可以参考这里.在 substrate 中共有三种 Extrinsics，分别是 Inherents、Signed transactions 和 Unsigned transactions。而在我们开发 pallet 的过程中，比较常用到的是后两种，所以我们这里也主要介绍后两种，对于 Inherents 有兴趣的小伙伴可以自己看官方文档研究下。

## Signed transactions

签名交易包含发起该交易的账户的签名，并会支付交易费用。因为签名交易的合法性可以在执行之前识别，所以它们可以在节点之间的网络上传播，属于垃圾信息的风险比较小。签名交易和比特币、以太坊中的交易的概念类似。

## Unsigned transactions

但是有时候我们希望使用不需要花费交易费的交易。例如一个节点给链发送在线的心跳，表示自己在线，这种情况肯定是不希望花费交易费的，此时我们就可以使用 unsigned transaction。但是需要注意的是，由于交易没有签名，因此也没有人支付费用，所以缺乏防止垃圾信息的经济逻辑。所以我们在使用未签名交易时需要特别小心。

# 通常的写法

## 调度函数的位置

所有调度函数都放置在#[pallet::call]标注的代码段内，每个调度函数上面需要标注其调用的权重，并根据需要是否添加#[transactional] 如下：

```

#[pallet::call]
impl<T: Config> Pallet<T> {

    #[pallet::weight(10_000)]  //具体的函数使用对应的权重计算函数
    pub fn function1(origin: OriginFor<T>, param: u32, ...) -> DispatchResult {}

    #[transactional]
    #[pallet::weight(10_000)]  //具体的函数换成对应的权重计算函数
    pub fn function2(origin: OriginFor<T>, param: u32, ...) -> DispatchResult {}

    //其它调度函数
}
```

## 函数体的写法

在 pallet 的开发中，不管是签名的交易还是非签名的交易，通常都遵循以下三个步骤进行，分别是：

```
1、判断调用者是否有权限;
2、执行逻辑;
3、发出事件。
```

示例如下：

```
#[pallet::weight(10_000)]
pub fn set_number_bigger_than_100(origin: OriginFor<T>, number: u32) -> DispatchResult {
    ensure_signed(origin)?;  //1、判断是否是合法调用

    // 2、函数的具体逻辑
    ...

    //3、成功后需要发出事件
    Self::deposit_event(Event::xx事件);

    Ok(()) //固定写法
}
```

这里需要特别说明的是第一步，分三种情况：

```
1、如果是签名调用的则判断的写法是 ensure_signed(origin)?。
   可以回忆回忆之前的例子（https://github.com/anonymousGiga/learn-substrate-easy-source/blob/main/substrate-node-template/pallets/use-storage/src/lib.rs#L78）
2、如果是需要root用户调用的则判断的写法是ensure_root(origin)?。
   可以回忆回忆之前的例子（https://github.com/anonymousGiga/learn-substrate-easy-source/blob/main/substrate-node-template/pallets/use-storage/src/lib.rs#L64）
3、如果非签名的函数调用的判断写法则是ensure_none(origin)?。
   这里暂时没有例子，后续我们会使用到。
```

## 权重

从上面的示例中可以看到，每个调度函数上面都需要标注其权重，在具体开发时，我们一般是先写一个固定的权重，然后在所以功能开发完成后，再写对应的 benchmarking（在本教程的最后位置会来学习）。关于权重的详细介绍可以参考官方文档。

### transactional

当我们去看不管是我们上面的例子还是 substrate 已经实现好的 pallet 的调度函数，会发现有些调度函数上面会有#[transactional]，有些则没有。这个属性宏实际上是保证调度函数执行的一致性。更具体点说就是当在函数中遇到错误后，会回滚状态，保证错误发生之前写入的状态回滚。解释起来比较敖口，看下面的例子可以很好的理解。

```

#[transactional]
#[pallet::weight(0)]
pub fn set_param_bigger_than_100(origin: OriginFor<T>, param: u32)
   -> DispatchResult {
    //1、判断调用者权限
    ensure_signed(origin)?;

    //2、开始业务逻辑
    //2.1、将标志位设置为true
    SetFlag::<T>::put(true);

    //2.2、如果参数大于100,则写入到storage praram中
    if param <= 100u32 {
  return Err(Error::<T>::ParamInvalid.into())
    }
    Param::<T>::put(param);

    //3、发出事件
    Self::deposit_event(Event::SetParam(param));
    Ok(().into())
}
```

上面例子中的逻辑很简单，就是设置一个大于 100 的参数到 storage Param 中，还有一个 Flag 标识表示是否已经设置了参数。我们很容易发现一个问题，如果我们在 2.2 中判断输入的参数小与等于 100,则会发生错误直接返回，但是我们前面已经在 2.1 设置了 flag 了，这样逻辑上不就错误了吗？对于这个问题，我们有两种方式解决。

- 第一种就是我们这里介绍的，在函数的上面加上#[transactional]属性宏。加上此宏后，表示该函数是原子执行的，函数中的任何位置错误返回都会导致函数中设置的所有状态回滚。像上面的例子中，加上#[transactional]后，如果 2.2 的位置错误返回，也会让 2.1 位置设置的值无效。

- 第二种方式就是靠程序员解决，就是调整代码的顺序，遵循先判断，再修改状态的规则实现。如上面的例子可以修改为如下，就不需要加 #[transactional]:

```
//该实现不需要加#[transactional]
#[pallet::weight(0)]
pub fn set_param_bigger_than_100(origin: OriginFor<T>, param: u32)
   -> DispatchResult {
    //1、判断调用者权限
    ensure_signed(origin)?;

    //2、开始业务逻辑
    //2.2、如果参数大于100,则写入到storage praram中
    if param <= 100u32 {
  return Err(Error::<T>::ParamInvalid.into())
    }
    Param::<T>::put(param);

    //2.1、将标志位设置为true，将此步放在判断后面，保证一定能成功
    SetFlag::<T>::put(true);

    //3、发出事件
    Self::deposit_event(Event::SetParam(param));
    Ok(().into())
}
```

但是在有些复杂实现的时候，往往很难保证代码实现遵循先判断再修改的原则，此时就需要给函数加上#[transactional]。

不过需要注意的是，使用 transactional 需要通过 use frame_support::transactional;引入。

# 示例

```

#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use frame_support::transactional;

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
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>>
         + IsType<<Self as frame_system::Config>::Event>;
  }

  // 4. Runtime Storage
  #[pallet::storage]
  #[pallet::getter(fn my_param)]
  pub type Param<T: Config> = StorageValue<_, u32, ValueQuery>;

  #[pallet::storage]
  pub type SetFlag<T: Config> = StorageValue<_, bool, ValueQuery>;

  // 5. Runtime Events
  // Can stringify event types to metadata.
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    SetParam(u32),
  }

  // 8. Runtime Errors
  #[pallet::error]
  pub enum Error<T> {
    // 参数必须大于100
        ParamInvalid,
    }

  // 7. Extrinsics
  // Functions that are callable from outside the runtime.
  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[transactional]
    #[pallet::weight(0)]
    pub fn set_param_bigger_than_100(
        origin: OriginFor<T>,
         param: u32) -> DispatchResult {
      //1、判断调用者权限
      ensure_signed(origin)?;

      //2、开始业务逻辑
      //2.1、将标志位设置为true
      SetFlag::<T>::put(true);

      //2.2、如果参数大于100,则写入到storage praram中
      if param <= 100u32 {
        return Err(Error::<T>::ParamInvalid.into())
      }
      Param::<T>::put(param);

      //3、发出事件
      Self::deposit_event(Event::SetParam(param));

      Ok(().into())
    }
  }
}
```
