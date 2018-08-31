{- This file was auto-generated from instr.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.Instr
       (Block(..), Func(..), Instr(..), Opcode(..), Opcode(),
        Opcode'UnrecognizedValue)
       where
import qualified Data.ProtoLens.Reexport.Lens.Labels.Prism
       as Lens.Labels.Prism
import qualified Data.ProtoLens.Reexport.Prelude as Prelude
import qualified Data.ProtoLens.Reexport.Data.Int as Data.Int
import qualified Data.ProtoLens.Reexport.Data.Word as Data.Word
import qualified Data.ProtoLens.Reexport.Data.ProtoLens
       as Data.ProtoLens
import qualified
       Data.ProtoLens.Reexport.Data.ProtoLens.Message.Enum
       as Data.ProtoLens.Message.Enum
import qualified
       Data.ProtoLens.Reexport.Data.ProtoLens.Service.Types
       as Data.ProtoLens.Service.Types
import qualified Data.ProtoLens.Reexport.Lens.Family2
       as Lens.Family2
import qualified Data.ProtoLens.Reexport.Lens.Family2.Unchecked
       as Lens.Family2.Unchecked
import qualified Data.ProtoLens.Reexport.Data.Default.Class
       as Data.Default.Class
import qualified Data.ProtoLens.Reexport.Data.Text as Data.Text
import qualified Data.ProtoLens.Reexport.Data.Map as Data.Map
import qualified Data.ProtoLens.Reexport.Data.ByteString
       as Data.ByteString
import qualified Data.ProtoLens.Reexport.Data.ByteString.Char8
       as Data.ByteString.Char8
import qualified Data.ProtoLens.Reexport.Lens.Labels as Lens.Labels
import qualified Data.ProtoLens.Reexport.Text.Read as Text.Read

{- | Fields :

    * 'Proto.Instr_Fields.bitmap' @:: Lens' Block Data.ByteString.ByteString@
    * 'Proto.Instr_Fields.code' @:: Lens' Block [Instr]@
 -}
data Block = Block{_Block'bitmap :: !Data.ByteString.ByteString,
                   _Block'code :: ![Instr],
                   _Block'_unknownFields :: !Data.ProtoLens.FieldSet}
               deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Block x a, a ~ b) =>
         Lens.Labels.HasLens f Block Block x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Block "bitmap" (Data.ByteString.ByteString)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Block'bitmap
                 (\ x__ y__ -> x__{_Block'bitmap = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Block "code" ([Instr])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Block'code
                 (\ x__ y__ -> x__{_Block'code = y__}))
              Prelude.id
instance Data.Default.Class.Default Block where
        def
          = Block{_Block'bitmap = Data.ProtoLens.fieldDefault,
                  _Block'code = [], _Block'_unknownFields = ([])}
instance Data.ProtoLens.Message Block where
        messageName _ = Data.Text.pack "Block"
        fieldsByTag
          = let bitmap__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "bitmap"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.BytesField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.ByteString.ByteString)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "bitmap")))
                      :: Data.ProtoLens.FieldDescriptor Block
                code__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "code"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Instr)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "code")))
                      :: Data.ProtoLens.FieldDescriptor Block
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, bitmap__field_descriptor),
                 (Data.ProtoLens.Tag 2, code__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Block'_unknownFields
              (\ x__ y__ -> x__{_Block'_unknownFields = y__})
{- | Fields :

    * 'Proto.Instr_Fields.arity' @:: Lens' Func Data.Word.Word32@
    * 'Proto.Instr_Fields.srt' @:: Lens' Func [Data.Word.Word64]@
    * 'Proto.Instr_Fields.blocks' @:: Lens' Func [Block]@
    * 'Proto.Instr_Fields.symbol' @:: Lens' Func Data.Text.Text@
 -}
data Func = Func{_Func'arity :: !Data.Word.Word32,
                 _Func'srt :: ![Data.Word.Word64], _Func'blocks :: ![Block],
                 _Func'symbol :: !Data.Text.Text,
                 _Func'_unknownFields :: !Data.ProtoLens.FieldSet}
              deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Func x a, a ~ b) =>
         Lens.Labels.HasLens f Func Func x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Func "arity" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Func'arity
                 (\ x__ y__ -> x__{_Func'arity = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Func "srt" ([Data.Word.Word64])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Func'srt
                 (\ x__ y__ -> x__{_Func'srt = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Func "blocks" ([Block])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Func'blocks
                 (\ x__ y__ -> x__{_Func'blocks = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Func "symbol" (Data.Text.Text)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Func'symbol
                 (\ x__ y__ -> x__{_Func'symbol = y__}))
              Prelude.id
instance Data.Default.Class.Default Func where
        def
          = Func{_Func'arity = Data.ProtoLens.fieldDefault, _Func'srt = [],
                 _Func'blocks = [], _Func'symbol = Data.ProtoLens.fieldDefault,
                 _Func'_unknownFields = ([])}
instance Data.ProtoLens.Message Func where
        messageName _ = Data.Text.pack "Func"
        fieldsByTag
          = let arity__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "arity"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "arity")))
                      :: Data.ProtoLens.FieldDescriptor Func
                srt__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "srt"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Packed
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "srt")))
                      :: Data.ProtoLens.FieldDescriptor Func
                blocks__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "blocks"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Block)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "blocks")))
                      :: Data.ProtoLens.FieldDescriptor Func
                symbol__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "symbol"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.StringField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Text.Text)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "symbol")))
                      :: Data.ProtoLens.FieldDescriptor Func
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, arity__field_descriptor),
                 (Data.ProtoLens.Tag 2, srt__field_descriptor),
                 (Data.ProtoLens.Tag 3, blocks__field_descriptor),
                 (Data.ProtoLens.Tag 4, symbol__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Func'_unknownFields
              (\ x__ y__ -> x__{_Func'_unknownFields = y__})
{- | Fields :

    * 'Proto.Instr_Fields.opcode' @:: Lens' Instr Opcode@
    * 'Proto.Instr_Fields.pNum' @:: Lens' Instr Data.Word.Word32@
    * 'Proto.Instr_Fields.operands' @:: Lens' Instr [Data.Word.Word32]@
 -}
data Instr = Instr{_Instr'opcode :: !Opcode,
                   _Instr'pNum :: !Data.Word.Word32,
                   _Instr'operands :: ![Data.Word.Word32],
                   _Instr'_unknownFields :: !Data.ProtoLens.FieldSet}
               deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Instr x a, a ~ b) =>
         Lens.Labels.HasLens f Instr Instr x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Instr "opcode" (Opcode)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Instr'opcode
                 (\ x__ y__ -> x__{_Instr'opcode = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Instr "pNum" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Instr'pNum
                 (\ x__ y__ -> x__{_Instr'pNum = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Instr "operands" ([Data.Word.Word32])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Instr'operands
                 (\ x__ y__ -> x__{_Instr'operands = y__}))
              Prelude.id
instance Data.Default.Class.Default Instr where
        def
          = Instr{_Instr'opcode = Data.Default.Class.def,
                  _Instr'pNum = Data.ProtoLens.fieldDefault, _Instr'operands = [],
                  _Instr'_unknownFields = ([])}
instance Data.ProtoLens.Message Instr where
        messageName _ = Data.Text.pack "Instr"
        fieldsByTag
          = let opcode__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "opcode"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.EnumField ::
                         Data.ProtoLens.FieldTypeDescriptor Opcode)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "opcode")))
                      :: Data.ProtoLens.FieldDescriptor Instr
                pNum__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "p_num"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "pNum")))
                      :: Data.ProtoLens.FieldDescriptor Instr
                operands__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "operands"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Packed
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "operands")))
                      :: Data.ProtoLens.FieldDescriptor Instr
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, opcode__field_descriptor),
                 (Data.ProtoLens.Tag 2, pNum__field_descriptor),
                 (Data.ProtoLens.Tag 3, operands__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Instr'_unknownFields
              (\ x__ y__ -> x__{_Instr'_unknownFields = y__})
data Opcode = NOP
            | JUMP
            | PUSH
            | RETURN
            | PRIMOP_P
            | PRIMOP_R
            | CONSTR_P
            | CONSTR_R
            | FUNC_P
            | FUNC_R
            | THUNK_F_P
            | THUNK_F_R
            | THUNK_C_P
            | THUNK_C_R
            | BOTTOM_P
            | BOTTOM_R
            | EVAL
            | MATCH_A_E_P
            | MATCH_A_E_R
            | Opcode'Unrecognized !Opcode'UnrecognizedValue
                deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
newtype Opcode'UnrecognizedValue = Opcode'UnrecognizedValue Data.Int.Int32
                                     deriving (Prelude.Eq, Prelude.Ord, Prelude.Show)
instance Data.ProtoLens.MessageEnum Opcode where
        maybeToEnum 0 = Prelude.Just NOP
        maybeToEnum 2 = Prelude.Just JUMP
        maybeToEnum 3 = Prelude.Just PUSH
        maybeToEnum 4 = Prelude.Just RETURN
        maybeToEnum 5 = Prelude.Just PRIMOP_P
        maybeToEnum 6 = Prelude.Just PRIMOP_R
        maybeToEnum 7 = Prelude.Just CONSTR_P
        maybeToEnum 8 = Prelude.Just CONSTR_R
        maybeToEnum 9 = Prelude.Just FUNC_P
        maybeToEnum 10 = Prelude.Just FUNC_R
        maybeToEnum 11 = Prelude.Just THUNK_F_P
        maybeToEnum 12 = Prelude.Just THUNK_F_R
        maybeToEnum 13 = Prelude.Just THUNK_C_P
        maybeToEnum 14 = Prelude.Just THUNK_C_R
        maybeToEnum 15 = Prelude.Just BOTTOM_P
        maybeToEnum 16 = Prelude.Just BOTTOM_R
        maybeToEnum 17 = Prelude.Just EVAL
        maybeToEnum 18 = Prelude.Just MATCH_A_E_P
        maybeToEnum 19 = Prelude.Just MATCH_A_E_R
        maybeToEnum k
          = Prelude.Just
              (Opcode'Unrecognized
                 (Opcode'UnrecognizedValue (Prelude.fromIntegral k)))
        showEnum NOP = "NOP"
        showEnum JUMP = "JUMP"
        showEnum PUSH = "PUSH"
        showEnum RETURN = "RETURN"
        showEnum PRIMOP_P = "PRIMOP_P"
        showEnum PRIMOP_R = "PRIMOP_R"
        showEnum CONSTR_P = "CONSTR_P"
        showEnum CONSTR_R = "CONSTR_R"
        showEnum FUNC_P = "FUNC_P"
        showEnum FUNC_R = "FUNC_R"
        showEnum THUNK_F_P = "THUNK_F_P"
        showEnum THUNK_F_R = "THUNK_F_R"
        showEnum THUNK_C_P = "THUNK_C_P"
        showEnum THUNK_C_R = "THUNK_C_R"
        showEnum BOTTOM_P = "BOTTOM_P"
        showEnum BOTTOM_R = "BOTTOM_R"
        showEnum EVAL = "EVAL"
        showEnum MATCH_A_E_P = "MATCH_A_E_P"
        showEnum MATCH_A_E_R = "MATCH_A_E_R"
        showEnum (Opcode'Unrecognized (Opcode'UnrecognizedValue k))
          = Prelude.show k
        readEnum "NOP" = Prelude.Just NOP
        readEnum "JUMP" = Prelude.Just JUMP
        readEnum "PUSH" = Prelude.Just PUSH
        readEnum "RETURN" = Prelude.Just RETURN
        readEnum "PRIMOP_P" = Prelude.Just PRIMOP_P
        readEnum "PRIMOP_R" = Prelude.Just PRIMOP_R
        readEnum "CONSTR_P" = Prelude.Just CONSTR_P
        readEnum "CONSTR_R" = Prelude.Just CONSTR_R
        readEnum "FUNC_P" = Prelude.Just FUNC_P
        readEnum "FUNC_R" = Prelude.Just FUNC_R
        readEnum "THUNK_F_P" = Prelude.Just THUNK_F_P
        readEnum "THUNK_F_R" = Prelude.Just THUNK_F_R
        readEnum "THUNK_C_P" = Prelude.Just THUNK_C_P
        readEnum "THUNK_C_R" = Prelude.Just THUNK_C_R
        readEnum "BOTTOM_P" = Prelude.Just BOTTOM_P
        readEnum "BOTTOM_R" = Prelude.Just BOTTOM_R
        readEnum "EVAL" = Prelude.Just EVAL
        readEnum "MATCH_A_E_P" = Prelude.Just MATCH_A_E_P
        readEnum "MATCH_A_E_R" = Prelude.Just MATCH_A_E_R
        readEnum k
          = (Prelude.>>=) (Text.Read.readMaybe k) Data.ProtoLens.maybeToEnum
instance Prelude.Bounded Opcode where
        minBound = NOP
        maxBound = MATCH_A_E_R
instance Prelude.Enum Opcode where
        toEnum k__
          = Prelude.maybe
              (Prelude.error
                 ((Prelude.++) "toEnum: unknown value for enum Opcode: "
                    (Prelude.show k__)))
              Prelude.id
              (Data.ProtoLens.maybeToEnum k__)
        fromEnum NOP = 0
        fromEnum JUMP = 2
        fromEnum PUSH = 3
        fromEnum RETURN = 4
        fromEnum PRIMOP_P = 5
        fromEnum PRIMOP_R = 6
        fromEnum CONSTR_P = 7
        fromEnum CONSTR_R = 8
        fromEnum FUNC_P = 9
        fromEnum FUNC_R = 10
        fromEnum THUNK_F_P = 11
        fromEnum THUNK_F_R = 12
        fromEnum THUNK_C_P = 13
        fromEnum THUNK_C_R = 14
        fromEnum BOTTOM_P = 15
        fromEnum BOTTOM_R = 16
        fromEnum EVAL = 17
        fromEnum MATCH_A_E_P = 18
        fromEnum MATCH_A_E_R = 19
        fromEnum (Opcode'Unrecognized (Opcode'UnrecognizedValue k))
          = Prelude.fromIntegral k
        succ MATCH_A_E_R
          = Prelude.error
              "Opcode.succ: bad argument MATCH_A_E_R. This value would be out of bounds."
        succ NOP = JUMP
        succ JUMP = PUSH
        succ PUSH = RETURN
        succ RETURN = PRIMOP_P
        succ PRIMOP_P = PRIMOP_R
        succ PRIMOP_R = CONSTR_P
        succ CONSTR_P = CONSTR_R
        succ CONSTR_R = FUNC_P
        succ FUNC_P = FUNC_R
        succ FUNC_R = THUNK_F_P
        succ THUNK_F_P = THUNK_F_R
        succ THUNK_F_R = THUNK_C_P
        succ THUNK_C_P = THUNK_C_R
        succ THUNK_C_R = BOTTOM_P
        succ BOTTOM_P = BOTTOM_R
        succ BOTTOM_R = EVAL
        succ EVAL = MATCH_A_E_P
        succ MATCH_A_E_P = MATCH_A_E_R
        succ _
          = Prelude.error "Opcode.succ: bad argument: unrecognized value"
        pred NOP
          = Prelude.error
              "Opcode.pred: bad argument NOP. This value would be out of bounds."
        pred JUMP = NOP
        pred PUSH = JUMP
        pred RETURN = PUSH
        pred PRIMOP_P = RETURN
        pred PRIMOP_R = PRIMOP_P
        pred CONSTR_P = PRIMOP_R
        pred CONSTR_R = CONSTR_P
        pred FUNC_P = CONSTR_R
        pred FUNC_R = FUNC_P
        pred THUNK_F_P = FUNC_R
        pred THUNK_F_R = THUNK_F_P
        pred THUNK_C_P = THUNK_F_R
        pred THUNK_C_R = THUNK_C_P
        pred BOTTOM_P = THUNK_C_R
        pred BOTTOM_R = BOTTOM_P
        pred EVAL = BOTTOM_R
        pred MATCH_A_E_P = EVAL
        pred MATCH_A_E_R = MATCH_A_E_P
        pred _
          = Prelude.error "Opcode.pred: bad argument: unrecognized value"
        enumFrom = Data.ProtoLens.Message.Enum.messageEnumFrom
        enumFromTo = Data.ProtoLens.Message.Enum.messageEnumFromTo
        enumFromThen = Data.ProtoLens.Message.Enum.messageEnumFromThen
        enumFromThenTo = Data.ProtoLens.Message.Enum.messageEnumFromThenTo
instance Data.Default.Class.Default Opcode where
        def = NOP
instance Data.ProtoLens.FieldDefault Opcode where
        fieldDefault = NOP