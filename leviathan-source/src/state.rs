use crate::{error::SourceError, grouper::TokenGroup, parser::Token, source::Source};

pub struct SourceState {
    errors: Vec<SourceError>,
    source: Source,
    valid: bool,
    tokens: Vec<Token>,
    groups: Vec<TokenGroup>,
}
