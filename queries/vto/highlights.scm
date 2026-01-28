;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vento/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

(keyword) @keyword

(tag
  [
    "{{"
    "{{-"
    "}}"
    "-}}"
  ] @punctuation.special
)

"|>" @operator
