;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hoon/highlights.scm
;; Licensed under the Apache License 2.0
(number) @number

(string) @string

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  (coreTerminator)
  (seriesTerminator)
] @punctuation.delimiter

(rune) @operator

(term) @constant

(aura) @constant.builtin

(lineComment) @comment

(boolean) @constant.builtin

(date) @string.special

(mold) @string.special.symbol

(specialIndex) @number

(lark) @operator

(fullContext) @string.special.symbol
