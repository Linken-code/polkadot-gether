# 在 ocw 中发送 http 请求

本节我们开始学习在 ocw 中发送 http 请求，主要用到 sp_runtime::offchain::http 和 lite_json 库。详细的实现我们看代码。

## ocw 中发送 http 请求实现

这部分的功能非常简单，就是发送请求https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD 来获取 btc 价格，获取价格的主要代码如下：

```
impl<T: Config> Pallet<T> {
    fn parse_price(price_str: &str) -> Option<u32> {
      let val = lite_json::parse_json(price_str);
      let price = match val.ok()? {
        JsonValue::Object(obj) => {
          let (_, v) =
            obj.into_iter().find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
          match v {
            JsonValue::Number(number) => number,
            _ => return None,
          }
        },
        _ => return None,
      };

      let exp = price.fraction_length.saturating_sub(2);
      Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
    }

    fn fetch_price() -> Result<u32, http::Error> {
      let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
      let request = http::Request::get(
        "https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD",
      );
      let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

      let response =
        pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
      if response.code != 200 {
        log::warn!("Unexpected status code: {}", response.code);
        return Err(http::Error::Unknown)
      }

      let body = response.body().collect::<Vec<u8>>();

      let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
        log::warn!("No UTF8 body");
        http::Error::Unknown
      })?;

      let price = match Self::parse_price(body_str) {
        Some(price) => Ok(price),
        None => {
          log::warn!("Unable to extract price from the response: {:?}", body_str);
          Err(http::Error::Unknown)
        },
      }?;

      log::warn!("Got price: {} cents", price);
      Ok(price)
    }
```

然后我们在 ocw 中调用上面实现的 fetch_price 函数，当获取到价格后便将值打印出来，如下：

```
#[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(_block_number: T::BlockNumber) {
      if let Ok(data) = Self::fetch_price() {
        log::info!(target:"offchain-index-demo", "1. get price, price ======================== {:?}", data);
      } else {
        log::info!(target:"offchain-index-demo", "2. get price failed ==================== ");
      }
    }
  }
```

至此，pallet 中的功能基本就实现好了。然后我们只需要将该 pallet 加入到 runtime 中就可以进行测试了
