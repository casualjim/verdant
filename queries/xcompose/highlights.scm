;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/xcompose/highlights.scm
;; Licensed under the Apache License 2.0
(keysym) @constant

(
  (keysym) @constant.builtin
  (#eq? @constant.builtin "Multi_key")
)

(text) @string

"include" @keyword.import

[
  (octal)
  (hex)
] @number

[
  (modifier)
  "None"
] @keyword.modifier

[
  "%L"
  "%H"
  "%S"
] @string.special

[
  "!"
  "~"
] @operator

[
  ":"
  "<"
  ">"
  "\""
] @punctuation.delimiter

(comment) @comment @spell
