;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/git_rebase/highlights.scm
;; Licensed under the Apache License 2.0
(
  (command) @keyword
  (label)? @constant
  (message)? @none @spell
)

(option) @operator

(comment) @comment
