use crate::{
    segment_backward_longest,
    segment_forward_longest,
    BackwardDictionary,
    ForwardDictionary,
    BehaviorForUnmatched,
    Match,
};

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_bidirectional_longest<T: AsRef<str>>(
    text: T,
    forward_dict: &ForwardDictionary,
    backward_dict: &BackwardDictionary,
    behavior_for_unmatched: BehaviorForUnmatched,
) -> Vec<Match> {
    let forward_results = segment_forward_longest(
        text.as_ref(),
        forward_dict,
        behavior_for_unmatched,
    );
    let backward_results = segment_backward_longest(
        text.as_ref(),
        backward_dict,
        behavior_for_unmatched,
    );

    if forward_results.len() < backward_results.len() {
        forward_results
    } else if forward_results.len() > backward_results.len() {
        backward_results
    } else {
        let forward_single_chars_count = count_single_chars(
            &forward_results,
            text.as_ref()
        );
        let backward_single_chars_count = count_single_chars(
            &backward_results,
            text.as_ref()
        );

        if forward_single_chars_count < backward_single_chars_count {
            forward_results
        } else {
            backward_results
        }
    }
}

fn count_single_chars<T: AsRef<str>>(
    results: &Vec<Match>,
    text: T
) -> usize {
    let mut result = 0;

    for mat in results {
        if mat.range().extract(text.as_ref()).chars().count() == 1 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{
        segment_bidirectional_longest,
        BackwardDictionary,
        ForwardDictionary,
        BehaviorForUnmatched,
    };

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
        let result = segment_bidirectional_longest(
            text,
            &forward_dict,
            &backward_dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
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
        let result = segment_bidirectional_longest(
            text,
            &forward_dict,
            &backward_dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "服务",]
        )
    }

    #[test]
    fn test_ignore_unmatched() {
        let text = " 商品和服务, hello world ";
        let dict = vec!["商品", "和服", "服务", "你好世界"];
        let forward_dict = ForwardDictionary::new(dict.clone());
        let backward_dict = BackwardDictionary::new(dict.clone());

        let result = segment_bidirectional_longest(
            text,
            &forward_dict,
            &backward_dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "服务",]
        )
    }

    #[test]
    fn test_keep_unmatched_as_chars() {
        let text = " 商品和服务, hello world ";
        let dict = vec!["商品", "和服", "服务", "你好世界"];
        let forward_dict = ForwardDictionary::new(dict.clone());
        let backward_dict = BackwardDictionary::new(dict.clone());

        let result = segment_bidirectional_longest(
            text,
            &forward_dict,
            &backward_dict,
            BehaviorForUnmatched::KeepAsChars,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec![
                " ",
                "商品",
                "和",
                "服务",
                ",",
                " ",
                "h",
                "e",
                "l",
                "l",
                "o",
                " ",
                "w",
                "o",
                "r",
                "l",
                "d",
                " ",
            ]
        )
    }

    #[test]
    fn test_keep_unmatched_as_words() {
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

        let result = segment_bidirectional_longest(
            text,
            &forward_dict,
            &backward_dict,
            BehaviorForUnmatched::KeepAsWords,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec![" ", "当下", "雨天", "地面", "积水", ", hello world "]
        )
    }

    #[test]
    fn test_tf_idf() {
        let text = " 当下雨天地面积水, hello world ";
        let dict: Vec<(&str, f64)> = vec![
            ("当", 0f64),
            ("当下", 1f64),
            ("下雨", 2f64),
            ("下雨天", 3f64),
            ("雨天", 4f64),
            ("地面", 5f64),
            ("积水", 6f64),
            ("你好世界", 7f64),
        ];
        let forward_dict = ForwardDictionary::new_with_tf_idf(dict.clone());
        let backward_dict = BackwardDictionary::new_with_tf_idf(dict.clone());

        // 正向结果: [当下, 雨天, 地面, 积水]
        // 逆向结果: [当, 下雨天, 地面, 积水]
        // 结果数量相同, 单字数量正向结果少于逆向结果, 返回单字数量更少的正向结果.
        let result = segment_bidirectional_longest(
            text,
            &forward_dict,
            &backward_dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.tf_idf().unwrap())
                .collect::<Vec<_>>(),
            vec![1f64, 4f64, 5f64, 6f64]
        )
    }
}
