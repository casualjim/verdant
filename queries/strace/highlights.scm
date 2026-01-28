;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/strace/highlights.scm
;; Licensed under the Apache License 2.0
[
  "killed"
  "by"
  "exited"
  "with"
  "<unfinished ...>"
  "<..."
  "resumed>"
] @keyword

[
  (errorName)
  (errorDescription)
] @keyword.exception

(syscall) @function.builtin

; Literals
[
  (integer)
  (pointer)
] @number

(value) @label

(string) @string

[
  "="
  "|"
  "*"
  "&&"
  "=="
] @operator

; Punctuation
[
  "+++"
  "---"
  "..."
  "~"
] @punctuation.special

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  ","
  "=>"
] @punctuation.delimiter

(comment) @comment @spell
