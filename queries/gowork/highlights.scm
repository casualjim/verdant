;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gowork/highlights.scm
;; Licensed under the Apache License 2.0
[
  "replace"
  "go"
  "use"
] @keyword

"=>" @operator

(comment) @comment @spell

[
  (version)
  (go_version)
] @string
