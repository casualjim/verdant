;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/nqc/highlights.scm
;; Licensed under the Apache License 2.0
; inherits: c
"task" @keyword.function

"until" @keyword.repeat

[
  "acquire"
  "monitor"
  "catch"
  "start"
  "stop"
] @keyword.coroutine
