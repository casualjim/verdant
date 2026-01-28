;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/pem/highlights.scm
;; Licensed under the Apache License 2.0
[
  "BEGIN"
  "END"
] @keyword

(dashes) @punctuation.delimiter

(label) @label

(data) @none

(comment) @comment @spell
