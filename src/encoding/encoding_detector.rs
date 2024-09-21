use chardetng::EncodingDetector;
use encoding_rs::Encoding;

// Fonction pour détecter l'encodage d'un texte
pub fn detect_encoding(text: &[u8]) -> &'static Encoding {
    let mut detector = EncodingDetector::new();
    detector.feed(text, true);
    detector.guess(None, true) // Provide two arguments: `None` for language, `true` for completeness
}
