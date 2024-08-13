/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_lowercase().as_bytes().iter()
    .map(|x| {
        if b"AEIOULNRST".to_ascii_lowercase().contains(x) {
            1
        } else if b"DG".to_ascii_lowercase().contains(x) {
            2
        } else if b"BCMP".to_ascii_lowercase().contains(x) {
            3
        } else if b"FHVWY".to_ascii_lowercase().contains(x) {
            4
        } else if b"K".to_ascii_lowercase().contains(x) {
            5
        } else if b"JX".to_ascii_lowercase().contains(x) {
            8
        } else if b"QZ".to_ascii_lowercase().contains(x) {
            10
        } else {
            0
        }
    })
    .sum()
}
