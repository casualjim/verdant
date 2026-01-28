;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/forth/highlights.scm
;; Licensed under the Apache License 2.0
(core) @function

(operator) @operator

(word) @variable

(
  (word) @constant
  (#lua-match? @constant "^[A-Z_]+$")
)

(number) @number

(string) @string

[
  (start_definition)
  (end_definition)
] @punctuation.delimiter

(comment) @comment @spell
