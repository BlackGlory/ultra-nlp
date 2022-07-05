# ultra-nlp
## Usage
### Ignore unmatched contents
```rs
use ultra_nlp::{
    segment_fully,
    StandardDictionary,
    BehaviorForUnmatched,
};

let text = " 南京市长江大桥, hello world ";
let dict = StandardDictionary::new(
    vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
);

let result = segment_fully(text, &dict, BehaviorForUnmatched::Ignore);

assert_eq!(
    result
        .iter()
        .map(|x| x.range.extract(text))
        .collect::<Vec<_>>(),
    vec![
      "南京",
      "南京市",
      "市长",
      "长江",
      "大桥",
    ]
);
```

### Keep unmatched contents as chars
```rs
use ultra_nlp::{
    segment_fully,
    StandardDictionary,
    BehaviorForUnmatched,
};

let text = " 南京市长江大桥, hello world ";
let dict =
    StandardDictionary::new(vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]);

let result = segment_fully(text, &dict, BehaviorForUnmatched::KeepAsChars);

assert_eq!(
    result
        .iter()
        .map(|x| x.range.extract(text))
        .collect::<Vec<_>>(),
    vec![
        " ",
        "南京",
        "南京市",
        "市长",
        "长江",
        "大桥",
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
);
```

### Keep unmatched contents as words
```rs
use ultra_nlp::{
    segment_fully,
    StandardDictionary,
    BehaviorForUnmatched,
};

let text = " 南京市长江大桥, hello world ";
let dict =
    StandardDictionary::new(vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]);

let result = segment_fully(text, &dict, BehaviorForUnmatched::KeepAsWords);

assert_eq!(
    result
        .iter()
        .map(|x| x.range.extract(text))
        .collect::<Vec<_>>(),
    vec![
        " ",
        "南京",
        "南京市",
        "市长",
        "长江",
        "大桥",
        ", hello world ",
    ]
);
```
