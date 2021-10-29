// use std::future::Future;
// use std::pin::Pin;

mod trait_object_in_fn;
mod service;

pub use service::*;
pub use trait_object_in_fn::*;

// 在参数中使用 trait object
// pub trait CookieStore: Send + Sync {
//     // 在 set_cookies 方法中使用了 &mut dyn Iterator 这样一个 trait object。
//     fn set_cookies(
//         &self,
//         cookie_headers: &mut dyn Iterator<Item=&HeaderValue>,
//         url: &Url,
//     );
//     fn cookies(&self, url: &Url) -> Option<HeaderValue>;
// }


// Rust 的 async trait 还没有稳定，可以用 async_trait 宏
// #[async_trait]
// pub trait Fetch {
//     type Error;
//     async fn fetch(&self) -> Result<String, Self::Error>;
// }

// 上面的宏展开后，类似于
// 使用了 trait object 作为返回值。
// 这样，不管 fetch() 的实现，返回什么样的 Future 类型，
// 都可以被 trait object 统一起来，调用者只需要按照正常 Future 的接口使用即可
// pub trait Fetch {
//     type Error;
//     fn fetch<'a>(&'a self) ->
//     Result<Pin<Box<dyn Future<Output=String> + Send + 'a>>, Self::Error>;
// }

// 在数据结构中使用 trait object 典型场景：闭包
// pub struct AttributeGetter(
//     Arc<dyn Fn(&Instance, &mut Host) -> crate::Result<PolarValue> + Send + Sync>,
// );

// pub struct Input<'a, T> {
//     prompt: String,
//     default: Option<T>,
//     show_default: bool,
//     initial_text: Option<String>,
//     theme: &'a dyn Theme,
//     permit_empty: bool,
//     validator: Option<Box<dyn FnMut(&T) -> Option<String> + 'a>>,
//     #[cfg(feature = "history")]
//     history: Option<&'a mut dyn History<T>>,
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
