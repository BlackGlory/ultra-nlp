use super::{
    ForwardDictionary,
    BackwardDictionary,
    TextRange,
    segment_forward_longest,
    segment_backward_longest,
};

pub fn segment_bidirectional_longest(
    text: &str,
    forward_dict: &ForwardDictionary,
    backward_dict: &BackwardDictionary,
) -> Vec<TextRange> {
    let forward_results = segment_forward_longest(text, forward_dict);
    let backward_results = segment_backward_longest(text, backward_dict);

    if forward_results.len() < backward_results.len() {
        forward_results
    } else if forward_results.len() > backward_results.len() {
        backward_results
    } else {
        let forward_single_chars_count = count_single_chars(&forward_results, text);
        let backward_single_chars_count = count_single_chars(&backward_results, text);

        if forward_single_chars_count < backward_single_chars_count {
            forward_results
        } else {
            backward_results
        }
    }
}

fn count_single_chars(words: &Vec<TextRange>, text: &str) -> usize {
    let mut result = 0;

    for word in words {
        if word.get(&text).chars().count() == 1 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{
        segment_bidirectional_longest,
        ForwardDictionary,
        BackwardDictionary
    };

    #[test]
    fn test() {
        let text = " 商品和服务, hello world ";
        let dict = vec!["商品", "和服", "服务", "你好世界"];
        let forward_dict = ForwardDictionary::new(dict.clone());
        let backward_dict = BackwardDictionary::new(dict.clone());

        let result = segment_bidirectional_longest(
            text,
            &forward_dict,
            &backward_dict
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.get(text))
                .collect::<Vec<_>>(),
            vec![
                "商品",
                "服务"
            ]
        )
    }
}
