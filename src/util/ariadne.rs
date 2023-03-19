use ariadne::{Color, Label, Report, ReportKind};

use super::source::Span;

pub fn span_error_report<'a>(
    filename: &'a str,
    span: &Span,
    msg: &str,
) -> Report<'a, (&'a str, Span)> {
    Report::build(ReportKind::Error, filename, span.start)
        .with_label(
            Label::new((filename, span.clone()))
                .with_color(Color::Red)
                .with_message(msg),
        )
        .finish()
}

pub fn span_error_report_with_note<'a>(
    filename: &'a str,
    span: &Span,
    msg: &str,
    note: &str,
) -> Report<'a, (&'a str, Span)> {
    Report::build(ReportKind::Error, filename, span.start)
        .with_label(
            Label::new((filename, span.clone()))
                .with_color(Color::Red)
                .with_message(msg),
        )
        .with_note(note)
        .finish()
}

pub fn error_report<'a>(filename: &'a str, msg: &str) -> Report<'a, (&'a str, Span)> {
    Report::build(ReportKind::Error, filename, 0)
        .with_message(msg)
        .finish()
}
