;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/elsa/highlights.scm
;; Licensed under the Apache License 2.0
; Keywords
[
  "eval"
  "let"
] @keyword

; Function
(function) @function

; Method
(method) @function.method

; Parameter
(parameter) @variable.parameter

; Variables
(identifier) @variable

; Operators
[
  "\\"
  "->"
  "="
  (step)
] @operator

; Punctuation
[
  "("
  ")"
] @punctuation.bracket

":" @punctuation.delimiter

; Comments
(comment) @comment @spell
