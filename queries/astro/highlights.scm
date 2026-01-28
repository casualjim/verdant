;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/astro/highlights.scm
;; Licensed under the Apache License 2.0
; inherits: html_tags
(doctype) @constant

"<!" @tag.delimiter

"---" @punctuation.delimiter

[
  "{"
  "}"
] @punctuation.special

; custom components get `@type` highlighting
(
  (start_tag
    (tag_name) @type
  )
  (#lua-match? @type "^[A-Z]")
)

(
  (end_tag
    (tag_name) @type
  )
  (#lua-match? @type "^[A-Z]")
)

(
  (self_closing_tag
    (tag_name) @type
  )
  (#lua-match? @type "^[A-Z]")
)

(
  (erroneous_end_tag
    (erroneous_end_tag_name) @type
  )
  (#lua-match? @type "^[A-Z]")
)
