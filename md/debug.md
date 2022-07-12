# 调试

今天我们来讲讲在开发 pallet 的时候如何调试。在 pallet 开发时主要有以下几种调试方式：

- logging uilities;

- printable trait;

- print 函数;

- if_std.

下面我们来一一演示。

## 准备

首先我们进入到 substrate-node-template/pallets 目录下，拷贝 template 然后命名为 Debug，然后修改包名为 pallet-debug。然后修改 runtime 中的内容将 pallet-debug 加载到 runtime 中。对于这几步不会的可以看我们前面的讲解，基本上我们每新加一个 pallet 都会使用这步。

接下来就是在 pallet-debug 中进行修改。

## 使用 logging uilities

这种方式就是使用 log 包进行打印，需要在 pallet-debug 的 Cargo.toml 中添加依赖如下：

```

[dependencies]
...
log = { version = "0.4.14", default-features = false }
...

[features]
default = ["std"]
std = [
  ...
  "log/std",
  "sp-runtime/std",
  "sp-std/std",
]
```

然后我们可以在 lib.rs 的代码中使用 log 进行打印，如下：

```

pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
      let who = ensure_signed(origin)?;
      <Something<T>>::put(something);
      log::info!("|||||||||||||||||||||| called by {:?}", who);

      Self::deposit_event(Event::SomethingStored(something, who));
      Ok(())
    }
```

## 使用 printable trait

此种方式我们需要为需要打印的类型实现 printable trait，在我们的示例中我们主要为 Error 类型实现对应的 trait，然后再进行打印，需要修改代码如下：

```

use sp_runtime::traits::Printable;
use sp_runtime::print;
...

  #[pallet::error]
  pub enum Error<T> {
    NoneValue,
    StorageOverflow,
  }

  impl<T: Config> Printable for Error<T> {
        fn print(&self) {
            match self {
                Error::NoneValue => "Invalid Value".print(),
                Error::StorageOverflow => "++++++++++++++++++++++++++ Value Exceeded and Overflowed".print(),
                _ => "Invalid Error Case".print(),
            }
        }
    }

   ...

    #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
    pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
      log::info!("|||||||||||||||||||||| cause error");
      let _who = ensure_signed(origin)?;
      match <Something<T>>::get() {
        None => {
          //下面一行打印对应的错误
          print(Error::<T>::NoneValue);
          Err(Error::<T>::NoneValue)?
        },
        Some(old) => {
          log::info!("|||||||||||||||||||||| 2 error");
          let new = old.checked_add(1).ok_or({
            //下面一行打印对应的错误
            print(Error::<T>::StorageOverflow);
            Error::<T>::StorageOverflow
          })?;
          <Something<T>>::put(new);
          Ok(())
        },
      }
    }
```

## 使用 print 函数

此处直接使用 print 进行打印，不过使用前也需要引入 use sp_runtime::print;

打印的示例代码如下：

```

#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
    pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
      let who = ensure_signed(origin)?;
      <Something<T>>::put(something);
      //示例代码
      print("After storing my_val");
      Self::deposit_event(Event::SomethingStored(something, who));
      Ok(())
    }
```

## 使用 if_std

此种方式我本地没有实验成功，有兴趣的小伙伴可以研究研究。示例代码如下：

```

use sp_std::if_std;
  ...

  #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
  pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
    let who = ensure_signed(origin)?;
    <Something<T>>::put(something);

    if_std! {
                  println!("Hello native world!");
                        println!("My value is: {:#?}", something);
                  println!("The caller account is: {:#?}", who);
              }

    Self::deposit_event(Event::SomethingStored(something, who));
    Ok(())
  }
```
