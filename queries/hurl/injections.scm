;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/hurl/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

; injections.scm
(
  (json_value) @injection.content
  (#set! injection.language "json")
)

(
  (xml) @injection.content
  (#set! injection.language "xml")
)

(multiline_string
  (multiline_string_type) @injection.language
  (multiline_string_content) @injection.content
  (#set! injection.combined)
)
