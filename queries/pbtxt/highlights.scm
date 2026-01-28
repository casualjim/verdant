;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/textproto/highlights.scm
;; Licensed under the Apache License 2.0
(string) @string

(field_name) @variable.member

(comment) @comment @spell

(number) @number

; For stuff like "inf" and "-inf".
(scalar_value
  [
    (identifier)
    (signed_identifier)
  ]
) @number

[
  (open_squiggly)
  (close_squiggly)
  (open_square)
  (close_square)
  (open_arrow)
  (close_arrow)
] @punctuation.bracket
