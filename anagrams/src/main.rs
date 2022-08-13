use std::collections::HashSet;

fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let mut sorted_word = word.clone().to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_by(|a, b| a.cmp(b));
    let str_sorted_word = sorted_word.iter().collect::<String>();
    possible_anagrams
        .iter()
        .map(|x| {
            let mut mut_word_in_list = x.to_lowercase().chars().collect::<Vec<char>>();
            mut_word_in_list.sort_by(|a, b| a.cmp(b));
            if *x.to_lowercase() != word.to_lowercase()
                && str_sorted_word == mut_word_in_list.iter().collect::<String>()
            {
                anagrams.insert(*x);
            } else {
                ();
            }
        })
        .collect::<()>();
    anagrams
}

fn main() {
    let word = "listen";

    let inputs = ["enlists", "google", "inlets", "banana"];
    let anagrams = anagrams_for(word, &inputs);
    println!("Hiii {:?}", anagrams);
}
