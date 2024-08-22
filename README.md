```
Find and replace multi-lines using a match-case

Usage: matchpick [OPTIONS] [FILE]

Arguments:
  [FILE]  Read from file (otherwise from stdin)

Options:
  -m, --match <MATCH_AGAINST>            Match against (switch case)
  -s, --start-pattern <START_PATTERN>    Pattern to start matching and switch cases [default: ~>>>]
  -e, --end-pattern <END_PATTERN>        Pattern to end matching [default: ~<<<]
      --ignore-pattern <IGNORE_PATTERN>  Pattern to ignore other patterns
  -o, --output <OUTPUT>                  Output to file (otherwise to stdout)
      --print-start                      Print default starting pattern
      --print-end                        Print default ending pattern
  -h, --help                             Print help
  -V, --version                          Print version
```

As an example, the input:
```
start
~>>>
default output
~>>> eggs
foo, foo
~>>> spam
foo? bar!
foo... baz.
~<<<
end
```
Produces with default arguments:
```
start
default output
end
```
Produces with `--match eggs`:
```
start
foo, foo
end
```
Produces with `--match spam`:
```
start
foo? bar!
foo... baz.
end
```
Produces with `--match something_else`:
```
start
default output
end
```
