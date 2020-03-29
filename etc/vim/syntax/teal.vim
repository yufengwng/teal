" vim: set et sw=2 sts=2 ts=2:

if version < 600
  syntax clear
elseif exists("b:current_syntax")
  finish
endif

" definitions

"" comments
syn keyword tealTodo contained TODO FIXME XXX NB NOTE
syn region tealCommentLine start="#" end="$" contains=tealTodo,@Spell
syn region tealCommentDoc start="#/" end="$" contains=tealTodo,@Spell
syn region tealCommentInnerDoc start="#|" end="$" contains=tealTodo,@Spell

"" reserved
syn keyword tealReserved mod
syn keyword tealReserved ref
syn keyword tealReserved macro

"" keywords
syn keyword tealKeyword let const
syn keyword tealKeyword and or not
syn keyword tealKeyword end return
syn keyword tealKeyword use pub lib
syn keyword tealKeyword trait impl where
syn keyword tealCastOp  as
syn keyword tealBranch  if elif else match
syn keyword tealLoop    for in loop break continue yield
syn keyword tealKeyword fn gn nextgroup=tealFnName skipwhite skipempty
syn keyword tealTypeAlias alias nextgroup=tealIdent skipwhite skipempty
syn keyword tealStructure struct enum nextgroup=tealIdent skipwhite skipempty

"" builtin types
syn keyword tealType bool byte uint
syn keyword tealType int real str
syn keyword tealType map set vec tup
syn keyword tealType Self
syn keyword tealEnum opt
syn keyword tealVariant some none
syn keyword tealEnum res
syn keyword tealVariant ok err
syn keyword tealQualifier mut own

"" literals
syn keyword tealSelf self
syn keyword tealBool true false
syn match tealIdent "\%([^[:cntrl:][:space:][:punct:][:digit:]]\|_\)\%([^[:cntrl:][:punct:][:space:]]\|_\)*" display contained
syn match tealFnName "\%([^[:cntrl:][:space:][:punct:][:digit:]]\|_\)\%([^[:cntrl:][:punct:][:space:]]\|_\)*" display contained
syn match tealInt    display "\<[0-9][0-9_]*"
syn match tealBinInt display "\<0b[01_]\+"
syn match tealOctInt display "\<0c[0-7_]\+"
syn match tealHexInt display "\<0x[a-fA-F0-9_]\+"
syn match tealReal display "\<[0-9][0-9_]*\%(\.[0-9][0-9_]*\)"
syn match tealReal display "\<[0-9][0-9_]*\%([e][_]\=[-]\=[0-9_]\+\)"
syn match tealReal display "\<[0-9][0-9_]*\%(\.[0-9][0-9_]*\)\%([e][_]\=[-]\=[0-9_]\+\)"
syn match tealStrEscape display contained /\\\([\\nrt"]\)/
syn region tealStr start=+"+ end=+"+ contains=tealStrEscape,@Spell

"" other constructs
syn match tealFnCall "\w\(\w\)*("he=e-1,me=e-1
syn match tealMacro  "\w\(\w\)*!("he=e-1,me=e-1
syn match tealOp     display "\%(+\|-\|*\|\^\|/\|%\||\|=\|!\|<\|>\)=\?"

" highlights
hi def link tealTodo Todo
hi def link tealCommentLine Comment
hi def link tealCommentDoc  SpecialComment
hi def link tealCommentInnerDoc SpecialComment

hi def link tealReserved Error
hi def link tealKeyword  Keyword
hi def link tealCastOp   Operator
hi def link tealBranch   Conditional
hi def link tealLoop     Repeat
hi def link tealTypeAlias Typedef
hi def link tealStructure Structure

hi def link tealType Type
hi def link tealEnum Type
hi def link tealVariant Constant
hi def link tealQualifier Keyword

hi def link tealSelf Constant
hi def link tealBool Boolean
hi def link tealIdent  Identifier
hi def link tealFnName Function
hi def link tealInt    Number
hi def link tealBinInt Number
hi def link tealOctInt Number
hi def link tealHexInt Number
hi def link tealReal   Number
hi def link tealStr    String
hi def link tealStrEscape Special

hi def link tealFnCall Function
hi def link tealMacro  Macro
hi def link tealOp     Operator

syn sync minlines=200
syn sync maxlines=500

let b:current_syntax = "teal"
