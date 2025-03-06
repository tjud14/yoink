pub fn is_text(data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }

    let text_chars = data.iter().take(512).filter(|&&b| {
        b != 0 && (b >= 32 || b == b'\n' || b == b'\r' || b == b'\t')
    }).count();

    (text_chars as f32 / data.len().min(512) as f32) > 0.9
}