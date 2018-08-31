{- This file was auto-generated from app.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.App (App(..), AppFile(..)) where
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

{- | Fields :

    * 'Proto.App_Fields.id' @:: Lens' App Data.Word.Word64@
    * 'Proto.App_Fields.version' @:: Lens' App Data.Word.Word64@
    * 'Proto.App_Fields.name' @:: Lens' App Data.Text.Text@
    * 'Proto.App_Fields.cafs' @:: Lens' App [Proto.Expr.Expr]@
 -}
data App = App{_App'id :: !Data.Word.Word64,
               _App'version :: !Data.Word.Word64, _App'name :: !Data.Text.Text,
               _App'cafs :: ![Proto.Expr.Expr],
               _App'_unknownFields :: !Data.ProtoLens.FieldSet}
             deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f App x a, a ~ b) =>
         Lens.Labels.HasLens f App App x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f App "id" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _App'id
                 (\ x__ y__ -> x__{_App'id = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f App "version" (Data.Word.Word64)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _App'version
                 (\ x__ y__ -> x__{_App'version = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f App "name" (Data.Text.Text)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _App'name
                 (\ x__ y__ -> x__{_App'name = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f App "cafs" ([Proto.Expr.Expr])
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _App'cafs
                 (\ x__ y__ -> x__{_App'cafs = y__}))
              Prelude.id
instance Data.Default.Class.Default App where
        def
          = App{_App'id = Data.ProtoLens.fieldDefault,
                _App'version = Data.ProtoLens.fieldDefault,
                _App'name = Data.ProtoLens.fieldDefault, _App'cafs = [],
                _App'_unknownFields = ([])}
instance Data.ProtoLens.Message App where
        messageName _ = Data.Text.pack "App"
        fieldsByTag
          = let id__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "id"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "id")))
                      :: Data.ProtoLens.FieldDescriptor App
                version__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "version"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed64Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word64)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "version")))
                      :: Data.ProtoLens.FieldDescriptor App
                name__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "name"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.StringField ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Text.Text)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "name")))
                      :: Data.ProtoLens.FieldDescriptor App
                cafs__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "cafs"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor Proto.Expr.Expr)
                      (Data.ProtoLens.RepeatedField Data.ProtoLens.Unpacked
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafs")))
                      :: Data.ProtoLens.FieldDescriptor App
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, id__field_descriptor),
                 (Data.ProtoLens.Tag 2, version__field_descriptor),
                 (Data.ProtoLens.Tag 3, name__field_descriptor),
                 (Data.ProtoLens.Tag 10, cafs__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _App'_unknownFields
              (\ x__ y__ -> x__{_App'_unknownFields = y__})
{- | Fields :

    * 'Proto.App_Fields.magicNumber' @:: Lens' AppFile Data.Word.Word32@
    * 'Proto.App_Fields.app' @:: Lens' AppFile App@
    * 'Proto.App_Fields.maybe'app' @:: Lens' AppFile (Prelude.Maybe App)@
 -}
data AppFile = AppFile{_AppFile'magicNumber :: !Data.Word.Word32,
                       _AppFile'app :: !(Prelude.Maybe App),
                       _AppFile'_unknownFields :: !Data.ProtoLens.FieldSet}
                 deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
instance (Lens.Labels.HasLens' f AppFile x a, a ~ b) =>
         Lens.Labels.HasLens f AppFile AppFile x a b
         where
        lensOf = Lens.Labels.lensOf'
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f AppFile "magicNumber" (Data.Word.Word32)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _AppFile'magicNumber
                 (\ x__ y__ -> x__{_AppFile'magicNumber = y__}))
              Prelude.id
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f AppFile "app" (App)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _AppFile'app
                 (\ x__ y__ -> x__{_AppFile'app = y__}))
              (Data.ProtoLens.maybeLens Data.Default.Class.def)
instance Prelude.Functor f =>
         Lens.Labels.HasLens' f AppFile "maybe'app" (Prelude.Maybe App)
         where
        lensOf' _
          = (Prelude..)
              (Lens.Family2.Unchecked.lens _AppFile'app
                 (\ x__ y__ -> x__{_AppFile'app = y__}))
              Prelude.id
instance Data.Default.Class.Default AppFile where
        def
          = AppFile{_AppFile'magicNumber = Data.ProtoLens.fieldDefault,
                    _AppFile'app = Prelude.Nothing, _AppFile'_unknownFields = ([])}
instance Data.ProtoLens.Message AppFile where
        messageName _ = Data.Text.pack "AppFile"
        fieldsByTag
          = let magicNumber__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "magic_number"
                      (Data.ProtoLens.ScalarField Data.ProtoLens.Fixed32Field ::
                         Data.ProtoLens.FieldTypeDescriptor Data.Word.Word32)
                      (Data.ProtoLens.PlainField Data.ProtoLens.Optional
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "magicNumber")))
                      :: Data.ProtoLens.FieldDescriptor AppFile
                app__field_descriptor
                  = Data.ProtoLens.FieldDescriptor "app"
                      (Data.ProtoLens.MessageField Data.ProtoLens.MessageType ::
                         Data.ProtoLens.FieldTypeDescriptor App)
                      (Data.ProtoLens.OptionalField
                         (Lens.Labels.lensOf
                            ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "maybe'app")))
                      :: Data.ProtoLens.FieldDescriptor AppFile
              in
              Data.Map.fromList
                [(Data.ProtoLens.Tag 1, magicNumber__field_descriptor),
                 (Data.ProtoLens.Tag 2, app__field_descriptor)]
        unknownFields
          = Lens.Family2.Unchecked.lens _AppFile'_unknownFields
              (\ x__ y__ -> x__{_AppFile'_unknownFields = y__})