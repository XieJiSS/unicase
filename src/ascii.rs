use alloc::borrow::Cow;
use alloc::string::String;
#[cfg(__unicase__iter_cmp)]
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::str::FromStr;
#[cfg(not(__unicase__core_and_alloc))]
#[allow(deprecated, unused)]
use std::ascii::AsciiExt;

use super::{Ascii, Encoding, UniCase};

impl<S> Ascii<S> {
    #[inline]
    #[cfg(__unicase__const_fns)]
    pub const fn new(s: S) -> Ascii<S> {
        Ascii(s)
    }

    /// Construct a new `Ascii`.
    ///
    /// For Rust versions >= 1.31, this is a `const fn`.
    #[inline]
    #[cfg(not(__unicase__const_fns))]
    pub fn new(s: S) -> Ascii<S> {
        Ascii(s)
    }

    #[cfg(__unicase__const_fns)]
    pub const fn into_unicase(self) -> UniCase<S> {
        UniCase(Encoding::Ascii(self))
    }

    #[cfg(not(__unicase__const_fns))]
    pub fn into_unicase(self) -> UniCase<S> {
        UniCase(Encoding::Ascii(self))
    }

    #[inline]
    pub fn into_inner(self) -> S {
        self.0
    }
}

#[cfg(feature = "serde")]
impl<'de, S> serde::Deserialize<'de> for Ascii<S>
where
    S: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Ascii<S>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        S::deserialize(deserializer).map(Ascii)
    }
}

#[cfg(feature = "serde")]
impl<S> serde::Serialize for Ascii<S>
where
    S: serde::Serialize,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<S> Deref for Ascii<S> {
    type Target = S;
    #[inline]
    fn deref<'a>(&'a self) -> &'a S {
        &self.0
    }
}

impl<S> DerefMut for Ascii<S> {
    #[inline]
    fn deref_mut<'a>(&'a mut self) -> &'a mut S {
        &mut self.0
    }
}

#[cfg(__unicase__iter_cmp)]
impl<T: AsRef<str>> PartialOrd for Ascii<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(__unicase__iter_cmp)]
impl<T: AsRef<str>> Ord for Ascii<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chars = self.as_ref().chars().map(|c| c.to_ascii_lowercase());
        let other_chars = other.as_ref().chars().map(|c| c.to_ascii_lowercase());
        self_chars.cmp(other_chars)
    }
}

impl<S: AsRef<str>> AsRef<str> for Ascii<S> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<S: fmt::Display> fmt::Display for Ascii<S> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl<S1: AsRef<str>> PartialEq<Ascii<S1>> for String {
    #[inline]
    fn eq(&self, other: &Ascii<S1>) -> bool {
        other == self
    }
}

impl<'a, S1: AsRef<str>> PartialEq<Ascii<S1>> for &'a str {
    #[inline]
    fn eq(&self, other: &Ascii<S1>) -> bool {
        other == self
    }
}

impl<S1: AsRef<str>, S2: AsRef<str>> PartialEq<S2> for Ascii<S1> {
    #[inline]
    fn eq(&self, other: &S2) -> bool {
        self.as_ref().eq_ignore_ascii_case(other.as_ref())
    }
}

impl<S: AsRef<str>> Eq for Ascii<S> {}

impl<S: FromStr> FromStr for Ascii<S> {
    type Err = <S as FromStr>::Err;
    fn from_str(s: &str) -> Result<Ascii<S>, <S as FromStr>::Err> {
        s.parse().map(Ascii)
    }
}

macro_rules! from_impl {
    ($from:ty => $to:ty; $by:ident) => (
        impl<'a> From<$from> for Ascii<$to> {
            fn from(s: $from) -> Self {
                Ascii(s.$by())
            }
        }
    );
    ($from:ty => $to:ty) => ( from_impl!($from => $to; into); )
}

impl<S: AsRef<str>> From<S> for Ascii<S> {
    #[inline]
    fn from(s: S) -> Ascii<S> {
        Ascii(s)
    }
}

impl<S: AsRef<str> + Clone> From<&S> for Ascii<S> {
    #[inline]
    fn from(s: &S) -> Ascii<S> {
        let owned = s.clone();
        Ascii(owned)
    }
}

macro_rules! into_impl {
    ($to:ty) => {
        impl<'a> Into<$to> for Ascii<$to> {
            fn into(self) -> $to {
                self.0.into()
            }
        }
    };
}

from_impl!(&'a str => Cow<'a, str>);
from_impl!(String => Cow<'a, str>);
from_impl!(&'a str => String);
from_impl!(Cow<'a, str> => String; into_owned);
from_impl!(&'a String => &'a str; as_ref);

into_impl!(String);
into_impl!(alloc::borrow::Cow<'a, str>);
into_impl!(&'a str);

impl<S: AsRef<str>> Hash for Ascii<S> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for byte in self.as_ref().bytes().map(|b| b.to_ascii_lowercase()) {
            hasher.write_u8(byte);
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(__unicase__default_hasher)]
    use std::collections::hash_map::DefaultHasher;
    #[cfg(not(__unicase__default_hasher))]
    use std::hash::SipHasher as DefaultHasher;
    use std::hash::{Hash, Hasher};
    use Ascii;

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_case_insensitive() {
        let a = Ascii("foobar");
        let b = Ascii("FOOBAR");

        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));

        assert_eq!(a, "fooBar");
        assert_eq!("fooBar", a);
        assert_eq!(String::from("fooBar"), a);
        assert_eq!(a, String::from("fooBar"));
    }

    #[test]
    fn test_ref_into() {
        let input = "foobar".to_string();
        let _: Ascii<String> = (&input).into();
    }

    #[test]
    fn test_into_impls() {
        let a = Ascii("foobar");
        let b: &str = a.into();
        assert_eq!(b, "foobar");

        let c = Ascii("FOOBAR".to_string());
        let d: String = c.into();
        assert_eq!(d, "FOOBAR");

        let view: Ascii<&'static str> = Ascii("foobar");
        let _: &'static str = view.into();
        let _: &str = view.into();

        let owned: Ascii<String> = "foobar".into();
        let _: String = owned.clone().into();
        let _: &str = owned.as_ref();
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_ascii_eq(b: &mut ::test::Bencher) {
        b.bytes = b"foobar".len() as u64;
        b.iter(|| assert_eq!(Ascii("foobar"), Ascii("FOOBAR")));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_ascii_deserialize() {
        let a = serde_json::from_str::<Ascii<String>>("\"foobar\"").unwrap();
        assert_eq!(a, Ascii("foobar"));
        let b = serde_json::from_str::<Ascii<String>>("\"FOOBAR\"").unwrap();
        assert_eq!(b, Ascii("FOOBAR"));
        assert_eq!(b, Ascii("foobar"));
        assert_eq!(b, a);
        assert_ne!(b, Ascii("baz"));

        let c = serde_json::from_str::<Ascii<&str>>("\"baz\"").unwrap();
        assert_eq!(c, Ascii("baz"));
        assert_eq!(c, Ascii("Baz"));
        assert_eq!(c, Ascii("Baz".to_string()));

        let mut map = std::collections::HashMap::<Ascii<&str>, i32>::new();
        map.insert("abc".into(), 2);
        assert_eq!(map.get(&"abc".into()), Some(&2));
        assert_eq!(map.get(&"ABC".into()), Some(&2));
        assert_eq!(map.len(), 1);
        // test if the value is updated correctly using a different cased key
        let old = map.insert("ABC".into(), 3);
        assert_eq!(old, Some(2));
        assert_eq!(map.get(&"abc".into()), Some(&3));
        assert_eq!(map.get(&"ABC".into()), Some(&3));
        assert_eq!(map.len(), 1);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_ascii_serialize() {
        let a = serde_json::to_string(&Ascii("foobar")).unwrap();
        assert_eq!(a, "\"foobar\"");
        let b = serde_json::to_string(&Ascii("FOOBAR")).unwrap();
        assert_eq!(b, "\"FOOBAR\"");
    }

    #[cfg(__unicase__iter_cmp)]
    #[test]
    fn test_case_cmp() {
        assert!(Ascii("foobar") == Ascii("FOOBAR"));
        assert!(Ascii("a") < Ascii("B"));

        assert!(Ascii("A") < Ascii("b"));
        assert!(Ascii("aa") > Ascii("a"));

        assert!(Ascii("a") < Ascii("aa"));
        assert!(Ascii("a") < Ascii("AA"));
    }

    #[cfg(__unicase__const_fns)]
    #[test]
    fn test_ascii_new_const() {
        const _ASCII: Ascii<&'static str> = Ascii::new("");
    }
}
