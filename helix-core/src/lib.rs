pub use encoding_rs as encoding;

pub mod auto_pairs;
pub mod case_conversion;
pub mod chars;
pub mod command_line;
pub mod comment;
pub mod completion;
pub mod config;
pub mod diagnostic;
pub mod diff;
pub mod doc_formatter;
pub mod fuzzy;
pub mod graphemes;
pub mod history;
pub mod increment;
pub mod indent;
pub mod line_ending;
pub mod macros;
pub mod match_brackets;
pub mod movement;
pub mod object;
mod position;
pub mod search;
pub mod selection;
pub mod snippets;
pub mod surround;
pub mod syntax;
pub mod test;
pub mod text_annotations;
pub mod textobject;
mod transaction;
pub mod uri;
pub mod wrap;

pub mod unicode {
    pub use unicode_general_category as category;
    pub use unicode_segmentation as segmentation;
    pub use unicode_width as width;
}

pub use helix_loader::find_workspace;

mod rope_reader;

pub use rope_reader::RopeReader;
pub use ropey::{self, str_utils, Rope, RopeBuilder, RopeSlice};

// pub use tendril::StrTendril as Tendril;
pub use smartstring::SmartString;

pub type Tendril = SmartString<smartstring::LazyCompact>;

#[doc(inline)]
pub use {regex, tree_sitter};

pub use position::{
    char_idx_at_visual_offset, coords_at_pos, pos_at_coords, softwrapped_dimensions,
    visual_offset_from_anchor, visual_offset_from_block, Position, VisualOffsetError,
};
#[allow(deprecated)]
pub use position::{pos_at_visual_coords, visual_coords_at_pos};

pub use selection::{Range, Selection};
pub use smallvec::{smallvec, SmallVec};
pub use syntax::Syntax;

pub use completion::CompletionItem;

pub use line_ending::{LineEnding, NATIVE_LINE_ENDING};
pub use transaction::{Assoc, Change, ChangeSet, Deletion, Operation, Transaction};

pub use uri::Uri;

/// A language to use for spell checking.
///
/// This is defined in the form `"ab_CD"` where `a`, `b`, `C` and `D` are all ASCII alphanumeric.
/// The first two letters declare the ISO 639 language code and the later two declare the
/// territory identifier. The territory identifier is optional, so a language may just be `"ab"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellingLanguage([u8; 5]);

impl SpellingLanguage {
    pub const EN_US: Self = Self(*b"en_US");

    pub fn as_str(&self) -> &str {
        // SAFETY: `.0` is all ASCII bytes which is valid UTF-8.
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl std::fmt::Display for SpellingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
