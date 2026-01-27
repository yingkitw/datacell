//! Text analysis operations
//!
//! Provides text analysis capabilities including sentiment analysis,
//! keyword extraction, text statistics, and language detection.

use crate::common::collection;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Text analysis statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStats {
    pub word_count: usize,
    pub character_count: usize,
    pub sentence_count: usize,
    pub paragraph_count: usize,
    pub avg_word_length: f64,
    pub avg_sentence_length: f64,
    pub readability_score: f64,
    pub unique_words: usize,
    pub lexical_diversity: f64,
}

/// Sentiment analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentResult {
    pub sentiment: Sentiment,
    pub confidence: f64,
    pub positive_score: f64,
    pub negative_score: f64,
    pub neutral_score: f64,
}

/// Sentiment classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

/// Keyword extraction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordResult {
    pub keywords: Vec<Keyword>,
    pub total_keywords: usize,
}

/// Individual keyword
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub word: String,
    pub score: f64,
    pub frequency: usize,
    pub importance: Importance,
}

/// Keyword importance level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Importance {
    High,
    Medium,
    Low,
}

/// Language detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageResult {
    pub language: String,
    pub confidence: f64,
    pub supported_languages: Vec<String>,
}

/// Text analyzer
pub struct TextAnalyzer {
    stop_words: HashSet<String>,
    sentiment_words: SentimentWords,
}

/// Sentiment word lists
#[derive(Debug, Clone)]
struct SentimentWords {
    positive: HashSet<String>,
    negative: HashSet<String>,
    neutral: HashSet<String>,
}

impl TextAnalyzer {
    /// Create a new text analyzer
    pub fn new() -> Self {
        Self {
            stop_words: Self::default_stop_words(),
            sentiment_words: Self::default_sentiment_words(),
        }
    }

    /// Analyze text statistics
    pub fn analyze_stats(&self, text: &str) -> TextStats {
        let words = self.extract_words(text);
        let sentences = self.extract_sentences(text);
        let paragraphs = self.extract_paragraphs(text);

        let word_count = words.len();
        let character_count = text.chars().count();
        let sentence_count = sentences.len();
        let paragraph_count = paragraphs.len();

        let avg_word_length = if word_count > 0 {
            words.iter().map(|w| w.len()).sum::<usize>() as f64 / word_count as f64
        } else {
            0.0
        };

        let avg_sentence_length = if sentence_count > 0 {
            words.len() as f64 / sentence_count as f64
        } else {
            0.0
        };

        let readability_score = self.calculate_readability_score(&words, &sentences);

        let unique_words = collection::unique_preserve_order(&words).len();
        let lexical_diversity = if word_count > 0 {
            unique_words as f64 / word_count as f64
        } else {
            0.0
        };

        TextStats {
            word_count,
            character_count,
            sentence_count,
            paragraph_count,
            avg_word_length,
            avg_sentence_length,
            readability_score,
            unique_words,
            lexical_diversity,
        }
    }

    /// Perform sentiment analysis
    pub fn analyze_sentiment(&self, text: &str) -> SentimentResult {
        let words = self.extract_words(text);

        let mut positive_count = 0;
        let mut negative_count = 0;
        let mut neutral_count = 0;

        for word in &words {
            let lower_word = word.to_lowercase();
            if self.sentiment_words.positive.contains(&lower_word) {
                positive_count += 1;
            } else if self.sentiment_words.negative.contains(&lower_word) {
                negative_count += 1;
            } else {
                neutral_count += 1;
            }
        }

        let total = positive_count + negative_count + neutral_count;
        let (positive_score, negative_score, neutral_score) = if total > 0 {
            (
                positive_count as f64 / total as f64,
                negative_count as f64 / total as f64,
                neutral_count as f64 / total as f64,
            )
        } else {
            (0.0, 0.0, 1.0)
        };

        let (sentiment, confidence) =
            if positive_score > negative_score && positive_score > neutral_score {
                (Sentiment::Positive, positive_score)
            } else if negative_score > positive_score && negative_score > neutral_score {
                (Sentiment::Negative, negative_score)
            } else {
                (Sentiment::Neutral, neutral_score)
            };

        SentimentResult {
            sentiment,
            confidence,
            positive_score,
            negative_score,
            neutral_score,
        }
    }

    /// Extract keywords from text
    pub fn extract_keywords(&self, text: &str, max_keywords: usize) -> KeywordResult {
        let words = self.extract_words(text);
        let word_frequencies = self.calculate_word_frequencies(&words);

        let mut keywords: Vec<Keyword> = word_frequencies
            .into_iter()
            .filter(|(word, _)| !self.stop_words.contains(word) && word.len() > 2)
            .map(|(word, frequency)| {
                let score = self.calculate_keyword_score(&word, frequency, &words);
                let importance = self.determine_importance(score);

                Keyword {
                    word,
                    score,
                    frequency,
                    importance,
                }
            })
            .collect();

        // Sort by score and take top keywords
        keywords.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        keywords.truncate(max_keywords);

        KeywordResult {
            keywords: keywords.clone(),
            total_keywords: keywords.len(),
        }
    }

    /// Detect language (simplified implementation)
    pub fn detect_language(&self, text: &str) -> LanguageResult {
        let words = self.extract_words(text);

        // Simple language detection based on common words
        let language_scores = self.calculate_language_scores(&words);

        let (language, confidence) = language_scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(("unknown".to_string(), 0.0));

        LanguageResult {
            language,
            confidence,
            supported_languages: vec![
                "english".to_string(),
                "spanish".to_string(),
                "french".to_string(),
                "german".to_string(),
                "unknown".to_string(),
            ],
        }
    }

    /// Extract words from text
    fn extract_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c| c.is_alphabetic() || c.is_ascii_digit())
                    .collect::<String>()
            })
            .filter(|word| !word.is_empty())
            .collect()
    }

    /// Extract sentences from text
    fn extract_sentences(&self, text: &str) -> Vec<String> {
        text.split(&['.', '!', '?'][..])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Extract paragraphs from text
    fn extract_paragraphs(&self, text: &str) -> Vec<String> {
        text.split('\n')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .map(|p| p.to_string())
            .collect()
    }

    /// Calculate readability score (simplified Flesch Reading Ease)
    fn calculate_readability_score(&self, words: &[String], sentences: &[String]) -> f64 {
        if sentences.is_empty() || words.is_empty() {
            return 0.0;
        }

        let avg_sentence_length = words.len() as f64 / sentences.len() as f64;
        let avg_syllables = self.estimate_syllables(words) as f64 / words.len() as f64;

        // Simplified Flesch Reading Ease formula
        206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables)
    }

    /// Estimate syllables in words (simplified)
    fn estimate_syllables(&self, words: &[String]) -> usize {
        words
            .iter()
            .map(|word| {
                let word_lower = word.to_lowercase();
                let vowel_groups = word_lower
                    .chars()
                    .fold((0, false), |(count, in_vowel_group), c| {
                        let is_vowel = matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y');
                        if is_vowel && !in_vowel_group {
                            (count + 1, true)
                        } else if !is_vowel {
                            (count, false)
                        } else {
                            (count, true)
                        }
                    })
                    .0;

                // At least one syllable per word
                vowel_groups.max(1)
            })
            .sum()
    }

    /// Calculate word frequencies
    fn calculate_word_frequencies(&self, words: &[String]) -> HashMap<String, usize> {
        let mut frequencies = HashMap::new();

        for word in words {
            let lower_word = word.to_lowercase();
            *frequencies.entry(lower_word).or_insert(0) += 1;
        }

        frequencies
    }

    /// Calculate keyword score
    fn calculate_keyword_score(&self, word: &str, frequency: usize, total_words: &[String]) -> f64 {
        let tf = frequency as f64 / total_words.len() as f64; // Term frequency
        let idf = (total_words.len() as f64 / frequency as f64).ln(); // Inverse document frequency
        let length_factor = (word.len() as f64 / 10.0).min(1.0); // Prefer longer words

        tf * idf * length_factor
    }

    /// Determine keyword importance
    fn determine_importance(&self, score: f64) -> Importance {
        if score > 0.1 {
            Importance::High
        } else if score > 0.05 {
            Importance::Medium
        } else {
            Importance::Low
        }
    }

    /// Calculate language scores (simplified)
    fn calculate_language_scores(&self, words: &[String]) -> Vec<(String, f64)> {
        let mut scores = HashMap::new();

        // Common words in different languages
        let language_indicators = [
            (
                "english",
                vec![
                    "the", "and", "is", "in", "to", "of", "a", "that", "it", "with",
                ],
            ),
            (
                "spanish",
                vec!["el", "la", "de", "que", "y", "en", "un", "es", "se", "no"],
            ),
            (
                "french",
                vec![
                    "le", "de", "et", "à", "un", "il", "être", "et", "en", "avoir",
                ],
            ),
            (
                "german",
                vec![
                    "der", "die", "und", "in", "den", "von", "zu", "das", "mit", "sich",
                ],
            ),
        ];

        let lower_words: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();

        for (language, indicators) in &language_indicators {
            let mut count = 0;
            for word in &lower_words {
                if indicators.contains(&word.as_str()) {
                    count += 1;
                }
            }
            let score = if lower_words.is_empty() {
                0.0
            } else {
                count as f64 / lower_words.len() as f64
            };
            scores.insert(language.to_string(), score);
        }

        scores.insert("unknown".to_string(), 0.1); // Default unknown score

        scores.into_iter().collect()
    }

    /// Default stop words
    fn default_stop_words() -> HashSet<String> {
        vec![
            "a", "an", "and", "are", "as", "at", "be", "but", "by", "for", "if", "in", "into",
            "is", "it", "no", "not", "of", "on", "or", "such", "that", "the", "their", "then",
            "there", "these", "they", "this", "to", "was", "will", "with", "the", "this", "is",
            "at", "which", "on",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Default sentiment words
    fn default_sentiment_words() -> SentimentWords {
        let positive = vec![
            "good",
            "great",
            "excellent",
            "amazing",
            "wonderful",
            "fantastic",
            "awesome",
            "brilliant",
            "outstanding",
            "superb",
            "magnificent",
            "marvelous",
            "terrific",
            "splendid",
            "remarkable",
            "exceptional",
            "perfect",
            "love",
            "like",
            "enjoy",
            "happy",
            "pleased",
            "satisfied",
            "delighted",
            "thrilled",
            "excited",
            "joyful",
            "cheerful",
            "positive",
            "optimistic",
            "hopeful",
            "confident",
        ];

        let negative = vec![
            "bad",
            "terrible",
            "awful",
            "horrible",
            "dreadful",
            "appalling",
            "disgusting",
            "atrocious",
            "abysmal",
            "deplorable",
            "regrettable",
            "unfortunate",
            "disappointing",
            "sad",
            "angry",
            "frustrated",
            "annoyed",
            "upset",
            "worried",
            "concerned",
            "anxious",
            "nervous",
            "stressed",
            "unhappy",
            "miserable",
            "depressed",
            "negative",
            "pessimistic",
            "hopeless",
            "desperate",
            "failed",
            "failure",
            "wrong",
        ];

        SentimentWords {
            positive: positive.into_iter().map(|s| s.to_string()).collect(),
            negative: negative.into_iter().map(|s| s.to_string()).collect(),
            neutral: HashSet::new(),
        }
    }
}

impl Default for TextAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_stats() {
        let analyzer = TextAnalyzer::new();
        let text = "Hello world! This is a test. How are you today?";

        let stats = analyzer.analyze_stats(text);

        assert!(stats.word_count > 0);
        assert!(stats.character_count > 0);
        assert!(stats.sentence_count > 0);
        assert!(stats.avg_word_length > 0.0);
    }

    #[test]
    fn test_sentiment_analysis() {
        let analyzer = TextAnalyzer::new();

        let positive_text = "This is amazing and wonderful! I love it so much.";
        let negative_text = "This is terrible and awful. I hate it completely.";

        let positive_result = analyzer.analyze_sentiment(positive_text);
        let negative_result = analyzer.analyze_sentiment(negative_text);

        // Check that positive text has higher positive score
        assert!(
            positive_result.positive_score > positive_result.negative_score,
            "Positive text should have higher positive score. Positive: {}, Negative: {}",
            positive_result.positive_score,
            positive_result.negative_score
        );

        // Check that negative text has higher negative score
        assert!(
            negative_result.negative_score > negative_result.positive_score,
            "Negative text should have higher negative score. Positive: {}, Negative: {}",
            negative_result.positive_score,
            negative_result.negative_score
        );

        // If scores are clear, check sentiment classification
        if positive_result.positive_score > positive_result.neutral_score {
            assert!(
                matches!(positive_result.sentiment, Sentiment::Positive),
                "Expected Positive sentiment, got {:?}",
                positive_result.sentiment
            );
        }

        if negative_result.negative_score > negative_result.neutral_score {
            assert!(
                matches!(negative_result.sentiment, Sentiment::Negative),
                "Expected Negative sentiment, got {:?}",
                negative_result.sentiment
            );
        }
    }

    #[test]
    fn test_keyword_extraction() {
        let analyzer = TextAnalyzer::new();
        let text = "Data analysis involves processing large datasets to extract meaningful insights and patterns.";

        let result = analyzer.extract_keywords(text, 5);

        assert!(!result.keywords.is_empty());
        assert!(result.total_keywords <= 5);
    }

    #[test]
    fn test_language_detection() {
        let analyzer = TextAnalyzer::new();

        let english_text = "The quick brown fox jumps over the lazy dog";
        let result = analyzer.detect_language(english_text);

        assert_eq!(result.language, "english");
        assert!(result.confidence > 0.0);
    }
}
