;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/qmldir/highlights.scm
;; Licensed under the Apache License 2.0
; Preproc
(command
  (identifier) @keyword.directive
)

; Keywords
(keyword) @keyword

; Literals
(number) @number

(float) @number.float

; Variables
[
  (identifier)
  (unit)
] @variable

; Comments
(comment) @comment @spell
