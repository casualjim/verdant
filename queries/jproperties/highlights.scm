;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/properties/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

(key) @property

(value) @string

(value
  (escape) @string.escape
)

(
  (value) @boolean
  (#any-of? @boolean "true" "false")
)

(
  (value) @number
  (#lua-match? @number "^%d+$")
)

(
  (index) @number
  (#lua-match? @number "^%d+$")
)

(
  (substitution
    (key) @constant
  )
  (#lua-match? @constant "^[A-Z_][A-Z0-9_]*$")
)

(substitution
  (key) @function
  "::" @punctuation.special
  (secret) @constant.macro
)

(property
  [
    "="
    ":"
  ] @operator
)

[
  "${"
  "}"
] @punctuation.special

(substitution
  ":" @punctuation.special
)

[
  "["
  "]"
] @punctuation.bracket

[
  "."
  "\\"
] @punctuation.delimiter
