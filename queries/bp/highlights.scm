;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/bp/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

(operator) @operator

(integer_literal
  "-" @operator
)

[
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(boolean_literal) @boolean

(integer_literal) @number

[
  (raw_string_literal)
  (interpreted_string_literal)
] @string

(escape_sequence) @string.escape

(identifier) @variable

(module
  type: (identifier) @function.call
)

(module
  (property
    field: (identifier) @variable.parameter
  )
)

[
  (unset)
  (default)
  (any)
] @variable.builtin

(condition
  name: (identifier) @function.builtin
)

(map_expression
  (property
    field: (identifier) @property
  )
)

(select_expression
  "select" @keyword.conditional
)
