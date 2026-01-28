;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ungrammar/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

(definition) @keyword

(identifier) @variable

(label_name) @label

(token) @string

[
  "="
  "|"
] @operator

[
  "*"
  "?"
] @keyword.repeat

":" @punctuation.delimiter

[
  "("
  ")"
] @punctuation.bracket
