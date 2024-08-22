use matchpick::process;

const ENTER_PAT: &str = "~>>>";
const EXIT_PAT: &str = "~<<<";
const IGNORE_PAT: &str = "###";
const INPUT: &str = "start
~>>>
default
~>>> eggs
foo
~>>> spam ###
bar
~>>> baz second
foobar
~<<<
end";
const OUTPUT_DEFAULT: &str = "start
default
end";
const OUTPUT_EGGS: &str = "start
foo
end";
const OUTPUT_EGGS_SPAM: &str = "start
foo
~>>> spam ###
bar
end";
const OUTPUT_SPAM: &str = "start
bar
end";
const OUTPUT_BAZ_SECOND: &str = "start
foobar
end";

#[test]
fn case_default() {
    let output = process(
        INPUT,
        None,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    assert_eq!(output, OUTPUT_DEFAULT);
}

#[test]
fn case_eggs() {
    let match_against = Some("eggs".to_owned());
    let output = process(INPUT, match_against, ENTER_PAT, EXIT_PAT, None).unwrap();
    assert_eq!(output, OUTPUT_EGGS);
}

#[test]
fn case_eggs_ignore() {
    let match_against = Some("eggs".to_owned());
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    assert_eq!(output, OUTPUT_EGGS_SPAM);
}

#[test]
fn case_spam() {
    let match_against = Some("spam".to_owned());
    let output = process(INPUT, match_against, ENTER_PAT, EXIT_PAT, None).unwrap();
    assert_eq!(output, OUTPUT_SPAM);
}

#[test]
fn case_spam_ignored() {
    let match_against = Some("spam".to_owned());
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    assert_eq!(output, OUTPUT_DEFAULT);
}

#[test]
fn case_baz() {
    let match_against = Some("baz".to_owned());
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    assert_eq!(output, OUTPUT_BAZ_SECOND);
}

#[test]
fn case_second() {
    let match_against = Some("second".to_owned());
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    assert_eq!(output, OUTPUT_BAZ_SECOND);
}

#[test]
fn case_other() {
    let match_against = Some("something_else_that_will_not_trigger_any_case".to_owned());
    let output = process(
        INPUT,
        match_against,
        ENTER_PAT,
        EXIT_PAT,
        Some(IGNORE_PAT.to_owned()),
    )
    .unwrap();
    assert_eq!(output, OUTPUT_DEFAULT);
}
