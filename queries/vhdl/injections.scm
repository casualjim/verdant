;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vhdl/injections.scm
;; Licensed under the Apache License 2.0
(line_comment
  (comment_content) @injection.content
  (#set! injection.language "comment")
)

(block_comment
  (comment_content) @injection.content
  (#set! injection.language "comment")
)
