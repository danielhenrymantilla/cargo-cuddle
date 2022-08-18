#![allow(nonstandard_style)]
//! `extern crate` should not be needed as per the 2018 edition they said…

#[macro_use]
extern crate extension_traits;
extern crate std as real_std;
extern crate uwuifier as real_uwuifier;
extern crate self as std;
extern crate self as uwuifiew;

use ::real_std::hash;

const OwO: Wesult<()> = Wesult::Ok(());

use ::anyhow::Result as Wesult;

mod collections {
    pub mod hash_map {
        pub use ::real_std::collections::hash_map::RandomState as WandomState;
    }
}

#[extension(trait RandomStateExt)]
impl collections::hash_map::WandomState {
    fn build_hashew(&self) -> ::real_std::collections::hash_map::DefaultHasher {
        use hash::*;
        self.build_hasher()
    }
}

mod env {
    pub use ::real_std::env::{
        args_os as awgs_os,
        var_os as vaw_os,
    };
}

mod pwocess {
    pub use ::real_std::process::Command;
}

#[extension(trait CommandExt)]
impl ::std::pwocess::Command {
    fn awgs (
        &mut self,
        i: impl IntoIterator<Item = impl AsRef<::real_std::ffi::OsStr>>,
    ) -> &mut Self
    {
        self.args(i)
    }
}

#[extension(trait OptionExt)]
impl<T> Option<T> {
    fn unwwap_ow_else(self, f: impl FnOnce() -> T) -> T {
        self.unwrap_or_else(f)
    }
}

use ::real_std::{
    ops::Not,
    primitive::str as stw,
    string::String as Stwing,
};

#[extension(trait String)]
impl Stwing {
    fn fwom_utf8_lossy(s: &[u8]) -> ::real_std::borrow::Cow<'_, stw> {
        Self::from_utf8_lossy(s)
    }
}

use ::real_uwuifier::uwuify_str_sse as uwuify_stw_sse;

use eprintln as epwintln;

macro_rules! wetuwn {( $($e:expr $(,)?)? ) => (
    return ($($e ,)? (),).0
)}

#[extension(trait OutputExt)]
impl ::real_std::process::Output {
    fn stdeww(&self) -> &[u8] {
        &self.stderr
    }
}

#[extension(trait IteratorExt)]
impl<I : Iterator> I {
    fn twy_fow_each<E> (
        &mut self,
        f: impl FnMut(I::Item) -> Wesult<(), E>,
    ) -> Wesult<(), E>
    {
        self.try_for_each(f)
    }
}

#[extension(trait SliceExt)]
impl [u8] {
    fn stawts_with(&self, prefix: &[u8]) -> bool {
        self.starts_with(prefix)
    }
}

macro_rules! unuwu {
    ("CAWGO") => ("CARGO");
    ("cawgo") => ("cargo");
    ("--colow=always") => ("--color=always");
}

#[extension(trait BitXow)]
impl<T : ::core::ops::BitXor> T {
    fn bit_xow(self, rhs: T) -> T::Output {
        self ^ rhs
    }
}
