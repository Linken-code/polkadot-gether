# storage 的使用方式

先看官方文档如何在 pallet 中定义 storage 的代码

```
#[pallet::storage]
2 #[pallet::getter(fn $getter_name)] // optional
3 $vis type $StorageName<$some_generic> $optional_where_clause
4 = $StorageType<$generic_name = $some_generics, $other_name = $some_other, ...>;
```

第一行，#[pallet::storage]是定义的 storage 时的固定写法，表示下面定义了一个 storage。在定义 storage 时，无论你怎么使用，都必须写这一行。

第二行，#[pallet::getter(fn $getter_name)]，在有些定义 storage 时会使用这一行，有些不会。这一行的意思是自动为下面定义的 storage 生成一个 getter 函数，函数的名字就是$getter_name。例如我定义如下的 storage：

```
 #[pallet::storage]
 #[pallet::getter(fn my_id)]
 pub type MyId<T: Config> = StorageValue<_, u8, ValueQuery>;
```

这里我定义了一个存储 MyId，就自动为其生成了 getter 函数，函数名字是 my_id，后续可以在 pallet 使用 my_id()函数来获取该 Storage 中存储的值

第三行和第四行就是真正定义 Storage。定义的格式一定是$vis type开头（其中$vis 是 public、或者无这些范围修饰符，也就是表示其在代码中的使用范围）。接下来的$StorageName就是存储的名字，然后紧接着的尖括号中的$some_generic 是泛型类型，而$optional_where_clause 是对应泛型类型的约束。所以，上面那个例子我们也可以定义成这样：

```
 #[pallet::storage]
 #[pallet::getter(fn my_id)]
 pub type MyId<T> where T: Config = StorageValue<_, u8, ValueQuery>;
```

这里我们使用了 ValueQuery，表示为查询值实现了 QueryKindTrait

第四行中的$StorageType则是具体的storage类型（也就是StorageValue\StorageMap\StorageDoubleMap\StorageNMap中的一种），接着的尖括号中的第一个参数$generic*name = $some_generics 主要用来生产 storage 的前缀（有兴趣的小伙伴可以深入研究下，可能和底层存储有关），在具体使用中一般都使用*即可,尖括号中从第二个参数起，就和具体的 Storage 类型相关，需要参见具体的 Storage 类型。

# 使用示例

下面我们就来用一个例子演示一下各种存储。我们假定有这样一个应用，记录某个年纪各个寝室每个床位的学生姓名，我们将分别使用 StorageValue\StorageMap\StorageDoubleMap 几种存储类型。在此例子中，我们将重新写一个 pallet，其过程和前面我们讲到的实现简单的 pallet 中的过程一样，后续我们的示例也都是这样的过程。本节我们还是复习一下创建 pallet 以及加载的过程，但是在后面的例子中，如非必要，我们将只写 pallet 部分的代码，在 runtime 中使用 pallet 我们将不重复赘述。

## 创建 pallet

我们还是在之前的 substrate-node-template 中进行。

- 拷贝 template，过程如下：

```
cd 上层路径/substrate-node-template/pallets
cp template use-storage -rf
```

修改 pallet 包名，打开 substrate-node-template/pallets/use-storage/Cargo.toml 文件，修改如下内容：

```

[package]
name = "pallet-use-storage"    #修改此行
...

```

- 添加模板

  接下来我们将 substrate-node-template/pallets/use-storage/src/lib.rs 中的内容完全删掉，然后拷贝模板到这个文件中。

## 编写 pallet 中的逻辑

然后我们在 pallet 中定义三个存储，分别用来存储班级、学生、寝室的信息，分别如下：

```
// 4. Runtime Storage
  // use storageValue store class.
  #[pallet::storage]
  #[pallet::getter(fn my_class)]
  pub type Class<T: Config> = StorageValue<_, u32>;

  // use storageMap store (student number -> student name).
  #[pallet::storage]
  #[pallet::getter(fn students_info)]
  pub type StudentsInfo<T: Config> =
     StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn dorm_info)]
  pub type DormInfo<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    u32, //dorm number
    Blake2_128Concat,
    u32, //bed number
    u32, // student number
    ValueQuery,
  >;
```

- Class 存储班级编号，需要 root 权限才能设置，使用 StorageValue 存储;

- StudentsInfo 存储学生的学号和姓名的对应关系，使用 StorageMap 存储;

- DormInfo 存储寝室号、床号、学号之间的对应关系，使用 StorageDoubleMap 存储。

另外我们定义了设置这些信息成功后对应的 Event 和可能的错误类型，分别如下：

```
#[pallet::error]
pub enum Error<T> {
    // Class 只允许设置一次
    SetClassDuplicate,
    // 相同学号的只允许设置一次名字
    SetStudentsInfoDuplicate,
    // 相同床位只允许设置一次
    SetDormInfoDuplicate,
}


// 5. Runtime Events
  // Can stringify event types to metadata.
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    SetClass(u32),
    SetStudentInfo(u32, u128),
    SetDormInfo(u32, u32, u32),
  }
```

所有的设置信息的交易函数实现如下：

```

#[pallet::weight(0)]
pub fn set_class_info(origin: OriginFor<T>, class: u32)
  -> DispatchResultWithPostInfo {
  ensure_root(origin)?;
  //使用Error类型
    if Class::<T>::exists() {
        return Err(Error::<T>::SetClassDuplicate.into())
    }
  Class::<T>::put(class);
  Self::deposit_event(Event::SetClass(class));

  Ok(().into())
}

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
    student_number,
    student_name));

  Ok(().into())
}

#[pallet::weight(0)]
pub fn set_dorm_info(
  origin: OriginFor<T>,
  dorm_number: u32,
  bed_number: u32,
  student_number: u32,
) -> DispatchResultWithPostInfo {
  ensure_signed(origin)?;
  if DormInfo::<T>::contains_key(dorm_number, bed_number) {
                return Err(Error::<T>::SetDormInfoDuplicate.into())
  }
  DormInfo::<T>::insert(&dorm_number,
     &bed_number,
     &student_number);
  Self::deposit_event(Event::SetDormInfo(
    dorm_number,
    bed_number,
    student_number));

  Ok(().into())
}
```

基本上都是判断发起交易者的权限，然后设置信息、发出事件这样的过程，整个 pallet 完整的代码可以参考这里.

## 在 runtime 中使用

写完 pallet 后，我们就可以将 pallet 添加到 runtime 中。

- 添加依赖，在 substrate-node-template/runtime/Cargo.toml 中添加如下代码：

```

[dependencies]
...
#添加下面这行
pallet-use-storage = {
  version = "4.0.0-dev",
  default-features = false,
  path = "../pallets/use-storage"
}
...

[features]
default = ["std"]
std = [
  "codec/std",
  ...
  #添加下面这行
  "pallet-use-storage/std",
  ...
  ]
```

- 修改 runtime/src/lib.rs，添加如下代码：

```
...
impl pallet_use_storage::Config for Runtime {
  type Event = Event;
}
...

construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    System: frame_system,
    ...
    TemplateModule: pallet_template,
    SimplePallet: pallet_simple_pallet,
    //添加下面这行
    UseStorage: pallet_use_storage,
  }
);
```

## 编译&运行

回到 substrate-node-template 目录，执行如下编译：

```
cargo build

```

启动节点：

```
./target/debug/node-template --dev
```
