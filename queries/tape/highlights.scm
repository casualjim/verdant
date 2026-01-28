;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vhs/highlights.scm
;; Licensed under the Apache License 2.0
[
  "Output"
  "Backspace"
  "Down"
  "Enter"
  "Escape"
  "Left"
  "Right"
  "Space"
  "Tab"
  "Up"
  "Set"
  "Type"
  "Sleep"
  "Hide"
  "Show"
] @keyword

[
  "Shell"
  "FontFamily"
  "FontSize"
  "Framerate"
  "PlaybackSpeed"
  "Height"
  "LetterSpacing"
  "TypingSpeed"
  "LineHeight"
  "Padding"
  "Theme"
  "LoopOffset"
  "Width"
  "BorderRadius"
  "Margin"
  "MarginFill"
  "WindowBar"
  "WindowBarSize"
  "CursorBlink"
] @type

"@" @operator

(control) @function.macro

(float) @number.float

(integer) @number

(comment) @comment @spell

[
  (string)
  (json)
] @string

(path) @string.special.path

(time) @string.special
