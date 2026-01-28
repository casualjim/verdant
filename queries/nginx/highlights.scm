;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/nginx/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

(value) @variable

[
  (location_modifier)
  "="
] @operator

[
  (keyword)
  "location"
] @keyword

[
  "if"
  "map"
] @keyword.conditional

(boolean) @boolean

[
  (auto)
  (constant)
  (level)
  (connection_method)
  (var)
  (condition)
] @variable.builtin

[
  (file)
  (mask)
] @string.special.path

[
  (string_literal)
  (quoted_string_literal)
] @string

(directive
  (variable
    (keyword) @variable.parameter
  )
)

(location_route) @string.special

";" @punctuation.delimiter

[
  (numeric_literal)
  (time)
  (size)
  (cpumask)
] @number

[
  "{"
  "}"
] @punctuation.bracket
