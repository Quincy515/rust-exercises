use bytes::Bytes;
use pin_project_lite::pin_project;
use std::ops::Deref;

/// A string like type backed by `Bytes` making it cheap to clone.
// 包装了 Bytes
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ByteStr(Bytes);

// 给包装的 Bytes 实现 Deref
impl Deref for ByteStr {
    type Target = str;

    #[inline]
    // 自动解引用转换为 &str
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl ByteStr {
    pub(crate) fn new<S>(s: S) -> Self
        where
            S: AsRef<str>,
    {
        Self(Bytes::copy_from_slice(s.as_ref().as_bytes()))
    }

    // 实现 as_str 方法，转换为字符串切片 &str
    pub(crate) fn as_str(&self) -> &str {
        // `ByteStr` can only be constructed from strings which are always valid
        // utf-8 so this wont panic.
        std::str::from_utf8(&self.0).unwrap()
    }
}

pin_project! {
    #[project = EitherProj]
    pub(crate) enum Either<A, B> {
        A { #[pin] inner: A },
        B { #[pin] inner: B },
    }
}