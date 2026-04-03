use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    LexerError,
    ParseError,
    SemanticError,
    Warning,
}

#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: ErrorKind,
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub source_line: String,
    pub suggestion: Option<String>,
}

impl CompileError {
    pub fn new(
        kind: ErrorKind,
        message: impl Into<String>,
        line: usize,
        col: usize,
        source_line: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            line,
            col,
            source_line: source_line.into(),
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    pub fn lexer(
        message: impl Into<String>,
        line: usize,
        col: usize,
        source_line: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::LexerError, message, line, col, source_line)
    }

    pub fn parse(
        message: impl Into<String>,
        line: usize,
        col: usize,
        source_line: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::ParseError, message, line, col, source_line)
    }

    pub fn semantic(
        message: impl Into<String>,
        line: usize,
        col: usize,
        source_line: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::SemanticError, message, line, col, source_line)
    }

    pub fn warning(
        message: impl Into<String>,
        line: usize,
        col: usize,
        source_line: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::Warning, message, line, col, source_line)
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.kind {
            ErrorKind::Warning => "Warning",
            _ => "Error",
        };

        write!(
            f,
            "{} [line {}, col {}]: {}",
            label, self.line, self.col, self.message
        )?;

        if !self.source_line.is_empty() {
            let line_str = self.line.to_string();
            let gutter_width = line_str.len() + 5;
            write!(f, "\n\n  {} | {}", line_str, self.source_line)?;
            write!(
                f,
                "\n{}^",
                " ".repeat(gutter_width + self.col.saturating_sub(1))
            )?;
        }

        if let Some(ref suggestion) = self.suggestion {
            write!(f, "\n  {}", suggestion)?;
        }

        Ok(())
    }
}

impl std::error::Error for CompileError {}

pub fn get_source_line(source: &str, line: usize) -> String {
    source
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .to_string()
}

pub fn suggest_closest(input: &str, candidates: &[&str], max_distance: usize) -> Option<String> {
    let input_lower = input.to_lowercase();
    candidates
        .iter()
        .map(|c| (*c, levenshtein(&input_lower, c)))
        .filter(|(_, d)| *d > 0 && *d <= max_distance)
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c.to_string())
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[m][n]
}
