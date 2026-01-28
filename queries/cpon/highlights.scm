;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/cpon/highlights.scm
;; Licensed under the Apache License 2.0
; Literals
(string) @string

(escape_sequence) @string.escape

(hex_blob
  "x" @character.special
  (_) @string
)

(esc_blob
  "b" @character.special
  (_) @string
)

(datetime
  "d" @character.special
  (_) @string.special
)

(_
  key: (_) @property
)

(number) @number

(float) @number.float

(boolean) @boolean

(null) @constant.builtin

; Punctuation
[
  ","
  ":"
] @punctuation.delimiter

[
  "{"
  "}"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  "<"
  ">"
] @punctuation.bracket

("\""
  @string
  (#set! conceal "")
)

; Comments
(comment) @comment @spell
