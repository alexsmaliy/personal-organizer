use std::fmt::Write;

#[cfg(feature = "ssr")]
#[inline(always)]
pub(crate) fn random_string(length: usize) -> Result<String, getrandom::Error> {
    let half = length.div_ceil(2);
    let mut buf = vec![0; half];
    match getrandom::getrandom(buf.as_mut()) {
        Ok(_) => {
            let mut s = String::with_capacity(length);
            for byte in buf {
                write!(s, "{byte:0>2x}");
            }
            s.truncate(length);
            Ok(s)
        },
        Err(e) => Err(e), // gotta noop-convert a Result<(), E> into a Result<String, E>
    }
}
