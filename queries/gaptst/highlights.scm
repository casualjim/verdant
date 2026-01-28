;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gaptst/highlights.scm
;; Licensed under the Apache License 2.0
(output_line) @markup.raw.block

[
  "#@local"
  "#@exec"
] @keyword

[
  "gap> "
  "> "
] @keyword.debug

[
  "#@if"
  "#@else"
  "#@fi"
] @keyword.conditional

(comment) @comment @spell
