//! WordSearch component generator
//!
//! Generates a parameter-driven word search puzzle grid with word placement logic,
//! random letter fill, and optional solution tracking.

use super::Component;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Grid cell model passed to Typst
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordSearchCell {
    /// Character at this cell
    pub char: String,
    /// Whether this cell is part of a hidden word
    pub is_solution: bool,
}

/// Simple deterministic PRNG (Xorshift64) for reproducible grid generation
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_usize(&mut self, max: usize) -> usize {
        if max == 0 {
            0
        } else {
            // Reduce modulo `max` while still widened, so the final cast to usize
            // only ever narrows a value already known to fit (< max).
            (self.next_u64() % max as u64) as usize
        }
    }

    fn random_char(&mut self) -> char {
        // idx is in 0..26 by construction, always fits in u8.
        let idx = self.next_usize(26) as u8;
        (b'A' + idx) as char
    }
}

/// Direction vector for word placement
#[derive(Debug, Clone, Copy)]
struct Direction {
    dx: isize,
    dy: isize,
}

impl Direction {
    const RIGHT: Self = Self { dx: 1, dy: 0 };
    const DOWN: Self = Self { dx: 0, dy: 1 };
    const DOWN_RIGHT: Self = Self { dx: 1, dy: 1 };
    const UP_RIGHT: Self = Self { dx: 1, dy: -1 };
    const LEFT: Self = Self { dx: -1, dy: 0 };
    const UP: Self = Self { dx: 0, dy: -1 };
    const UP_LEFT: Self = Self { dx: -1, dy: -1 };
    const DOWN_LEFT: Self = Self { dx: -1, dy: 1 };
}

/// WordSearch component builder and generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordSearch {
    /// Optional title for the puzzle
    pub title: Option<String>,
    /// Optional description / instructions
    pub description: Option<String>,
    /// Words to hide in the puzzle grid
    pub words: Vec<String>,
    /// Grid width in cells
    pub width: usize,
    /// Grid height in cells
    pub height: usize,
    /// Allow diagonal word placement
    pub allow_diagonal: bool,
    /// Allow reverse (right-to-left, bottom-to-top) placement
    pub allow_reverse: bool,
    /// Seed for random placement (for reproducible PDFs)
    pub seed: Option<u64>,
    /// Whether to highlight the solution cells
    pub show_solution: bool,
    /// Number of columns for displaying the word list
    pub columns_word_list: usize,
}

impl Default for WordSearch {
    fn default() -> Self {
        Self {
            title: Some("Wortsuchrätsel".into()),
            description: Some("Finde alle versteckten Wörter im Buchstabengitter!".into()),
            words: Vec::new(),
            width: 12,
            height: 12,
            allow_diagonal: true,
            allow_reverse: false,
            seed: Some(42),
            show_solution: false,
            columns_word_list: 3,
        }
    }
}

impl WordSearch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> WordSearchBuilder {
        WordSearchBuilder::new()
    }
}

/// Builder pattern for `WordSearch`
pub struct WordSearchBuilder {
    inner: WordSearch,
}

impl WordSearchBuilder {
    pub fn new() -> Self {
        Self {
            inner: WordSearch::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.inner.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }

    pub fn words<I, S>(mut self, words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.inner.words = words.into_iter().map(Into::into).collect();
        self
    }

    pub fn grid_size(mut self, width: usize, height: usize) -> Self {
        self.inner.width = width.max(3);
        self.inner.height = height.max(3);
        self
    }

    pub fn allow_diagonal(mut self, allow: bool) -> Self {
        self.inner.allow_diagonal = allow;
        self
    }

    pub fn allow_reverse(mut self, allow: bool) -> Self {
        self.inner.allow_reverse = allow;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.inner.seed = Some(seed);
        self
    }

    pub fn show_solution(mut self, show: bool) -> Self {
        self.inner.show_solution = show;
        self
    }

    pub fn columns_word_list(mut self, cols: usize) -> Self {
        self.inner.columns_word_list = cols.max(1);
        self
    }

    pub fn build(self) -> crate::Result<WordSearch> {
        Ok(self.inner)
    }
}

impl Component for WordSearch {
    fn component_id(&self) -> &str {
        "word-search"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut rng = SimpleRng::new(self.seed.unwrap_or(42));
        let width = self.width;
        let height = self.height;

        let mut grid: Vec<Vec<Option<char>>> = vec![vec![None; width]; height];
        let mut solution_cells: HashSet<(usize, usize)> = HashSet::new();

        // Clean & prepare words (uppercase, ascii/german chars)
        let mut clean_words: Vec<String> = self
            .words
            .iter()
            .map(|w| {
                w.to_uppercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|w| !w.is_empty())
            .collect();

        // Sort by length descending for better placement density
        clean_words.sort_by(|a, b| b.len().cmp(&a.len()));

        // Determine available directions
        let mut directions = vec![Direction::RIGHT, Direction::DOWN];
        if self.allow_diagonal {
            directions.push(Direction::DOWN_RIGHT);
            directions.push(Direction::UP_RIGHT);
        }
        if self.allow_reverse {
            directions.push(Direction::LEFT);
            directions.push(Direction::UP);
            if self.allow_diagonal {
                directions.push(Direction::UP_LEFT);
                directions.push(Direction::DOWN_LEFT);
            }
        }

        // Place words into grid
        let mut placed_words = Vec::new();
        for word in &clean_words {
            let chars: Vec<char> = word.chars().collect();
            let len = chars.len();

            // Try multiple random attempts to place word
            for _ in 0..300 {
                let dir = directions[rng.next_usize(directions.len())];
                let start_col = rng.next_usize(width);
                let start_row = rng.next_usize(height);

                let end_col = start_col as isize + dir.dx * (len as isize - 1);
                let end_row = start_row as isize + dir.dy * (len as isize - 1);

                if end_col >= 0
                    && end_col < width as isize
                    && end_row >= 0
                    && end_row < height as isize
                {
                    // Position of the i-th letter of the word, in [0, len). Safe to cast
                    // back to usize: bounded between (start_col, start_row) and
                    // (end_col, end_row), both already checked to be within the grid.
                    let cell_at = |i: usize| -> (usize, usize) {
                        let c = start_col as isize + dir.dx * i as isize;
                        let r = start_row as isize + dir.dy * i as isize;
                        (c as usize, r as usize)
                    };

                    // Check if placement is valid (cells empty or match char)
                    let mut fits = true;
                    for (i, &ch) in chars.iter().enumerate() {
                        let (c, r) = cell_at(i);
                        if let Some(existing) = grid[r][c] {
                            if existing != ch {
                                fits = false;
                                break;
                            }
                        }
                    }

                    if fits {
                        // Place characters and record solution
                        for (i, &ch) in chars.iter().enumerate() {
                            let (c, r) = cell_at(i);
                            grid[r][c] = Some(ch);
                            solution_cells.insert((c, r));
                        }
                        placed_words.push(word.clone());
                        break;
                    }
                }
            }
        }

        // Fill remaining empty cells with random letters
        let rendered_grid: Vec<Vec<WordSearchCell>> = (0..height)
            .map(|r| {
                (0..width)
                    .map(|c| {
                        let ch = grid[r][c].unwrap_or_else(|| rng.random_char());
                        let is_solution = solution_cells.contains(&(c, r));
                        WordSearchCell {
                            char: ch.to_string(),
                            is_solution,
                        }
                    })
                    .collect()
            })
            .collect();

        // Use original words or placed clean words
        let display_words = if placed_words.is_empty() {
            self.words.clone()
        } else {
            placed_words
        };

        serde_json::json!({
            "title": self.title,
            "description": self.description,
            "width": width,
            "height": height,
            "grid": rendered_grid,
            "words": display_words,
            "show_solution": self.show_solution,
            "columns_word_list": self.columns_word_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordsearch_builder_defaults() {
        let ws = WordSearch::builder()
            .words(vec!["RUST", "TYPST"])
            .grid_size(10, 10)
            .build()
            .unwrap();

        assert_eq!(ws.component_id(), "word-search");
        assert_eq!(ws.width, 10);
        assert_eq!(ws.height, 10);
        assert_eq!(ws.words.len(), 2);
    }

    #[test]
    fn test_wordsearch_grid_generation() {
        let ws = WordSearch::builder()
            .words(vec!["APPLE", "BANANA", "CHERRY"])
            .grid_size(8, 8)
            .seed(12345)
            .build()
            .unwrap();

        let data = ws.to_data();
        assert_eq!(data["width"], 8);
        assert_eq!(data["height"], 8);

        let grid = data["grid"].as_array().expect("grid should be array");
        assert_eq!(grid.len(), 8);
        assert_eq!(grid[0].as_array().unwrap().len(), 8);

        // Check solution cells are present
        let mut solution_count = 0;
        for row in grid {
            for cell in row.as_array().unwrap() {
                if cell["is_solution"].as_bool().unwrap_or(false) {
                    solution_count += 1;
                }
            }
        }
        assert!(solution_count > 0);
    }

    #[test]
    fn test_wordsearch_deterministic_seed() {
        let ws1 = WordSearch::builder()
            .words(vec!["ALPHA", "BETA", "GAMMA"])
            .seed(999)
            .build()
            .unwrap();
        let ws2 = WordSearch::builder()
            .words(vec!["ALPHA", "BETA", "GAMMA"])
            .seed(999)
            .build()
            .unwrap();

        assert_eq!(ws1.to_data(), ws2.to_data());
    }
}
