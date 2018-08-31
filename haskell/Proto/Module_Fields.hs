{- This file was auto-generated from module.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.Module_Fields where
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

cafExports ::
           forall f s t a b . (Lens.Labels.HasLens f s t "cafExports" a b) =>
             Lens.Family2.LensLike f s t a b
cafExports
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafExports")
cafs ::
     forall f s t a b . (Lens.Labels.HasLens f s t "cafs" a b) =>
       Lens.Family2.LensLike f s t a b
cafs
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "cafs")
codeExports ::
            forall f s t a b . (Lens.Labels.HasLens f s t "codeExports" a b) =>
              Lens.Family2.LensLike f s t a b
codeExports
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "codeExports")
entryPoint ::
           forall f s t a b . (Lens.Labels.HasLens f s t "entryPoint" a b) =>
             Lens.Family2.LensLike f s t a b
entryPoint
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "entryPoint")
imports ::
        forall f s t a b . (Lens.Labels.HasLens f s t "imports" a b) =>
          Lens.Family2.LensLike f s t a b
imports
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "imports")
key ::
    forall f s t a b . (Lens.Labels.HasLens f s t "key" a b) =>
      Lens.Family2.LensLike f s t a b
key
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "key")
local ::
      forall f s t a b . (Lens.Labels.HasLens f s t "local" a b) =>
        Lens.Family2.LensLike f s t a b
local
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "local")
localOffset ::
            forall f s t a b . (Lens.Labels.HasLens f s t "localOffset" a b) =>
              Lens.Family2.LensLike f s t a b
localOffset
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "localOffset")
majorVersion ::
             forall f s t a b .
               (Lens.Labels.HasLens f s t "majorVersion" a b) =>
               Lens.Family2.LensLike f s t a b
majorVersion
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "majorVersion")
maybe'value ::
            forall f s t a b . (Lens.Labels.HasLens f s t "maybe'value" a b) =>
              Lens.Family2.LensLike f s t a b
maybe'value
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "maybe'value")
minorVersion ::
             forall f s t a b .
               (Lens.Labels.HasLens f s t "minorVersion" a b) =>
               Lens.Family2.LensLike f s t a b
minorVersion
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "minorVersion")
module' ::
        forall f s t a b . (Lens.Labels.HasLens f s t "module'" a b) =>
          Lens.Family2.LensLike f s t a b
module'
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "module'")
name ::
     forall f s t a b . (Lens.Labels.HasLens f s t "name" a b) =>
       Lens.Family2.LensLike f s t a b
name
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "name")
package ::
        forall f s t a b . (Lens.Labels.HasLens f s t "package" a b) =>
          Lens.Family2.LensLike f s t a b
package
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "package")
text ::
     forall f s t a b . (Lens.Labels.HasLens f s t "text" a b) =>
       Lens.Family2.LensLike f s t a b
text
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "text")
value ::
      forall f s t a b . (Lens.Labels.HasLens f s t "value" a b) =>
        Lens.Family2.LensLike f s t a b
value
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "value")