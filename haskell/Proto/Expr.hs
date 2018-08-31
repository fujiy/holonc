{- This file was auto-generated from expr.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.Expr
       (Expr(..), ExprType(..), ExprType(), ExprType'UnrecognizedValue)
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

    * 'Proto.Expr_Fields.type'' @:: Lens' Expr ExprType@
    * 'Proto.Expr_Fields.pNum' @:: Lens' Expr Data.Word.Word32@
    * 'Proto.Expr_Fields.tag' @:: Lens' Expr Data.Word.Word32@
    * 'Proto.Expr_Fields.payloads' @:: Lens' Expr [Data.Word.Word64]@
 -}
data Expr = Expr{_Expr'type' :: !ExprType,
                 _Expr'pNum :: !Data.Word.Word32, _Expr'tag :: !Data.Word.Word32,
                 _Expr'payloads :: ![Data.Word.Word64],
                 _Expr'_unknownFields :: !Data.ProtoLens.FieldSet}
              deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Expr x a, a ~ b) =>
         Lens.Labels.HasLens f Expr Expr x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Expr "type'" (ExprType)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Expr'type'
                 (\ x__ y__ -> x__{_Expr'type' = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Expr "pNum" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Expr'pNum
                 (\ x__ y__ -> x__{_Expr'pNum = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Expr "tag" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Expr'tag
                 (\ x__ y__ -> x__{_Expr'tag = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Expr "payloads" ([Data.Word.Word64])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Expr'payloads
                 (\ x__ y__ -> x__{_Expr'payloads = y__}))
              Prelude.id
instance Data.Default.Class.Default Expr where
        def
          = Expr{_Expr'type' = Data.Default.Class.def,
                 _Expr'pNum = Data.ProtoLens.fieldDefault,
                 _Expr'tag = Data.ProtoLens.fieldDefault, _Expr'payloads = [],
                 _Expr'_unknownFields = ([])}
instance Data.ProtoLens.Message Expr where
        messageName _ = Data.Text.pack "Expr"
        fieldsByTag
          = let type'__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "type"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.EnumField ::
                         Data.ProtoLens.FieldTypeDescriptor ExprType)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "type'")))
                      :: Data.ProtoLens.FieldDescriptor Expr
                pNum__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "p_num"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "pNum")))
                      :: Data.ProtoLens.FieldDescriptor Expr
                tag__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "tag"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "tag")))
                      :: Data.ProtoLens.FieldDescriptor Expr
                payloads__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "payloads"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Packed
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "payloads")))
                      :: Data.ProtoLens.FieldDescriptor Expr
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, type'__field_descriptor),
                 (Data.ProtoLens.Tag 2, pNum__field_descriptor),
                 (Data.ProtoLens.Tag 3, tag__field_descriptor),
                 (Data.ProtoLens.Tag 4, payloads__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Expr'_unknownFields
              (\ x__ y__ -> x__{_Expr'_unknownFields = y__})
data ExprType = INVALID
              | INDIR
              | CONSTR
              | FUNC
              | THUNK_F
              | THUNK_C
              | PAPPLY
              | BOTTOM
              | ExprType'Unrecognized !ExprType'UnrecognizedValue
                  deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
newtype ExprType'UnrecognizedValue = ExprType'UnrecognizedValue Data.Int.Int32
                                       deriving (Prelude.Eq, Prelude.Ord, Prelude.Show)
instance Data.ProtoLens.MessageEnum ExprType where
        maybeToEnum 0 = Prelude.Just INVALID
        maybeToEnum 1 = Prelude.Just INDIR
        maybeToEnum 2 = Prelude.Just CONSTR
        maybeToEnum 4 = Prelude.Just FUNC
        maybeToEnum 6 = Prelude.Just THUNK_F
        maybeToEnum 7 = Prelude.Just THUNK_C
        maybeToEnum 8 = Prelude.Just PAPPLY
        maybeToEnum 9 = Prelude.Just BOTTOM
        maybeToEnum k
          = Prelude.Just
              (ExprType'Unrecognized
                 (ExprType'UnrecognizedValue (Prelude.fromIntegral k)))
        showEnum INVALID = "INVALID"
        showEnum INDIR = "INDIR"
        showEnum CONSTR = "CONSTR"
        showEnum FUNC = "FUNC"
        showEnum THUNK_F = "THUNK_F"
        showEnum THUNK_C = "THUNK_C"
        showEnum PAPPLY = "PAPPLY"
        showEnum BOTTOM = "BOTTOM"
        showEnum (ExprType'Unrecognized (ExprType'UnrecognizedValue k))
          = Prelude.show k
        readEnum "INVALID" = Prelude.Just INVALID
        readEnum "INDIR" = Prelude.Just INDIR
        readEnum "CONSTR" = Prelude.Just CONSTR
        readEnum "FUNC" = Prelude.Just FUNC
        readEnum "THUNK_F" = Prelude.Just THUNK_F
        readEnum "THUNK_C" = Prelude.Just THUNK_C
        readEnum "PAPPLY" = Prelude.Just PAPPLY
        readEnum "BOTTOM" = Prelude.Just BOTTOM
        readEnum k
          = (Prelude.>>=) (Text.Read.readMaybe k) Data.ProtoLens.maybeToEnum
instance Prelude.Bounded ExprType where
        minBound = INVALID
        maxBound = BOTTOM
instance Prelude.Enum ExprType where
        toEnum k__
          = Prelude.maybe
              (Prelude.error
                 ((Prelude.++) "toEnum: unknown value for enum ExprType: "
                    (Prelude.show k__)))
              Prelude.id
              (Data.ProtoLens.maybeToEnum k__)
        fromEnum INVALID = 0
        fromEnum INDIR = 1
        fromEnum CONSTR = 2
        fromEnum FUNC = 4
        fromEnum THUNK_F = 6
        fromEnum THUNK_C = 7
        fromEnum PAPPLY = 8
        fromEnum BOTTOM = 9
        fromEnum (ExprType'Unrecognized (ExprType'UnrecognizedValue k))
          = Prelude.fromIntegral k
        succ BOTTOM
          = Prelude.error
              "ExprType.succ: bad argument BOTTOM. This value would be out of bounds."
        succ INVALID = INDIR
        succ INDIR = CONSTR
        succ CONSTR = FUNC
        succ FUNC = THUNK_F
        succ THUNK_F = THUNK_C
        succ THUNK_C = PAPPLY
        succ PAPPLY = BOTTOM
        succ _
          = Prelude.error "ExprType.succ: bad argument: unrecognized value"
        pred INVALID
          = Prelude.error
              "ExprType.pred: bad argument INVALID. This value would be out of bounds."
        pred INDIR = INVALID
        pred CONSTR = INDIR
        pred FUNC = CONSTR
        pred THUNK_F = FUNC
        pred THUNK_C = THUNK_F
        pred PAPPLY = THUNK_C
        pred BOTTOM = PAPPLY
        pred _
          = Prelude.error "ExprType.pred: bad argument: unrecognized value"
        enumFrom = Data.ProtoLens.Message.Enum.messageEnumFrom
        enumFromTo = Data.ProtoLens.Message.Enum.messageEnumFromTo
        enumFromThen = Data.ProtoLens.Message.Enum.messageEnumFromThen
        enumFromThenTo = Data.ProtoLens.Message.Enum.messageEnumFromThenTo
instance Data.Default.Class.Default ExprType where
        def = INVALID
instance Data.ProtoLens.FieldDefault ExprType where
        fieldDefault = INVALID