// SPDX-License-Identifier: GPL-3.0

pub mod files;
mod highlighter;
mod toast;

pub use highlighter::SyntectHighlighter;
pub use highlighter::SyntectSettings;
pub use highlighter::to_format;
pub use toast::CedillaToast;
