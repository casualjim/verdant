;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hjson/highlights.scm
;; Licensed under the Apache License 2.0
(true) @boolean

(false) @boolean

(null) @constant.builtin

(number) @number

(pair
  key: (string) @label
)

(pair
  value: (string) @string
)

(array
  (string) @string
)

;  (string_content (escape_sequence) @string.escape)
;  "," @punctuation.delimiter
"[" @punctuation.bracket

"]" @punctuation.bracket

"{" @punctuation.bracket

"}" @punctuation.bracket

(comment) @comment @spell
