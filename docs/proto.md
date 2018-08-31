# Protocol Documentation
<a name="top"/>

## Table of Contents

- [app.proto](#app.proto)
    - [App](#.App)
    - [AppFile](#.AppFile)
  
  
  
  

- [expr.proto](#expr.proto)
    - [Expr](#.Expr)
  
    - [ExprType](#.ExprType)
  
  
  

- [instr.proto](#instr.proto)
    - [Block](#.Block)
    - [Func](#.Func)
    - [Instr](#.Instr)
  
    - [Opcode](#.Opcode)
  
  
  

- [module.proto](#module.proto)
    - [HObject](#.HObject)
    - [HObject.ImportsEntry](#.HObject.ImportsEntry)
    - [Symbol](#.Symbol)
  
  
  
  

- [prim.proto](#prim.proto)
  
    - [HPrimOp](#.HPrimOp)
  
  
  

- [Scalar Value Types](#scalar-value-types)



<a name="app.proto"/>
<p align="right"><a href="#top">Top</a></p>

## app.proto



<a name=".App"/>

### App
Holon Application


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [fixed64](#fixed64) |  |  |
| version | [fixed64](#fixed64) |  |  |
| name | [string](#string) |  |  |
| cafs | [Expr](#Expr) | repeated | repeated InfoTable info_tables = 9; |






<a name=".AppFile"/>

### AppFile
Holon Application File


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| magic_number | [fixed32](#fixed32) |  |  |
| app | [App](#App) |  |  |





 

 

 

 



<a name="expr.proto"/>
<p align="right"><a href="#top">Top</a></p>

## expr.proto



<a name=".Expr"/>

### Expr



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [ExprType](#ExprType) |  |  |
| p_num | [uint32](#uint32) |  | pointer number |
| tag | [uint32](#uint32) |  | Data constructor tag (optional) |
| payloads | [fixed64](#fixed64) | repeated |  |





 


<a name=".ExprType"/>

### ExprType


| Name | Number | Description |
| ---- | ------ | ----------- |
| INVALID | 0 |  |
| INDIR | 1 | Indirection |
| CONSTR | 2 | Data constructor |
| FUNC | 4 | Function object |
| THUNK_F | 6 | Thunk has a function |
| THUNK_C | 7 | Thunk has a closure |
| PAPPLY | 8 | Partial apply to a function |
| BOTTOM | 9 | ⊥ (error) |


 

 

 



<a name="instr.proto"/>
<p align="right"><a href="#top">Top</a></p>

## instr.proto



<a name=".Block"/>

### Block



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bitmap | [bytes](#bytes) |  | Stack payload layout bitmap |
| code | [Instr](#Instr) | repeated |  |






<a name=".Func"/>

### Func
sr: 2 byte. Stack relative address
st: 8 byte. Static object pointer
cp: 8 byte. Code pointer
cr: 2 byte. Code offset
k:  1 byte. Data constructor kind
po: 1 byte. PrimOp
bo: 2 byte. Block offset


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| arity | [uint32](#uint32) |  |  |
| srt | [fixed64](#fixed64) | repeated | SRT |
| blocks | [Block](#Block) | repeated |  |
| symbol | [string](#string) |  |  |






<a name=".Instr"/>

### Instr



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| opcode | [Opcode](#Opcode) |  |  |
| p_num | [uint32](#uint32) |  | pointer number |
| operands | [uint32](#uint32) | repeated | Actually 16 bit |





 


<a name=".Opcode"/>

### Opcode


| Name | Number | Description |
| ---- | ------ | ----------- |
| NOP | 0 |  |
| JUMP | 2 | Jump to the function (cp) |
| PUSH | 3 | Push (copy) local (sr) |
| RETURN | 4 | Return local (sr) |
| PRIMOP_P | 5 | Primitive operator (po) call |
| PRIMOP_R | 6 |  |
| CONSTR_P | 7 | Make a data cons (k) with arguments (sr)* |
| CONSTR_R | 8 |  |
| FUNC_P | 9 | Make a function (cp) object |
| FUNC_R | 10 |  |
| THUNK_F_P | 11 | Make a thunk with the function (cp) and arguments (sr)* |
| THUNK_F_R | 12 |  |
| THUNK_C_P | 13 | Make a thunk with the closure (sr) and arguments (sr)* |
| THUNK_C_R | 14 |  |
| BOTTOM_P | 15 | Make bottom object |
| BOTTOM_R | 16 |  |
| EVAL | 17 | eval closures (sr)* |
| MATCH_A_E_P | 18 | Eval ADT (sr) and pattern match, jump by (bo)* |
| MATCH_A_E_R | 19 |  |


 

 

 



<a name="module.proto"/>
<p align="right"><a href="#top">Top</a></p>

## module.proto



<a name=".HObject"/>

### HObject
Holon Object File


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| major_version | [int64](#int64) |  |  |
| minor_version | [int64](#int64) |  |  |
| entry_point | [fixed64](#fixed64) |  |  |
| local_offset | [fixed64](#fixed64) |  |  |
| imports | [HObject.ImportsEntry](#HObject.ImportsEntry) | repeated |  |
| caf_exports | [fixed64](#fixed64) | repeated |  |
| code_exports | [fixed64](#fixed64) | repeated |  |
| cafs | [Expr](#Expr) | repeated | fixed64 caf_num = 3; fixed64 table_num = 4; |
| text | [Func](#Func) | repeated |  |






<a name=".HObject.ImportsEntry"/>

### HObject.ImportsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [fixed64](#fixed64) |  |  |
| value | [Symbol](#Symbol) |  |  |






<a name=".Symbol"/>

### Symbol



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package | [int32](#int32) |  |  |
| module | [int32](#int32) |  |  |
| local | [fixed64](#fixed64) |  |  |
| name | [string](#string) |  |  |





 

 

 

 



<a name="prim.proto"/>
<p align="right"><a href="#top">Top</a></p>

## prim.proto


 


<a name=".HPrimOp"/>

### HPrimOp


| Name | Number | Description |
| ---- | ------ | ----------- |
| PNOP | 0 |  |
| INTADD | 8 | (&#43;#) :: Int# -&gt; Int# -&gt; Int# |
| INTSUB | 9 | (-#) :: Int# -&gt; Int# -&gt; Int# |
| INTMUL | 10 | (*#) :: Int# -&gt; Int# -&gt; Int# |
| INTNEG | 19 | negateInt# :: Int# -&gt; Int# |
| INTGT | 22 | (&gt;#) :: Int# -&gt; Int# -&gt; Int# |
| INTGE | 23 | (&gt;=#) :: Int# -&gt; Int# -&gt; Int# |
| INTEQ | 24 | (==#) :: Int# -&gt; Int# -&gt; Int# |
| INTNE | 25 | (/=#) :: Int# -&gt; Int# -&gt; Int# |
| INTLT | 26 | (&lt;#) :: Int# -&gt; Int# -&gt; Int# |
| INTLE | 27 | (&lt;=#) :: Int# -&gt; Int# -&gt; Int# |
| DATATOTAG | 100 | dataToTag# :: a -&gt; Int# |
| TAGTOENUM | 101 | tagToEnum# :: Int# -&gt; a |


 

 

 



## Scalar Value Types

| .proto Type | Notes | C++ Type | Java Type | Python Type |
| ----------- | ----- | -------- | --------- | ----------- |
| <a name="double" /> double |  | double | double | float |
| <a name="float" /> float |  | float | float | float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long |
| <a name="bool" /> bool |  | bool | boolean | boolean |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str |

