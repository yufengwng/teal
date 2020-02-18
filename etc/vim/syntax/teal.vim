" Language: Teal
" Maintainer: Yufeng Wang <yufengwang05@gmail.com>
" URL: https://github.com/yufengwng/teal

if version < 600
  syntax clear
elseif exists("b:current_syntax")
  finish
endif

" definitions

"" comments
syn keyword tealTodo contained TODO FIXME XXX NB NOTE
syn region tealCommentLine start="#" end="$" contains=tealTodo,@Spell
syn region tealCommentDoc start="\~!\?" end="$" contains=tealTodo,@Spell

"" reserved
syn keyword tealReserved break continue struct enum
syn keyword tealReserved use mod pub
syn keyword tealReserved trait impl self
syn keyword tealReserved alias type ref mut as
syn keyword tealReserved const macro where

"" keywords
syn keyword tealKeyword let
syn keyword tealKeyword and or not
syn keyword tealKeyword end return
syn keyword tealBranch  if elif else match
syn keyword tealLoop    for in loop
syn keyword tealKeyword fn nextgroup=tealFnName skipwhite skipempty

"" builtin types
syn keyword tealType bool int uint real str map set vec tup
syn keyword tealEnum opt
syn keyword tealVariant some none
syn keyword tealEnum res
syn keyword tealVariant ok err

"" literals
syn keyword tealBool true false
syn match tealIdent "\%([^[:cntrl:][:space:][:punct:][:digit:]]\|_\)\%([^[:cntrl:][:punct:][:space:]]\|_\)*" display contained
syn match tealFnName "\%([^[:cntrl:][:space:][:punct:][:digit:]]\|_\)\%([^[:cntrl:][:punct:][:space:]]\|_\)*" display contained

" highlights
hi def link tealTodo Todo
hi def link tealCommentLine Comment
hi def link tealCommentDoc  SpecialComment

hi def link tealReserved Error
hi def link tealKeyword  Keyword
hi def link tealBranch   Conditional
hi def link tealLoop     Conditional

hi def link tealType Type
hi def link tealEnum Type
hi def link tealVariant Constant

hi def link tealBool Boolean
hi def link tealIdent  Identifier
hi def link tealFnName Function

syn sync minlines=200
syn sync maxlines=500

let b:current_syntax = "teal"

" vim: set et sw=2 sts=2 ts=2:
