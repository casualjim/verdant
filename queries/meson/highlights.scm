;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/meson/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

(number) @number

(bool) @boolean

(identifier) @variable

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

[
  ":"
  ","
  "."
] @punctuation.delimiter

[
  "and"
  "not"
  "or"
  "in"
] @keyword.operator

[
  "="
  "=="
  "!="
  "+"
  "/"
  "/="
  "+="
  "-="
  ">"
  ">="
] @operator

(ternaryoperator
  [
    "?"
    ":"
  ] @keyword.conditional.ternary
)

[
  "if"
  "elif"
  "else"
  "endif"
] @keyword.conditional

[
  "foreach"
  "endforeach"
  (keyword_break)
  (keyword_continue)
] @keyword.repeat

(string) @string

"@" @punctuation.special

(normal_command
  command: (identifier) @function
)

(pair
  key: (identifier) @property
)

(escape_sequence) @string.escape

(
  (identifier) @variable.builtin
  (#any-of? @variable.builtin "meson" "host_machine" "build_machine" "target_machine")
)
