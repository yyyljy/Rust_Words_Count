use std::collections::HashMap;

fn main() {
    // PDF 파일 경로
    let file_path = "./src/Avalanche_Platform_Whitepaper.pdf";

    let bytes = std::fs::read(file_path).unwrap();

    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    
    // 단어별 카운트
    let word_counts = count_words(&out);
    print_sort_by_value_desc(&word_counts);

}

fn remove_special_chars(word: &str) -> String {
    let mut cleaned_word = String::new();

    for c in word.chars() {
        if c.is_alphanumeric() || c.is_whitespace() {
            cleaned_word.push(c);
        }
    }
    cleaned_word
}

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    let blacklists =  vec![
        "","a","an","any","are","and","all","at","by","be","as","each","high",
        "can","for","may","not", "has","in","its","is","it",
        "other", "or","one","of","on","such","set",
        "this","to", "that","their","they","the","time","we","with","which","will"];
    // 단어 추출 및 카운트
    for word in text.split_whitespace() {
        let cword = remove_special_chars(word);
        if !blacklists.iter().any(|&w| w == cword.to_lowercase()) {
            // println!("{}", cword.to_lowercase());
            *word_counts.entry(cword.to_lowercase()).or_insert(0) += 1;
        }
    }
    word_counts.retain(|_, &mut count| count >= 15);
    word_counts
}

fn print_sort_by_value_desc(map: &HashMap<String, u32>) {
    let mut sorted_vec: Vec<(&String, &u32)> = map.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (word, count) in sorted_vec {
        println!("{}:{}",word,count);
    }
}