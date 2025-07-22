use crate::parser::*;
use matcher_macros::define_matcher;

define_matcher!(Sql {
    delimiters: [
        "(" => ")",
        "[" => "]",
        "{" => "}"
    ],
    line_comment: ["--", "#"],
    block_comment: ["/*" => "*/"],
    string: ["\"", "'", "$$", "`"] // TODO: tag encoding: $tag$text$tag$
});
