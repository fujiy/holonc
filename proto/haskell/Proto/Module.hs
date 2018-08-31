{- This file was auto-generated from module.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.Module
       (Module(..), Module'ImportsEntry(..), Package(..), Symbol(..))
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
import qualified Proto.Expr
import qualified Proto.Instr

{- | Fields :

    * 'Proto.Module_Fields.name' @:: Lens' Module Data.Text.Text@
    * 'Proto.Module_Fields.entryPoint' @:: Lens' Module Data.Word.Word64@
    * 'Proto.Module_Fields.localOffset' @:: Lens' Module Data.Word.Word64@
    * 'Proto.Module_Fields.imports' @:: Lens' Module (Data.Map.Map Data.Word.Word64 Symbol)@
    * 'Proto.Module_Fields.cafExports' @:: Lens' Module [Data.Word.Word64]@
    * 'Proto.Module_Fields.codeExports' @:: Lens' Module [Data.Word.Word64]@
    * 'Proto.Module_Fields.cafs' @:: Lens' Module [Proto.Expr.Expr]@
    * 'Proto.Module_Fields.text' @:: Lens' Module [Proto.Instr.Func]@
 -}
data Module = Module{_Module'name :: !Data.Text.Text,
                     _Module'entryPoint :: !Data.Word.Word64,
                     _Module'localOffset :: !Data.Word.Word64,
                     _Module'imports :: !(Data.Map.Map Data.Word.Word64 Symbol),
                     _Module'cafExports :: ![Data.Word.Word64],
                     _Module'codeExports :: ![Data.Word.Word64],
                     _Module'cafs :: ![Proto.Expr.Expr],
                     _Module'text :: ![Proto.Instr.Func],
                     _Module'_unknownFields :: !Data.ProtoLens.FieldSet}
                deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Module x a, a ~ b) =>
         Lens.Labels.HasLens f Module Module x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "name" (Data.Text.Text)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'name
                 (\ x__ y__ -> x__{_Module'name = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "entryPoint" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'entryPoint
                 (\ x__ y__ -> x__{_Module'entryPoint = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "localOffset" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'localOffset
                 (\ x__ y__ -> x__{_Module'localOffset = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "imports"
           (Data.Map.Map Data.Word.Word64 Symbol)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'imports
                 (\ x__ y__ -> x__{_Module'imports = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "cafExports" ([Data.Word.Word64])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'cafExports
                 (\ x__ y__ -> x__{_Module'cafExports = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "codeExports" ([Data.Word.Word64])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'codeExports
                 (\ x__ y__ -> x__{_Module'codeExports = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "cafs" ([Proto.Expr.Expr])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'cafs
                 (\ x__ y__ -> x__{_Module'cafs = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module "text" ([Proto.Instr.Func])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'text
                 (\ x__ y__ -> x__{_Module'text = y__}))
              Prelude.id
instance Data.Default.Class.Default Module where
        def
          = Module{_Module'name = Data.ProtoLens.fieldDefault,
                   _Module'entryPoint = Data.ProtoLens.fieldDefault,
                   _Module'localOffset = Data.ProtoLens.fieldDefault,
                   _Module'imports = Data.Map.empty, _Module'cafExports = [],
                   _Module'codeExports = [], _Module'cafs = [], _Module'text = [],
                   _Module'_unknownFields = ([])}
instance Data.ProtoLens.Message Module where
        messageName _ = Data.Text.pack "Module"
        fieldsByTag
          = let name__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "name"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.StringField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Text.Text)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "name")))
                      :: Data.ProtoLens.FieldDescriptor Module
                entryPoint__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "entry_point"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "entryPoint")))
                      :: Data.ProtoLens.FieldDescriptor Module
                localOffset__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "local_offset"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "localOffset")))
                      :: Data.ProtoLens.FieldDescriptor Module
                imports__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "imports"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Module'ImportsEntry)
                      (Data.ProtoLens.MapField
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "key"))
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "value"))
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "imports")))
                      :: Data.ProtoLens.FieldDescriptor Module
                cafExports__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "caf_exports"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Packed
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafExports")))
                      :: Data.ProtoLens.FieldDescriptor Module
                codeExports__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "code_exports"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Packed
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "codeExports")))
                      :: Data.ProtoLens.FieldDescriptor Module
                cafs__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "cafs"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Proto.Expr.Expr)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafs")))
                      :: Data.ProtoLens.FieldDescriptor Module
                text__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "text"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Proto.Instr.Func)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "text")))
                      :: Data.ProtoLens.FieldDescriptor Module
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, name__field_descriptor),
                 (Data.ProtoLens.Tag 2, entryPoint__field_descriptor),
                 (Data.ProtoLens.Tag 3, localOffset__field_descriptor),
                 (Data.ProtoLens.Tag 8, imports__field_descriptor),
                 (Data.ProtoLens.Tag 9, cafExports__field_descriptor),
                 (Data.ProtoLens.Tag 10, codeExports__field_descriptor),
                 (Data.ProtoLens.Tag 11, cafs__field_descriptor),
                 (Data.ProtoLens.Tag 12, text__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Module'_unknownFields
              (\ x__ y__ -> x__{_Module'_unknownFields = y__})
{- | Fields :

    * 'Proto.Module_Fields.key' @:: Lens' Module'ImportsEntry Data.Word.Word64@
    * 'Proto.Module_Fields.value' @:: Lens' Module'ImportsEntry Symbol@
    * 'Proto.Module_Fields.maybe'value' @:: Lens' Module'ImportsEntry (Prelude.Maybe Symbol)@
 -}
data Module'ImportsEntry = Module'ImportsEntry{_Module'ImportsEntry'key
                                               :: !Data.Word.Word64,
                                               _Module'ImportsEntry'value ::
                                               !(Prelude.Maybe Symbol),
                                               _Module'ImportsEntry'_unknownFields ::
                                               !Data.ProtoLens.FieldSet}
                             deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Module'ImportsEntry x a, a ~ b) =>
         Lens.Labels.HasLens f Module'ImportsEntry Module'ImportsEntry x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module'ImportsEntry "key" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'ImportsEntry'key
                 (\ x__ y__ -> x__{_Module'ImportsEntry'key = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module'ImportsEntry "value" (Symbol)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'ImportsEntry'value
                 (\ x__ y__ -> x__{_Module'ImportsEntry'value = y__}))
              (Data.ProtoLens.maybeLens Data.Default.Class.def)
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Module'ImportsEntry "maybe'value"
           (Prelude.Maybe Symbol)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Module'ImportsEntry'value
                 (\ x__ y__ -> x__{_Module'ImportsEntry'value = y__}))
              Prelude.id
instance Data.Default.Class.Default Module'ImportsEntry where
        def
          = Module'ImportsEntry{_Module'ImportsEntry'key =
                                  Data.ProtoLens.fieldDefault,
                                _Module'ImportsEntry'value = Prelude.Nothing,
                                _Module'ImportsEntry'_unknownFields = ([])}
instance Data.ProtoLens.Message Module'ImportsEntry where
        messageName _ = Data.Text.pack "Module.ImportsEntry"
        fieldsByTag
          = let key__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "key"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "key")))
                      :: Data.ProtoLens.FieldDescriptor Module'ImportsEntry
                value__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "value"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Symbol)
                      (Data.ProtoLens.OptionalField
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "maybe'value")))
                      :: Data.ProtoLens.FieldDescriptor Module'ImportsEntry
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, key__field_descriptor),
                 (Data.ProtoLens.Tag 2, value__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Module'ImportsEntry'_unknownFields
              (\ x__ y__ -> x__{_Module'ImportsEntry'_unknownFields = y__})
{- | Fields :

    * 'Proto.Module_Fields.magic' @:: Lens' Package Data.Word.Word64@
    * 'Proto.Module_Fields.name' @:: Lens' Package Data.Text.Text@
    * 'Proto.Module_Fields.major' @:: Lens' Package Data.Word.Word32@
    * 'Proto.Module_Fields.minor' @:: Lens' Package Data.Word.Word32@
    * 'Proto.Module_Fields.revision' @:: Lens' Package Data.Word.Word32@
    * 'Proto.Module_Fields.build' @:: Lens' Package Data.Word.Word32@
    * 'Proto.Module_Fields.deps' @:: Lens' Package [Data.Text.Text]@
    * 'Proto.Module_Fields.modules' @:: Lens' Package [Module]@
 -}
data Package = Package{_Package'magic :: !Data.Word.Word64,
                       _Package'name :: !Data.Text.Text,
                       _Package'major :: !Data.Word.Word32,
                       _Package'minor :: !Data.Word.Word32,
                       _Package'revision :: !Data.Word.Word32,
                       _Package'build :: !Data.Word.Word32,
                       _Package'deps :: ![Data.Text.Text], _Package'modules :: ![Module],
                       _Package'_unknownFields :: !Data.ProtoLens.FieldSet}
                 deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Package x a, a ~ b) =>
         Lens.Labels.HasLens f Package Package x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "magic" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'magic
                 (\ x__ y__ -> x__{_Package'magic = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "name" (Data.Text.Text)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'name
                 (\ x__ y__ -> x__{_Package'name = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "major" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'major
                 (\ x__ y__ -> x__{_Package'major = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "minor" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'minor
                 (\ x__ y__ -> x__{_Package'minor = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "revision" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'revision
                 (\ x__ y__ -> x__{_Package'revision = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "build" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'build
                 (\ x__ y__ -> x__{_Package'build = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "deps" ([Data.Text.Text])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'deps
                 (\ x__ y__ -> x__{_Package'deps = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Package "modules" ([Module])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Package'modules
                 (\ x__ y__ -> x__{_Package'modules = y__}))
              Prelude.id
instance Data.Default.Class.Default Package where
        def
          = Package{_Package'magic = Data.ProtoLens.fieldDefault,
                    _Package'name = Data.ProtoLens.fieldDefault,
                    _Package'major = Data.ProtoLens.fieldDefault,
                    _Package'minor = Data.ProtoLens.fieldDefault,
                    _Package'revision = Data.ProtoLens.fieldDefault,
                    _Package'build = Data.ProtoLens.fieldDefault, _Package'deps = [],
                    _Package'modules = [], _Package'_unknownFields = ([])}
instance Data.ProtoLens.Message Package where
        messageName _ = Data.Text.pack "Package"
        fieldsByTag
          = let magic__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "magic"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "magic")))
                      :: Data.ProtoLens.FieldDescriptor Package
                name__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "name"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.StringField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Text.Text)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "name")))
                      :: Data.ProtoLens.FieldDescriptor Package
                major__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "major"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "major")))
                      :: Data.ProtoLens.FieldDescriptor Package
                minor__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "minor"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "minor")))
                      :: Data.ProtoLens.FieldDescriptor Package
                revision__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "revision"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "revision")))
                      :: Data.ProtoLens.FieldDescriptor Package
                build__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "build"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "build")))
                      :: Data.ProtoLens.FieldDescriptor Package
                deps__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "deps"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.StringField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Text.Text)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "deps")))
                      :: Data.ProtoLens.FieldDescriptor Package
                modules__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "modules"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Module)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "modules")))
                      :: Data.ProtoLens.FieldDescriptor Package
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, magic__field_descriptor),
                 (Data.ProtoLens.Tag 2, name__field_descriptor),
                 (Data.ProtoLens.Tag 3, major__field_descriptor),
                 (Data.ProtoLens.Tag 4, minor__field_descriptor),
                 (Data.ProtoLens.Tag 5, revision__field_descriptor),
                 (Data.ProtoLens.Tag 6, build__field_descriptor),
                 (Data.ProtoLens.Tag 10, deps__field_descriptor),
                 (Data.ProtoLens.Tag 11, modules__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _Package'_unknownFields
              (\ x__ y__ -> x__{_Package'_unknownFields = y__})
{- | Fields :

    * 'Proto.Module_Fields.package' @:: Lens' Symbol Data.Word.Word32@
    * 'Proto.Module_Fields.module'' @:: Lens' Symbol Data.Word.Word32@
    * 'Proto.Module_Fields.local' @:: Lens' Symbol Data.Word.Word64@
    * 'Proto.Module_Fields.name' @:: Lens' Symbol Data.Text.Text@
 -}
data Symbol = Symbol{_Symbol'package :: !Data.Word.Word32,
                     _Symbol'module' :: !Data.Word.Word32,
                     _Symbol'local :: !Data.Word.Word64,
                     _Symbol'name :: !Data.Text.Text,
                     _Symbol'_unknownFields :: !Data.ProtoLens.FieldSet}
                deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f Symbol x a, a ~ b) =>
         Lens.Labels.HasLens f Symbol Symbol x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Symbol "package" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _Symbol'package
                 (\ x__ y__ -> x__{_Symbol'package = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f Symbol "module'" (Data.Word.Word32)
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
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "package")))
                      :: Data.ProtoLens.FieldDescriptor Symbol
                module'__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "module"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.UInt32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
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