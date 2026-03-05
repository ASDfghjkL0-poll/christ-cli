use crate::data::books::{self, BookInfo};

/// A parsed Bible reference.
#[derive(Debug, Clone)]
pub struct BibleReference {
    pub book: &'static BookInfo,
    pub chapter: u32,
    pub verse_start: Option<u32>,
    pub verse_end: Option<u32>,
}

impl BibleReference {
    pub fn display(&self) -> String {
        match (self.verse_start, self.verse_end) {
            (Some(start), Some(end)) if start != end => {
                format!("{} {}:{}-{}", self.book.name, self.chapter, start, end)
            }
            (Some(start), _) => {
                format!("{} {}:{}", self.book.name, self.chapter, start)
            }
            _ => {
                format!("{} {}", self.book.name, self.chapter)
            }
        }
    }
}

/// Parse a Bible reference string into a structured reference.
///
/// Supports formats:
/// - "John 3:16"        -> book=John, chapter=3, verse=16
/// - "John 3:16-18"     -> book=John, chapter=3, verses=16-18
/// - "Genesis 1"        -> book=Genesis, chapter=1 (whole chapter)
/// - "1 Cor 13"         -> book=1 Corinthians, chapter=13
/// - "Ps 23:1-6"        -> book=Psalms, chapter=23, verses=1-6
/// - "jn3:16"           -> book=John, chapter=3, verse=16 (no space)
pub fn parse(input: &str) -> Result<BibleReference, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("Empty reference".to_string());
    }

    // Split into book part and chapter:verse part
    // Handle numbered books like "1 Corinthians" or "1cor"
    let (book_str, rest) = split_book_and_location(input)?;

    let book = books::normalize_book(&book_str)
        .ok_or_else(|| format!("Unknown book: '{}'", book_str))?;

    if rest.is_empty() {
        // Just a book name with no chapter — default to chapter 1
        return Ok(BibleReference {
            book,
            chapter: 1,
            verse_start: None,
            verse_end: None,
        });
    }

    // Parse chapter:verse from the rest
    let (chapter, verse_start, verse_end) = parse_chapter_verse(&rest)?;

    if chapter == 0 || chapter > book.chapters {
        return Err(format!(
            "{} has {} chapters, but chapter {} was requested",
            book.name, book.chapters, chapter
        ));
    }

    Ok(BibleReference {
        book,
        chapter,
        verse_start,
        verse_end,
    })
}

/// Split input into (book_name, chapter_verse_rest).
fn split_book_and_location(input: &str) -> Result<(String, String), String> {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    // Find where the book name ends and the chapter/verse begins.
    // The chapter/verse part starts at the first digit that isn't part of a book number prefix.
    // Book number prefixes: "1", "2", "3" followed by a letter or space.

    let mut i = 0;

    // Skip leading numbered book prefix (e.g., "1 ", "2", "1")
    if i < len && chars[i].is_ascii_digit() && chars[i] != '0' {
        let _digit_start = i;
        i += 1;
        // Check if next char is a letter or space (making this a numbered book)
        if i < len && (chars[i].is_alphabetic() || chars[i] == ' ') {
            // This is a numbered book prefix, skip the space if present
            if chars[i] == ' ' {
                i += 1;
            }
        } else {
            // Not a numbered book — the digits are probably a chapter
            return Ok(("".to_string(), input.to_string()));
        }
    }

    // Now find where the book name ends (first digit after letters)
    let book_start = 0;
    while i < len && (chars[i].is_alphabetic() || chars[i] == ' ' || chars[i] == '.') {
        i += 1;
        // Stop at space if next char is a digit (that's the chapter)
        if i < len && chars[i - 1] == ' ' && i < len && chars[i].is_ascii_digit() {
            // The space before a digit means we've hit the chapter
            i -= 1; // back up to not include the space
            break;
        }
    }

    let book_str = input[book_start..i].trim().to_string();
    let rest = input[i..].trim().to_string();

    if book_str.is_empty() {
        return Err("No book name found".to_string());
    }

    Ok((book_str, rest))
}

/// Parse "3:16", "3:16-18", "3" into (chapter, verse_start, verse_end).
fn parse_chapter_verse(input: &str) -> Result<(u32, Option<u32>, Option<u32>), String> {
    let input = input.trim();

    if let Some((chapter_str, verse_part)) = input.split_once(':') {
        let chapter: u32 = chapter_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid chapter number: '{}'", chapter_str))?;

        if let Some((start_str, end_str)) = verse_part.split_once('-') {
            let start: u32 = start_str
                .trim()
                .parse()
                .map_err(|_| format!("Invalid verse number: '{}'", start_str))?;
            let end: u32 = end_str
                .trim()
                .parse()
                .map_err(|_| format!("Invalid verse number: '{}'", end_str))?;
            Ok((chapter, Some(start), Some(end)))
        } else {
            let verse: u32 = verse_part
                .trim()
                .parse()
                .map_err(|_| format!("Invalid verse number: '{}'", verse_part))?;
            Ok((chapter, Some(verse), Some(verse)))
        }
    } else {
        // Just a chapter number
        let chapter: u32 = input
            .parse()
            .map_err(|_| format!("Invalid chapter number: '{}'", input))?;
        Ok((chapter, None, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_verse() {
        let r = parse("John 3:16").unwrap();
        assert_eq!(r.book.name, "John");
        assert_eq!(r.chapter, 3);
        assert_eq!(r.verse_start, Some(16));
        assert_eq!(r.verse_end, Some(16));
    }

    #[test]
    fn test_verse_range() {
        let r = parse("Psalm 23:1-6").unwrap();
        assert_eq!(r.book.name, "Psalms");
        assert_eq!(r.chapter, 23);
        assert_eq!(r.verse_start, Some(1));
        assert_eq!(r.verse_end, Some(6));
    }

    #[test]
    fn test_whole_chapter() {
        let r = parse("Genesis 1").unwrap();
        assert_eq!(r.book.name, "Genesis");
        assert_eq!(r.chapter, 1);
        assert_eq!(r.verse_start, None);
    }

    #[test]
    fn test_numbered_book() {
        let r = parse("1 Cor 13").unwrap();
        assert_eq!(r.book.name, "1 Corinthians");
        assert_eq!(r.chapter, 13);
    }

    #[test]
    fn test_abbreviation() {
        let r = parse("Jn 3:16").unwrap();
        assert_eq!(r.book.name, "John");
        assert_eq!(r.chapter, 3);
        assert_eq!(r.verse_start, Some(16));
    }

    #[test]
    fn test_abbreviated_numbered_book() {
        let r = parse("1jn 5:3").unwrap();
        assert_eq!(r.book.name, "1 John");
        assert_eq!(r.chapter, 5);
        assert_eq!(r.verse_start, Some(3));
    }

    #[test]
    fn test_display() {
        let r = parse("John 3:16").unwrap();
        assert_eq!(r.display(), "John 3:16");

        let r = parse("Genesis 1").unwrap();
        assert_eq!(r.display(), "Genesis 1");

        let r = parse("Psalm 23:1-6").unwrap();
        assert_eq!(r.display(), "Psalms 23:1-6");
    }

    #[test]
    fn test_invalid_chapter() {
        assert!(parse("Genesis 51").is_err());
    }

    #[test]
    fn test_invalid_book() {
        assert!(parse("Notabook 1:1").is_err());
    }
}
