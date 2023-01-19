use crate::{
    BehaviorForUnmatched,
    Match,
};
use crate::hashmap::{
    segment_backward_longest,
    segment_forward_longest,
    Dictionary,
};

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_bidirectional_longest<T: AsRef<str>>(
    text: T,
    dict: &Dictionary,
    behavior_for_unmatched: BehaviorForUnmatched,
) -> Vec<Match> {
    let forward_results = segment_forward_longest(
        &text,
        dict,
        behavior_for_unmatched,
    );
    let backward_results = segment_backward_longest(
        &text,
        dict,
        behavior_for_unmatched,
    );

    if forward_results.len() < backward_results.len() {
        forward_results
    } else if forward_results.len() > backward_results.len() {
        backward_results
    } else {
        let forward_single_chars_count = count_single_chars(
            &forward_results,
            &text,
        );
        let backward_single_chars_count = count_single_chars(
            &backward_results,
            &text,
        );

        if forward_single_chars_count < backward_single_chars_count {
            forward_results
        } else {
            backward_results
        }
    }
}

fn count_single_chars<T: AsRef<str>>(matches: &Vec<Match>, text: T) -> usize {
    matches
        .into_iter()
        .map(|mat | {
            if mat.range().extract(text.as_ref()).chars().count() == 1 {
                1
            } else {
                0
            }
        })
        .fold(0, |acc, cur| acc + cur)
}

#[cfg(test)]
mod tests {
    use crate::BehaviorForUnmatched;
    use crate::hashmap::{
        segment_bidirectional_longest,
        Dictionary,
    };

    #[test]
    fn test_should_returns_forward_longest_results() {
        let text = " 当下雨天地面积水, hello world ";
        let patterns = vec![
            "当",
            "当下",
            "下雨",
            "下雨天",
            "雨天",
            "地面",
            "积水",
            "你好世界",
        ];
        let dict = Dictionary::new(patterns.clone()).unwrap();

        // 正向结果: [当下, 雨天, 地面, 积水]
        // 逆向结果: [当, 下雨天, 地面, 积水]
        // 结果数量相同, 单字数量正向结果少于逆向结果, 返回单字数量更少的正向结果.
        let result = segment_bidirectional_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["当下", "雨天", "地面", "积水"]
        )
    }

    #[test]
    fn test_should_returns_backward_longest_results() {
        let text = " 商品和服务, hello world ";
        let patterns = vec!["商品", "和服", "服务", "你好世界"];
        let dict = Dictionary::new(patterns.clone()).unwrap();

        // 正向结果: [商品, 和服]
        // 逆向结果: [商品, 服务]
        // 结果数量相同, 单字数量也相同, 返回逆向结果.
        let result = segment_bidirectional_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "服务",]
        )
    }

    #[test]
    fn test_ignore_unmatched() {
        let text = " 商品和服务, hello world ";
        let patterns = vec!["商品", "和服", "服务", "你好世界"];
        let dict = Dictionary::new(patterns.clone()).unwrap();

        let result = segment_bidirectional_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "服务",]
        )
    }

    #[test]
    fn test_keep_unmatched_as_chars() {
        let text = " 商品和服务, hello world ";
        let patterns = vec!["商品", "和服", "服务", "你好世界"];
        let dict = Dictionary::new(patterns.clone()).unwrap();

        let result = segment_bidirectional_longest(
            text,
            &dict,
            BehaviorForUnmatched::KeepAsChars,
        );

        assert_eq!(
            result
                .into_iter()
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
        let patterns = vec![
            "当",
            "当下",
            "下雨",
            "下雨天",
            "雨天",
            "地面",
            "积水",
            "你好世界",
        ];
        let dict = Dictionary::new(patterns.clone()).unwrap();

        let result = segment_bidirectional_longest(
            text,
            &dict,
            BehaviorForUnmatched::KeepAsWords,
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec![" ", "当下", "雨天", "地面", "积水", ", hello world "]
        )
    }

    #[test]
    fn test_value() {
        let text = " 当下雨天地面积水, hello world ";
        let patterns: Vec<&str> = vec![
            "当",
            "当下",
            "下雨",
            "下雨天",
            "雨天",
            "地面",
            "积水",
            "你好世界",
        ];
        let dict = Dictionary::new(patterns.clone()).unwrap();

        // 正向结果: [当下, 雨天, 地面, 积水]
        // 逆向结果: [当, 下雨天, 地面, 积水]
        // 结果数量相同, 单字数量正向结果少于逆向结果, 返回单字数量更少的正向结果.
        let result = segment_bidirectional_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore,
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.index_of_patterns().unwrap())
                .collect::<Vec<_>>(),
            vec![1, 4, 5, 6]
        )
    }

    #[test]
    fn test_chars_on_edge() {
        let text = "你好世界";
        let patterns = vec!["你好", "世界"];
        let dict = Dictionary::new(patterns).unwrap();

        let result = segment_bidirectional_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["你好", "世界"]
        );
    }
}
