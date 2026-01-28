;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/blade/highlights.scm
;; Licensed under the Apache License 2.0
; inherits: html
[
  (directive)
  (directive_start)
  (directive_end)
] @tag

[
  (php_tag)
  (php_end_tag)
  "{{"
  "}}"
  "{!!"
  "!!}"
  "("
  ")"
] @punctuation.bracket
