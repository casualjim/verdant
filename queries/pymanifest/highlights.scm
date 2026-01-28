;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/pymanifest/highlights.scm
;; Licensed under the Apache License 2.0
(keyword) @keyword

(dir_sep) @punctuation.delimiter

(glob) @punctuation.special

(linebreak) @character.special

(char_sequence) @string.special

(char_sequence
  [
    "["
    "]"
  ] @punctuation.bracket
)

(char_sequence
  "!" @operator
)

(char_range
  "-" @operator
)

(escaped_char) @string.escape

(comment) @comment @spell
