;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/prisma/highlights.scm
;; Licensed under the Apache License 2.0
(variable) @variable

[
  "datasource"
  "generator"
  "model"
  "view"
] @keyword

[
  "type"
  "enum"
] @keyword.type

(comment) @comment @spell

(developer_comment) @comment.documentation @spell

[
  (attribute)
  (call_expression)
] @function

(arguments) @property

(column_type) @type

(enumeral) @constant

(column_declaration
  (identifier) @variable
)

(string) @string

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "="
  "@"
] @operator
