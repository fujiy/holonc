{- This file was auto-generated from module.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.Module
       (HObject(..), HObject'ImportsEntry(..), Symbol(..)) where
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
import qualified Proto.Expr
import qualified Proto.Instr

{- | Fields :

    * 'Proto.Module_Fields.name' @:: Lens' HObject Data.Text.Text@
    * 'Proto.Module_Fields.majorVersion' @:: Lens' HObject Data.Int.Int64@
    * 'Proto.Module_Fields.minorVersion' @:: Lens' HObject Data.Int.Int64@
    * 'Proto.Module_Fields.entryPoint' @:: Lens' HObject Data.Word.Word64@
    * 'Proto.Module_Fields.localOffset' @:: Lens' HObject Data.Word.Word64@
    * 'Proto.Module_Fields.imports' @:: Lens' HObject (Data.Map.Map Data.Word.Word64 Symbol)@
    * 'Proto.Module_Fields.cafExports' @:: Lens' HObject [Data.Word.Word64]@
    * 'Proto.Module_Fields.codeExports' @:: Lens' HObject [Data.Word.Word64]@
    * 'Proto.Module_Fields.cafs' @:: Lens' HObject [Proto.Expr.Expr]@
    * 'Proto.Module_Fields.text' @:: Lens' HObject [Proto.Instr.Func]@
 -}
data HObject = HObject{_HObject'name :: !Data.Text.Text,
                       _HObject'majorVersion :: !Data.Int.Int64,
                       _HObject'minorVersion :: !Data.Int.Int64,
                       _HObject'entryPoint :: !Data.Word.Word64,
                       _HObject'localOffset :: !Data.Word.Word64,
                       _HObject'imports :: !(Data.Map.Map Data.Word.Word64 Symbol),
                       _HObject'cafExports :: ![Data.Word.Word64],
                       _HObject'codeExports :: ![Data.Word.Word64],
                       _HObject'cafs :: ![Proto.Expr.Expr],
                       _HObject'text :: ![Proto.Instr.Func],
                       _HObject'_unknownFields :: !Data.ProtoLens.FieldSet}
                 deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f HObject x a, a ~ b) =>
         Lens.Labels.HasLens f HObject HObject x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "name" (Data.Text.Text)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'name
                 (\ x__ y__ -> x__{_HObject'name = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "majorVersion" (Data.Int.Int64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'majorVersion
                 (\ x__ y__ -> x__{_HObject'majorVersion = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "minorVersion" (Data.Int.Int64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'minorVersion
                 (\ x__ y__ -> x__{_HObject'minorVersion = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "entryPoint" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'entryPoint
                 (\ x__ y__ -> x__{_HObject'entryPoint = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "localOffset" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'localOffset
                 (\ x__ y__ -> x__{_HObject'localOffset = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "imports"
           (Data.Map.Map Data.Word.Word64 Symbol)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'imports
                 (\ x__ y__ -> x__{_HObject'imports = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "cafExports" ([Data.Word.Word64])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'cafExports
                 (\ x__ y__ -> x__{_HObject'cafExports = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "codeExports" ([Data.Word.Word64])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'codeExports
                 (\ x__ y__ -> x__{_HObject'codeExports = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "cafs" ([Proto.Expr.Expr])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'cafs
                 (\ x__ y__ -> x__{_HObject'cafs = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject "text" ([Proto.Instr.Func])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'text
                 (\ x__ y__ -> x__{_HObject'text = y__}))
              Prelude.id
instance Data.Default.Class.Default HObject where
        def
          = HObject{_HObject'name = Data.ProtoLens.fieldDefault,
                    _HObject'majorVersion = Data.ProtoLens.fieldDefault,
                    _HObject'minorVersion = Data.ProtoLens.fieldDefault,
                    _HObject'entryPoint = Data.ProtoLens.fieldDefault,
                    _HObject'localOffset = Data.ProtoLens.fieldDefault,
                    _HObject'imports = Data.Map.empty, _HObject'cafExports = [],
                    _HObject'codeExports = [], _HObject'cafs = [], _HObject'text = [],
                    _HObject'_unknownFields = ([])}
instance Data.ProtoLens.Message HObject where
        messageName _ = Data.Text.pack "HObject"
        fieldsByTag
          = let name__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "name"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.StringField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Text.Text)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "name")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                majorVersion__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "major_version"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Int64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Int.Int64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "majorVersion")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                minorVersion__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "minor_version"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Int64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Int.Int64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "minorVersion")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                entryPoint__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "entry_point"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "entryPoint")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                localOffset__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "local_offset"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "localOffset")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                imports__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "imports"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor HObject'ImportsEntry)
                      (Data.ProtoLens.MapField
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "key"))
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "value"))
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "imports")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                cafExports__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "caf_exports"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Packed
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafExports")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                codeExports__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "code_exports"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Packed
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "codeExports")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                cafs__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "cafs"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Proto.Expr.Expr)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafs")))
                      :: Data.ProtoLens.FieldDescriptor HObject
                text__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "text"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Proto.Instr.Func)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "text")))
                      :: Data.ProtoLens.FieldDescriptor HObject
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, name__field_descriptor),
                 (Data.ProtoLens.Tag 2, majorVersion__field_descriptor),
                 (Data.ProtoLens.Tag 3, minorVersion__field_descriptor),
                 (Data.ProtoLens.Tag 5, entryPoint__field_descriptor),
                 (Data.ProtoLens.Tag 6, localOffset__field_descriptor),
                 (Data.ProtoLens.Tag 10, imports__field_descriptor),
                 (Data.ProtoLens.Tag 11, cafExports__field_descriptor),
                 (Data.ProtoLens.Tag 12, codeExports__field_descriptor),
                 (Data.ProtoLens.Tag 13, cafs__field_descriptor),
                 (Data.ProtoLens.Tag 14, text__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _HObject'_unknownFields
              (\ x__ y__ -> x__{_HObject'_unknownFields = y__})
{- | Fields :

    * 'Proto.Module_Fields.key' @:: Lens' HObject'ImportsEntry Data.Word.Word64@
    * 'Proto.Module_Fields.value' @:: Lens' HObject'ImportsEntry Symbol@
    * 'Proto.Module_Fields.maybe'value' @:: Lens' HObject'ImportsEntry (Prelude.Maybe Symbol)@
 -}
data HObject'ImportsEntry = HObject'ImportsEntry{_HObject'ImportsEntry'key
                                                 :: !Data.Word.Word64,
                                                 _HObject'ImportsEntry'value ::
                                                 !(Prelude.Maybe Symbol),
                                                 _HObject'ImportsEntry'_unknownFields ::
                                                 !Data.ProtoLens.FieldSet}
                              deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f HObject'ImportsEntry x a,
          a ~ b) =>
         Lens.Labels.HasLens f HObject'ImportsEntry HObject'ImportsEntry x a
           b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject'ImportsEntry "key"
           (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'ImportsEntry'key
                 (\ x__ y__ -> x__{_HObject'ImportsEntry'key = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject'ImportsEntry "value" (Symbol)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'ImportsEntry'value
                 (\ x__ y__ -> x__{_HObject'ImportsEntry'value = y__}))
              (Data.ProtoLens.maybeLens Data.Default.Class.def)
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f HObject'ImportsEntry "maybe'value"
           (Prelude.Maybe Symbol)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _HObject'ImportsEntry'value
                 (\ x__ y__ -> x__{_HObject'ImportsEntry'value = y__}))
              Prelude.id
instance Data.Default.Class.Default HObject'ImportsEntry where
        def
          = HObject'ImportsEntry{_HObject'ImportsEntry'key =
                                   Data.ProtoLens.fieldDefault,
                                 _HObject'ImportsEntry'value = Prelude.Nothing,
                                 _HObject'ImportsEntry'_unknownFields = ([])}
instance Data.ProtoLens.Message HObject'ImportsEntry where
        messageName _ = Data.Text.pack "HObject.ImportsEntry"
        fieldsByTag
          = let key__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "key"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "key")))
                      :: Data.ProtoLens.FieldDescriptor HObject'ImportsEntry
                value__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "value"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Symbol)
                      (Data.ProtoLens.OptionalField
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "maybe'value")))
                      :: Data.ProtoLens.FieldDescriptor HObject'ImportsEntry
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, key__field_descriptor),
                 (Data.ProtoLens.Tag 2, value__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _HObject'ImportsEntry'_unknownFields
              (\ x__ y__ -> x__{_HObject'ImportsEntry'_unknownFields = y__})
{- | Fields :

    * 'Proto.Module_Fields.package' @:: Lens' Symbol Data.Int.Int32@
    * 'Proto.Module_Fields.module'' @:: Lens' Symbol Data.Int.Int32@
    * 'Proto.Module_Fields.local' @:: Lens' Symbol Data.Word.Word64@
    * 'Proto.Module_Fields.name' @:: Lens' Symbol Data.Text.Text@
 -}
data Symbol = Symbol{_Symbol'package :: !Data.Int.Int32,
                     _Symbol'module' :: !Data.Int.Int32,
                     _Symbol'local :: !Data.Word.Word64,
                     _Symbol'name :: !Data.Text.Text,
                     _Symbol'_unknownFields :: !Data.ProtoLens.FieldSet}
                deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Symbol x a, a ~ b) =>
         Lens.Labels.HasLens f Symbol Symbol x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Symbol "package" (Data.Int.Int32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Symbol'package
                 (\ x__ y__ -> x__{_Symbol'package = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Symbol "module'" (Data.Int.Int32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Symbol'module'
                 (\ x__ y__ -> x__{_Symbol'module' = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Symbol "local" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Symbol'local
                 (\ x__ y__ -> x__{_Symbol'local = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Symbol "name" (Data.Text.Text)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Symbol'name
                 (\ x__ y__ -> x__{_Symbol'name = y__}))
              Prelude.id
instance Data.Default.Class.Default Symbol where
        def
          = Symbol{_Symbol'package = Data.ProtoLens.fieldDefault,
                   _Symbol'module' = Data.ProtoLens.fieldDefault,
                   _Symbol'local = Data.ProtoLens.fieldDefault,
                   _Symbol'name = Data.ProtoLens.fieldDefault,
                   _Symbol'_unknownFields = ([])}
instance Data.ProtoLens.Message Symbol where
        messageName _ = Data.Text.pack "Symbol"
        fieldsByTag
          = let package__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "package"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Int32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Int.Int32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "package")))
                      :: Data.ProtoLens.FieldDescriptor Symbol
                module'__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "module"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Int32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Int.Int32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "module'")))
                      :: Data.ProtoLens.FieldDescriptor Symbol
                local__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "local"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "local")))
                      :: Data.ProtoLens.FieldDescriptor Symbol
                name__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "name"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.StringField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Text.Text)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "name")))
                      :: Data.ProtoLens.FieldDescriptor Symbol
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, package__field_descriptor),
                 (Data.ProtoLens.Tag 2, module'__field_descriptor),
                 (Data.ProtoLens.Tag 3, local__field_descriptor),
                 (Data.ProtoLens.Tag 4, name__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Symbol'_unknownFields
              (\ x__ y__ -> x__{_Symbol'_unknownFields = y__})