# ultra-nlp
## Usage
### ngrams
```rs
let text = "你好世界";

let result = ngrams(text, 2);

assert_eq!(
    result
        .into_iter()
        .collect::<Vec<String>>(),
    vec!["你好", "好世", "世界"]
);
```

### extract_consecutive_chinese_chars
```rs
let text = "foo中文bar字符baz";

let result = extract_consecutive_chinese_chars(text);

assert_eq!(result, vec!["中文", "字符"]);
```

### cedarwood(slow, low memory usage)
#### Ingore unmatched contents
```rs
use ultra_nlp::BehaviorForUnmatched,
use ultra_nlp::cedarwood::{
    segment_fully,
    ForwardDictionary,
};

let text = " 南京市长江大桥, hello world ";
let dict = ForwardDictionary::new(
    vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
).unwrap();

let result = segment_fully(
    text,
    &dict,
    BehaviorForUnmatched::Ignore
);

assert_eq!(
    result
        .iter()
        .map(|x| x.range().extract(text))
        .collect::<Vec<_>>(),
    vec!["南京", "南京市", "市长", "长江", "大桥"]
);
```

#### Keep unmatched contents as chars
```rs
use ultra_nlp::BehaviorForUnmatched,
use ultra_nlp::cedarwood::{
    segment_fully,
    ForwardDictionary,
};

let text = " 南京市长江大桥, hello world ";
let dict = ForwardDictionary::new(
    vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
).unwrap();

let result = segment_fully(
    text,
    &dict,
    BehaviorForUnmatched::KeepAsChars
);

assert_eq!(
    result
        .iter()
        .map(|x| x.range().extract(text))
        .collect::<Vec<_>>(),
    vec![
        " ", "南京", "南京市", "市长", "长江", "大桥", ",", " ", "h", "e", "l", "l", "o", " ", "w", "o", "r", "l", "d", " ",
    ]
);
```

#### Keep unmatched ocntents as words
```rs
use ultra_nlp::BehaviorForUnmatched,
use ultra_nlp::cedarwood::{
    segment_fully,
    ForwardDictionary,
};

let text = " 南京市长江大桥, hello world ";
let dict = ForwardDictionary::new(
    vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
).unwrap();

let result = segment_fully(
    text,
    &dict,
    BehaviorForUnmatched::KeepAsWords
);

assert_eq!(
    result
        .iter()
        .map(|x| x.range().extract(text))
        .collect::<Vec<_>>(),
    vec![
        " ", "南京", "南京市", "市长", "长江", "大桥", ", hello world ",
    ]
);
```

### daachorse(fast, high memory usage)
#### Ignore unmatched contents
```rs
use ultra_nlp::BehaviorForUnmatched,
use ultra_nlp::daachorse::{
    segment_fully,
    StandardDictionary,
};

let text = " 南京市长江大桥, hello world ";
let dict = StandardDictionary::new(
    vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
).unwrap();

let result = segment_fully(text, &dict, BehaviorForUnmatched::Ignore);

assert_eq!(
    result
        .iter()
        .map(|x| x.range().extract(text))
        .collect::<Vec<_>>(),
    vec![
      "南京", "南京市", "市长", "长江", "大桥",
    ]
);
```

#### Keep unmatched contents as chars
```rs
use ultra_nlp::BehaviorForUnmatched,
use ultra_nlp::daachorse::{
    segment_fully,
    StandardDictionary,
};

let text = " 南京市长江大桥, hello world ";
let dict = StandardDictionary::new(
    vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
).unwrap();

let result = segment_fully(text, &dict, BehaviorForUnmatched::KeepAsChars);

assert_eq!(
    result
        .iter()
        .map(|x| x.range().extract(text))
        .collect::<Vec<_>>(),
    vec![
        " ", "南京", "南京市", "市长", "长江", "大桥", ",", " ", "h", "e", "l", "l", "o", " ", "w", "o", "r", "l", "d", " ",
    ]
);
```

#### Keep unmatched contents as words
```rs
use ultra_nlp::BehaviorForUnmatched,
use ultra_nlp::daachorse::{
    segment_fully,
    StandardDictionary,
};

let text = " 南京市长江大桥, hello world ";
let dict = StandardDictionary::new(
    vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
).unwrap();

let result = segment_fully(text, &dict, BehaviorForUnmatched::KeepAsWords);

assert_eq!(
    result
        .iter()
        .map(|x| x.range().extract(text))
        .collect::<Vec<_>>(),
    vec![
        " ", "南京", "南京市", "市长", "长江", "大桥", ", hello world ",
    ]
);
```
