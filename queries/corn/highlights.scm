;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/corn/highlights.scm
;; Licensed under the Apache License 2.0
[
  "let"
  "in"
] @keyword

[
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

"." @punctuation.delimiter

[
  ".."
  "="
] @operator

(input) @constant

(null) @constant.builtin

(comment) @comment @spell

(string) @string

(integer) @number

(float) @number.float

(float
  "." @number.float
)

(boolean) @boolean

(path_seg) @property
