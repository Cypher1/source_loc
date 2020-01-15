/// Captures a [SourceLoc](struct.SourceLoc.html) source location.
///
/// # Example
///
/// ```rust
/// use source_loc::source_loc;
///
/// let loc = source_loc!();
/// assert_eq!(
///     "src/source_loc.rs:6:11",
///     &loc.to_string(),
/// );
/// ```
#[macro_export]
macro_rules! source_loc {
    () => {
        crate::source_loc::SourceLoc {
            file: std::file!(),
            line: std::line!(),
            column: std::column!(),
        }
    };
}

// tests as early as possible so the line numbers are more stable : )
#[cfg(test)]
mod tests {
    #[test]
    fn it_can_debug() {
        let loc = source_loc!();
        assert_eq!("src/source_loc.rs:30:19", &format!("{:?}", loc));
    }

    #[test]
    fn it_can_display() {
        let loc = source_loc!();
        assert_eq!("src/source_loc.rs:36:19", &format!("{}", loc));
    }
}

/// A captured source location. To capture, use the [source_loc!()](macro.source_loc.html) macro.
///
/// # Example
///
/// ```rust
/// use source_loc::source_loc;
///
/// let loc = source_loc!();
///
/// assert_eq!(
///     "src/source_loc.rs:6:11",
///     &loc.to_string(),
/// );
///
/// assert_eq!(
///     "src/source_loc.rs",
///     loc.file,
/// );
///
/// assert_eq!(6, loc.line);
///
/// assert_eq!(11, loc.column);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLoc {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Debug for SourceLoc {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

impl std::fmt::Display for SourceLoc {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
