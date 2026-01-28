;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gosum/highlights.scm
;; Licensed under the Apache License 2.0
[
  "alpha"
  "beta"
  "dev"
  "pre"
  "rc"
  "+incompatible"
] @keyword

(module_path) @string.special.url

(module_version) @string.special

(hash_version) @attribute

(hash) @string.special.symbol

[
  (number)
  (number_with_decimal)
  (hex_number)
] @number

(checksum
  "go.mod" @string
)

[
  ":"
  "."
  "-"
  "/"
] @punctuation.delimiter
