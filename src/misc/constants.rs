use lazy_static::lazy_static;
use regex::Regex;

#[cfg(feature="ssr")]
pub(crate) const SESSION_DURATION: time::Duration = time::Duration::new(30 * 60, 0);

lazy_static! {
    // 1 x (letter | number | punct | symbol | CJK | emoji)
    // 2+ x (as above, but also any space)
    // 1 x (letter | number | punct | symbol | CJK | emoji)
    pub(crate) static ref REGEX: Regex = Regex::new("\
        ^\
            [\
                \\p{Alphabetic}\
                \\p{Decimal_Number}\
                \\p{Punctuation}\
                \\p{Symbol}\
                \\p{Ideographic}\
                \\p{Emoji}\
            ]\
            [\
                \\p{Alphabetic}\
                \\p{Decimal_Number}\
                \\p{Punctuation}\
                \\p{Symbol}\
                \\p{Space_Separator}\
                \\p{Ideographic}\
                \\p{Emoji}\
            ]\
            {2,}\
            [\
                \\p{Alphabetic}\
                \\p{Decimal_Number}\
                \\p{Punctuation}\
                \\p{Symbol}\
                \\p{Ideographic}\
                \\p{Emoji}\
            ]\
        $\
    ").expect("validation regex didn't compile!");
}
