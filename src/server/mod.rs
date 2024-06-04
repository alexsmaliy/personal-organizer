mod get_bookmarks;
mod signup;

pub(crate) use get_bookmarks::{get_bookmarks, GetBookmarks};
pub(crate) use signup::{sign_up, SignUp};

use lazy_static::lazy_static;

#[cfg(feature="ssr")]
lazy_static! {
    pub(crate) static ref PROTECTED_ROUTES: hashbrown::HashSet<&'static str> = {
        let mut set = hashbrown::HashSet::new();
        set.insert("/api/get-bookmarks");
        set
    };
}