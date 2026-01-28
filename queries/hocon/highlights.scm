;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hocon/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

(null) @constant.builtin

[
  (true)
  (false)
] @boolean

(number) @number

(unit) @keyword

(string) @string

(multiline_string) @string

(string
  (escape_sequence) @string.escape
)

(unquoted_string) @string

[
  "url"
  "file"
  "classpath"
  "required"
] @keyword

(include
  "include" @keyword.import
)

(substitution
  [
    "${"
    "${?"
    "}"
  ] @punctuation.special
)

(substitution
  (_) @variable.member
)

(path
  (_) @variable.member
)

(value
  [
    ":"
    "="
    "+="
  ] @operator
)

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

"," @punctuation.delimiter

(unquoted_path
  "." @punctuation.delimiter
)
