# offchain worker 中提交未签名交易

要在 pallet 中使用 ocw 提交未签名交易，我们需要修改几个地方：

## 修改 Config 配置

```

#[pallet::config]
pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
    ...
}
```

在 Config 需要继承 trait SendTransactionTypes<Call>才能在 ocw 提交未签名交易。

## 实现具体的未签名调度函数

具体代码如下：

```
 #[pallet::weight(0)]
    pub fn submit_something_unsigned(
      origin: OriginFor<T>,
      number: u64,
    ) -> DispatchResultWithPostInfo {
      ensure_none(origin)?;

      let mut cnt: u64 = 0;
      if number > 0 {
        cnt = number;
      }

      log::info!(target:"ocw", "unsigned +++++++++++++++++++ offchain_worker set storage: {:?}, cnt: {:?}", number, cnt);
      SomeInfo::<T>::insert(&number, cnt);

      Self::deposit_event(Event::UnsignedPutSetSomeInfo(number, cnt));

      Ok(().into())
    }
```

## 在 ocw 中调用未签名交易函数

具体代码如下：

```

#[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
      let number: u64 = block_number.try_into().unwrap_or(0);
      //下面为具体的调用未签名交易的方式
      let call = Call::submit_something_unsigned { number };
      if let Err(e) =
        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
          .map_err(|_| <Error<T>>::OffchainUnsignedTxError)
      {
        log::error!(target:"ocw", "offchain_worker submit unsigned tx error: {:?}", e);
      } else {
        log::info!(target:"ocw", "offchain_worker submit unsigned tx success");
      }
    }
  }
```

## 实现未签名交易验证的 trait

代码如下：

```

#[pallet::validate_unsigned]
  impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
       //Call冒号后面就是具体的提交未签名交易的函数，
       //需要对此交易进行验证
       if let Call::submit_something_unsigned { number: _ } = call {
        ValidTransaction::with_tag_prefix("OcwUnsigtx")
          .priority(TransactionPriority::max_value())
          .longevity(5)
          .propagate(false)
          .build()
      } else {
        InvalidTransaction::Call.into()
      }
    }
  }
```

具体的 ValidTransaction 使用可以参考文档https://paritytech.github.io/substrate/master/sp_runtime/transaction_validity/struct.ValidTransaction.html

当我们要在 ocw 中提交未签名交易时，上面的 1.1、1.3、1.4 都是差不多的写法，1.2 为具体的未签名交易函数，根据自己的业务修改就好。当然 1.4 中的 ValidTransaction 根据自己的情况进行修改。

## 在 runtime 中添加相关代码

接下来就是在 runtime 中添加代码，本节需要添加的代码比较简单，如下：

```
impl pallet_ocw_unsigtx::Config for Runtime {
  type Event = Event;
}

construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    System: frame_system,
    ...
    OcwUnSigtx: pallet_ocw_unsigtx,
  }
```
