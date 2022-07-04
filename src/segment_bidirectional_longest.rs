use super::{
    segment_backward_longest, segment_forward_longest, BackwardDictionary, ForwardDictionary, Match,
};

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_bidirectional_longest(
    text: &str,
    forward_dict: &ForwardDictionary,
    backward_dict: &BackwardDictionary,
    ignore_unmatched_chars: bool,
) -> Vec<Match> {
    let forward_results = segment_forward_longest(text, forward_dict, ignore_unmatched_chars);
    let backward_results = segment_backward_longest(text, backward_dict, ignore_unmatched_chars);

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

fn count_single_chars(results: &Vec<Match>, text: &str) -> usize {
    let mut result = 0;

    for mat in results {
        if mat.range.extract(&text).chars().count() == 1 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{segment_bidirectional_longest, BackwardDictionary, ForwardDictionary};

    #[test]
    fn test_should_returns_forward_longest_results() {
        let text = " 当下雨天地面积水, hello world ";
        let dict = vec![
            "当",
            "当下",
            "下雨",
            "下雨天",
            "雨天",
            "地面",
            "积水",
            "你好世界",
        ];
        let forward_dict = ForwardDictionary::new(dict.clone());
        let backward_dict = BackwardDictionary::new(dict.clone());

        // 正向结果: [当下, 雨天, 地面, 积水]
        // 逆向结果: [当, 下雨天, 地面, 积水]
        // 结果数量相同, 单字数量正向结果少于逆向结果, 返回单字数量更少的正向结果.
        let result = segment_bidirectional_longest(text, &forward_dict, &backward_dict, true);

        assert_eq!(
            result
                .iter()
                .map(|x| x.range.extract(text))
                .collect::<Vec<_>>(),
            vec!["当下", "雨天", "地面", "积水"]
        )
    }

    #[test]
    fn test_should_returns_backward_longest_results() {
        let text = " 商品和服务, hello world ";
        let dict = vec!["商品", "和服", "服务", "你好世界"];
        let forward_dict = ForwardDictionary::new(dict.clone());
        let backward_dict = BackwardDictionary::new(dict.clone());

        // 正向结果: [商品, 和服]
        // 逆向结果: [商品, 服务]
        // 结果数量相同, 单字数量也相同, 返回逆向结果.
        let result = segment_bidirectional_longest(text, &forward_dict, &backward_dict, true);

        assert_eq!(
            result
                .iter()
                .map(|x| x.range.extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "服务",]
        )
    }

    #[test]
    fn test_ignore_unmatched_chars() {
        let text = " 商品和服务, hello world ";
        let dict = vec!["商品", "和服", "服务", "你好世界"];
        let forward_dict = ForwardDictionary::new(dict.clone());
        let backward_dict = BackwardDictionary::new(dict.clone());

        let result = segment_bidirectional_longest(text, &forward_dict, &backward_dict, true);

        assert_eq!(
            result
                .iter()
                .map(|x| x.range.extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "服务",]
        )
    }

    #[test]
    fn test_keep_unmatched_chars() {
        let text = " 商品和服务, hello world ";
        let dict = vec!["商品", "和服", "服务", "你好世界"];
        let forward_dict = ForwardDictionary::new(dict.clone());
        let backward_dict = BackwardDictionary::new(dict.clone());

        let result = segment_bidirectional_longest(text, &forward_dict, &backward_dict, false);

        assert_eq!(
            result
                .iter()
                .map(|x| x.range.extract(text))
                .collect::<Vec<_>>(),
            vec![
                " ", "商品", "和", "服务", ",", " ", "h", "e", "l", "l", "o", " ", "w", "o", "r",
                "l", "d", " ",
            ]
        )
    }
}
