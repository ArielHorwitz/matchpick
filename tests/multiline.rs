use matchpick::process;

const ENTER_PAT: &str = "~>>>";
const EXIT_PAT: &str = "~<<<";
const IGNORE_PAT: &str = "###";
const INPUT: &str = include_str!("example.txt");

fn check_output(resulted: &str, expected: &str) {
    assert_eq!(resulted, &format!("start\n{expected}\nend"));
}

#[test]
fn case_default() {
    let output = process(
        INPUT,
        Vec::new(),
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    check_output(&output, "default");
}

#[test]
fn case_eggs() {
    let match_against = vec!["eggs".to_owned()];
    let output = process(INPUT, match_against, ENTER_PAT, EXIT_PAT, None).unwrap();
    check_output(&output, "foo");
}

#[test]
fn case_eggs_second() {
    let match_against = vec!["second".to_owned(), "eggs".to_owned()];
    let output = process(INPUT, match_against, ENTER_PAT, EXIT_PAT, None).unwrap();
    check_output(&output, "foo");
}

#[test]
fn case_eggs_ignore() {
    let match_against = vec!["eggs".to_owned()];
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    check_output(&output, "foo\n~>>> spam ###\nbar");
}

#[test]
fn case_spam() {
    let match_against = vec!["spam".to_owned()];
    let output = process(INPUT, match_against, ENTER_PAT, EXIT_PAT, None).unwrap();
    check_output(&output, "bar");
}

#[test]
fn case_spam_ignored() {
    let match_against = vec!["spam".to_owned()];
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    check_output(&output, "default");
}

#[test]
fn case_baz() {
    let match_against = vec!["baz".to_owned()];
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    check_output(&output, "foobar");
}

#[test]
fn case_second() {
    let match_against = vec!["second".to_owned()];
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    check_output(&output, "foobar");
}

#[test]
fn case_spam_second_ignore() {
    let match_against = vec!["spam".to_owned(), "second".to_owned()];
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    check_output(&output, "foobar");
}

#[test]
fn case_other() {
    let match_against = vec!["something_else_that_will_not_trigger_any_case".to_owned()];
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    check_output(&output, "default");
}
