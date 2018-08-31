{- This file was auto-generated from instr.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.Instr_Fields where
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

arity ::
      forall f s t a b . (Lens.Labels.HasLens f s t "arity" a b) =>
        Lens.Family2.LensLike f s t a b
arity
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "arity")
bitmap ::
       forall f s t a b . (Lens.Labels.HasLens f s t "bitmap" a b) =>
         Lens.Family2.LensLike f s t a b
bitmap
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "bitmap")
blocks ::
       forall f s t a b . (Lens.Labels.HasLens f s t "blocks" a b) =>
         Lens.Family2.LensLike f s t a b
blocks
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "blocks")
code ::
     forall f s t a b . (Lens.Labels.HasLens f s t "code" a b) =>
       Lens.Family2.LensLike f s t a b
code
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "code")
opcode ::
       forall f s t a b . (Lens.Labels.HasLens f s t "opcode" a b) =>
         Lens.Family2.LensLike f s t a b
opcode
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "opcode")
operands ::
         forall f s t a b . (Lens.Labels.HasLens f s t "operands" a b) =>
           Lens.Family2.LensLike f s t a b
operands
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "operands")
pNum ::
     forall f s t a b . (Lens.Labels.HasLens f s t "pNum" a b) =>
       Lens.Family2.LensLike f s t a b
pNum
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "pNum")
srt ::
    forall f s t a b . (Lens.Labels.HasLens f s t "srt" a b) =>
      Lens.Family2.LensLike f s t a b
srt
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "srt")
symbol ::
       forall f s t a b . (Lens.Labels.HasLens f s t "symbol" a b) =>
         Lens.Family2.LensLike f s t a b
symbol
  = Lens.Labels.lensOf
      ((Lens.Labels.proxy#) :: (Lens.Labels.Proxy#) "symbol")