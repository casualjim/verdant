;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/disassembly/highlights.scm
;; Licensed under the Apache License 2.0
(byte) @constant

[
  (address)
  (hexadecimal)
  (integer)
] @number

(identifier) @variable

(bad_instruction) @comment.warning

(code_location
  (identifier) @function.call
)

(comment) @comment

(instruction) @function

(memory_dump) @string

[
  "<"
  ">"
] @punctuation.special

[
  "+"
  ":"
] @punctuation.delimiter
