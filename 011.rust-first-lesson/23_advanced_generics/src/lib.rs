mod identifier;
mod auth;
mod iterator;
mod functions;
mod complex_args;


pub use auth::*;
pub use complex_args::*;
pub use functions::*;
pub use identifier::*;
pub use iterator::*;


#[allow(dead_code)]
pub struct BufReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}

// fn for_each_concurrent<Fut, F>(self, limit: impl Into<Option<usize>>, f: F) -> ForEachConcurrent<Self, Fut, F>
//     where
//         F: FnMut(Self::Item) -> Fut,
//         Fut: Future<Output=()>,
//         Self: Sized, {}
//
//

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
