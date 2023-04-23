use dfa_regex::Regex;

#[test]
fn case01() {
    let regex = Regex::new(r"(p(erl|ython|hp)|ruby)").unwrap();
    assert!(regex.matches("python"));
    assert!(regex.matches("ruby"));
    assert!(regex.matches("perl"));
    assert!(!regex.matches("ruby2"));
    assert!(!regex.matches("java"));
    assert!(!regex.matches("VB"));
}

#[test]
fn case02() {
    let regex = Regex::new(r"山田(太|一|次|三)郎").unwrap();
    assert!(regex.matches("山田太郎"));
    assert!(regex.matches("山田三郎"));
    assert!(!regex.matches("山田郎"));
    assert!(!regex.matches("山田太郎三郎"));
}

#[test]
fn case03() {
    let regex = Regex::new(r"ｗｗ*|\(笑\)").unwrap();
    assert!(regex.matches("(笑)"));
    assert!(regex.matches("ｗｗｗ"));
    assert!(!regex.matches("笑"));
    assert!(!regex.matches("ww"));
}

#[test]
fn case04() {
    let regex = Regex::new(r"a\c").unwrap();
    assert!(regex.matches(r"ac"));
    assert!(!regex.matches(r"a\c"));
}

#[test]
fn case05() {
    let regex = Regex::new(r"a\\c").unwrap();
    assert!(regex.matches(r"a\c"));
    assert!(!regex.matches(r"ac"));
}

#[test]
fn case06() {
    let regex = Regex::new(r"a(b|)").unwrap();
    assert!(regex.matches(r"ab"));
    assert!(regex.matches(r"a"));
    assert!(!regex.matches(r"abb"));
}

#[test]
fn case07() {
    let regex = Regex::new(r"(ab|ba)+").unwrap();
    assert!(regex.matches(r"ab"));
    assert!(regex.matches(r"baabba"));
    assert!(!regex.matches(r"babab"));
    assert!(!regex.matches(r""));
    assert!(!regex.matches(r"b"));
}

#[test]
fn case08() {
    let regex = Regex::new(r"qwertyuiopasdfghjklzxcvbnm").unwrap();
    assert!(regex.matches(r"qwertyuiopasdfghjklzxcvbnm"));
    assert!(!regex.matches(r"qwertyuiopasdfghjklzxcvbnm "));
    assert!(!regex.matches(r" qwertyuiopasdfghjklzxcvbnm"));
    assert!(!regex.matches(r"qwertyuiopasdfghjklzxcvbn"));
}
