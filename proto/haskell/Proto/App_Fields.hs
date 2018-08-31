{- This file was auto-generated from app.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.App_Fields where
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

app ::
    forall f s t a b . (Lens.Labels.HasLens f s t "app" a b) =>
      Lens.Family2.LensLike f s t a b
app
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "app")
cafs ::
     forall f s t a b . (Lens.Labels.HasLens f s t "cafs" a b) =>
       Lens.Family2.LensLike f s t a b
cafs
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafs")
id ::
   forall f s t a b . (Lens.Labels.HasLens f s t "id" a b) =>
     Lens.Family2.LensLike f s t a b
id
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "id")
magicNumber ::
            forall f s t a b . (Lens.Labels.HasLens f s t "magicNumber" a b) =>
              Lens.Family2.LensLike f s t a b
magicNumber
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "magicNumber")
maybe'app ::
          forall f s t a b . (Lens.Labels.HasLens f s t "maybe'app" a b) =>
            Lens.Family2.LensLike f s t a b
maybe'app
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "maybe'app")
name ::
     forall f s t a b . (Lens.Labels.HasLens f s t "name" a b) =>
       Lens.Family2.LensLike f s t a b
name
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "name")
version ::
        forall f s t a b . (Lens.Labels.HasLens f s t "version" a b) =>
          Lens.Family2.LensLike f s t a b
version
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "version")