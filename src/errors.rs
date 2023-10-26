use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Ctx)), visibility(pub(crate)))]
pub(crate) enum Error {
    #[snafu(display("[{span}] {source}"))]
    Db {
        source: sqlx::Error,
        span: &'static str,
    },

    #[snafu(display("[{span}] {source} [while resolving] {also}"))]
    Combo {
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
        also: Box<Error>,
        span: &'static str,
    }
}
