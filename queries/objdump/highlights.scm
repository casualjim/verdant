;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/objdump/highlights.scm
;; Licensed under the Apache License 2.0
(byte) @constant

[
  (hexadecimal)
  (integer)
  (address)
] @number

; [
;   "file" "format"
;   "File" "Offset:"
;   "discriminator"
; ] @none
"Disassembly of section " @markup.heading

(section_address) @string.special

(identifier) @variable

(code_location
  (identifier) @function.call
)

(header
  (identifier) @keyword
)

(disassembly_section_label
  (identifier) @module
)

(disassembly_section
  (identifier) @module
)

[
  (file_offset)
  (discriminator)
] @variable.member

(file_path) @string.special.path

(instruction) @function

(bad_instruction) @comment.warning

(label) @label

[
  "<"
  ">"
] @punctuation.special

[
  "("
  ")"
] @punctuation.bracket

[
  "+"
  ":"
] @punctuation.delimiter

(comment) @comment.documentation
