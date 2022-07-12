# pallet 中的 Config

## pallet 简单示例

在正式开始前，我们先来写一个简单的 pallet，名字叫做 use-config1，其 pallet 中的 src/lib.rs 中的代码如下：

```
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  // 3. Runtime Configuration Trait
  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>>
         + IsType<<Self as frame_system::Config>::Event>;
  }

  // 4. Runtime Storage
  // 用storageMap存储学生信息，（key， value）分别对应的是学号和姓名.
  #[pallet::storage]
  #[pallet::getter(fn students_info)]
  pub type StudentsInfo<T: Config> =
      StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

  // 5. Runtime Events
  // Can stringify event types to metadata.
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    SetStudentInfo(u32, u128),
  }

  // 8. Runtime Errors
  #[pallet::error]
  pub enum Error<T> {
    // 相同学号的只允许设置一次名字
    SetStudentsInfoDuplicate,
  }

  // 7. Extrinsics
  // Functions that are callable from outside the runtime.
  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(0)]
    pub fn set_student_info(
      origin: OriginFor<T>,
      student_number: u32,
      student_name: u128,
    ) -> DispatchResultWithPostInfo {
      ensure_signed(origin)?;

      if StudentsInfo::<T>::contains_key(student_number) {
        return Err(Error::<T>::SetStudentsInfoDuplicate.into())
      }

      StudentsInfo::<T>::insert(&student_number, &student_name);
      Self::deposit_event(Event::SetStudentInfo(
            student_number, student_name));

      Ok(().into())
    }
  }
}
```

在上面的 pallet 中，我们就实现了一个功能，就是把学生的信息（学号和姓名）存储在链上。我们这里使用 StorageMap 来进行存储：

```
pub type StudentsInfo<T: Config> =
    StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;
```

这里我们定义了学号的格式是 u32 类型，学生的姓名是 u128 类型。但是其实在实际的应用中，我们学号的类型是 u8 类型，也可能是 u16 类型，还甚至是 u64 类型。同样学生的姓名也可能是其它类型。所以其实这里比较好的写法是下面这样：

```

pub type StudentsInfo<T: Config> =
    StorageMap<_,
         Blake2_128Concat,
         StudentNumberType,
         StudentNameType,
         ValueQuery>;
```

然后里面的 StudentNumberType, StudentNameType 可以在我们真正使用的时候指定。

## 在 Config 中定义配置类型

为了便于对比，我们拷贝上面的 use-config1 的代码，创建新的 pallet 名字叫做 use-config2，use-config2/src/lib.rs 中的代码如下：

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

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  // 3. Runtime Configuration Trait
  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>>
    + IsType<<Self as frame_system::Config>::Event>;

    //（1）声明了StudentNumberType和StudentNameType
    //声明StudentNumber类型
    type StudentNumberType: Member
      + Parameter
      + AtLeast32BitUnsigned
      + Codec
      + Copy
      + Debug
      + Default
      + MaxEncodedLen
      + MaybeSerializeDeserialize;

    //声明StudentName类型
    type StudentNameType: Parameter
      + Member
      + AtLeast32BitUnsigned
      + Codec
      + Default
      + From<u128>
      + Into<u128>
      + Copy
      + MaxEncodedLen
      + MaybeSerializeDeserialize
      + Debug;
  }

  // 4. Runtime Storage
  // 用storageMap存储学生信息，（key， value）分别对应的是学号和姓名.
  // （2）使用了T::StudentNumberType和T::StudentNameType
  // 替换了原来的u32和u128
  #[pallet::storage]
  #[pallet::getter(fn students_info)]
  pub type StudentsInfo<T: Config> =
    StorageMap<_,
       Blake2_128Concat,
       T::StudentNumberType,
       T::StudentNameType,
       ValueQuery>;

  // 5. Runtime Events
  // Can stringify event types to metadata.
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    // （3）使用了T::StudentNumberType和
    //  T::StudentNameType替换了原来的u32和u128
    SetStudentInfo(T::StudentNumberType, T::StudentNameType),
  }

  // 8. Runtime Errors
  #[pallet::error]
  pub enum Error<T> {
    // 相同学号的只允许设置一次名字
    SetStudentsInfoDuplicate,
  }

  // 7. Extrinsics
  // Functions that are callable from outside the runtime.
  #[pallet::call]
  impl<T: Config> Pallet<T> {
     // （4）使用了T::StudentNumberType和T::StudentNameType
     // 替换了原来的u32和u128
    #[pallet::weight(0)]
    pub fn set_student_info(
      origin: OriginFor<T>,
      student_number: T::StudentNumberType,
      student_name: T::StudentNameType,
    ) -> DispatchResultWithPostInfo {
      ensure_signed(origin)?;

      if StudentsInfo::<T>::contains_key(student_number) {
        return Err(Error::<T>::SetStudentsInfoDuplicate.into())
      }

      StudentsInfo::<T>::insert(&student_number, &student_name);
      Self::deposit_event(Event::SetStudentInfo(
            student_number, student_name));

      Ok(().into())
    }
  }
}
```

从上面的代码可以看出主要有 4 处修改，分别用（1）-（4）标注：
（1）在 Config 中声明了 StudentNumberType 和 StudentNameType，两种类型的冒号后面是对应的 trait 约束。

（2）在 StorageMap 中不再使用原来的 u32 和 u128 类型，而是使用关联类型 T::StudentNumberType 和 T::StudentNameType。

（3）在 Event 中不再使用原来的 u32 和 u128 类型，而是使用关联类型 T::StudentNumberType 和 T::StudentNameType。

（4）在函数的输入参数中不再使用原来的 u32 和 u128 类型，而是使用关联类型 T::StudentNumberType 和 T::StudentNameType。

至此，我们基本上把 pallet 中的类型换成了关联类型。但是这个关联类型具体的类型在哪里指定呢？下面我们就来揭晓。

## 在 runtime 中指定具体的类型

学过前面的知识我们知道，需要在 runtime/src/lib.rs 将 pallet 加载进来才能真正的使用 pallet。下面我们就把 use-config2 加到 runtime 中。首先在 substrate-node-template/runtime/Cargo.toml 添加依赖如下：

```

[dependencies]
...
pallet-use-config2 = {
    version = "4.0.0-dev",
    default-features = false,
    path = "../pallets/use-config2" }
...
[features]
default = ["std"]
std = [
  "codec/std",
  ...
  "pallet-use-config2/std",
  ...
      ]
```

接下来在 substrate-node-template/runtime/src/lib.rs 中添加如下代码：

```
//添加下面的5行
impl pallet_use_config2::Config for Runtime {
  type Event = Event;
  type StudentNumberType = u32;   //指定具体的类型
  type StudentNameType = u128;    //指定具体的类型
}

construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    System: frame_system,
                ...
    UseConfig2: pallet_use_config2, //添加此行
  }
);
```

从上面的代码可以看出，我们是在 impl pallet_use_config2::Config for Runtime 中为 StudentNumberType 和 StudentNameType 指定具体的类型。
