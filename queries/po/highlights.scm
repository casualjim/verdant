;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/po/highlights.scm
;; Licensed under the Apache License 2.0
; Keywords
[
  "msgctxt"
  "msgid"
  "msgid_plural"
  "msgstr"
  "msgstr_plural"
] @keyword

; Punctuation
[
  "["
  "]"
] @punctuation.bracket

; Literals
(string) @string

(escape_sequence) @string.escape

(number) @number

; Comments
(comment) @comment @spell

(comment
  (reference
    (text) @string.special.path
  )
)

(comment
  (flag
    (text) @keyword.directive
  )
)
