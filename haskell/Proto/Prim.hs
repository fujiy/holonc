{- This file was auto-generated from prim.proto by the proto-lens-protoc program. -}
{-# LANGUAGE ScopedTypeVariables, DataKinds, TypeFamilies,
  UndecidableInstances, GeneralizedNewtypeDeriving,
  MultiParamTypeClasses, FlexibleContexts, FlexibleInstances,
  PatternSynonyms, MagicHash, NoImplicitPrelude, DataKinds #-}
{-# OPTIONS_GHC -fno-warn-unused-imports#-}
{-# OPTIONS_GHC -fno-warn-duplicate-exports#-}
module Proto.Prim
       (HPrimOp(..), HPrimOp(), HPrimOp'UnrecognizedValue) where
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

data HPrimOp = PNOP
             | INTADD
             | INTSUB
             | INTMUL
             | INTNEG
             | INTGT
             | INTGE
             | INTEQ
             | INTNE
             | INTLT
             | INTLE
             | DATATOTAG
             | TAGTOENUM
             | HPrimOp'Unrecognized !HPrimOp'UnrecognizedValue
                 deriving (Prelude.Show, Prelude.Eq, Prelude.Ord)
newtype HPrimOp'UnrecognizedValue = HPrimOp'UnrecognizedValue Data.Int.Int32
                                      deriving (Prelude.Eq, Prelude.Ord, Prelude.Show)
instance Data.ProtoLens.MessageEnum HPrimOp where
        maybeToEnum 0 = Prelude.Just PNOP
        maybeToEnum 8 = Prelude.Just INTADD
        maybeToEnum 9 = Prelude.Just INTSUB
        maybeToEnum 10 = Prelude.Just INTMUL
        maybeToEnum 19 = Prelude.Just INTNEG
        maybeToEnum 22 = Prelude.Just INTGT
        maybeToEnum 23 = Prelude.Just INTGE
        maybeToEnum 24 = Prelude.Just INTEQ
        maybeToEnum 25 = Prelude.Just INTNE
        maybeToEnum 26 = Prelude.Just INTLT
        maybeToEnum 27 = Prelude.Just INTLE
        maybeToEnum 100 = Prelude.Just DATATOTAG
        maybeToEnum 101 = Prelude.Just TAGTOENUM
        maybeToEnum k
          = Prelude.Just
              (HPrimOp'Unrecognized
                 (HPrimOp'UnrecognizedValue (Prelude.fromIntegral k)))
        showEnum PNOP = "PNOP"
        showEnum INTADD = "INTADD"
        showEnum INTSUB = "INTSUB"
        showEnum INTMUL = "INTMUL"
        showEnum INTNEG = "INTNEG"
        showEnum INTGT = "INTGT"
        showEnum INTGE = "INTGE"
        showEnum INTEQ = "INTEQ"
        showEnum INTNE = "INTNE"
        showEnum INTLT = "INTLT"
        showEnum INTLE = "INTLE"
        showEnum DATATOTAG = "DATATOTAG"
        showEnum TAGTOENUM = "TAGTOENUM"
        showEnum (HPrimOp'Unrecognized (HPrimOp'UnrecognizedValue k))
          = Prelude.show k
        readEnum "PNOP" = Prelude.Just PNOP
        readEnum "INTADD" = Prelude.Just INTADD
        readEnum "INTSUB" = Prelude.Just INTSUB
        readEnum "INTMUL" = Prelude.Just INTMUL
        readEnum "INTNEG" = Prelude.Just INTNEG
        readEnum "INTGT" = Prelude.Just INTGT
        readEnum "INTGE" = Prelude.Just INTGE
        readEnum "INTEQ" = Prelude.Just INTEQ
        readEnum "INTNE" = Prelude.Just INTNE
        readEnum "INTLT" = Prelude.Just INTLT
        readEnum "INTLE" = Prelude.Just INTLE
        readEnum "DATATOTAG" = Prelude.Just DATATOTAG
        readEnum "TAGTOENUM" = Prelude.Just TAGTOENUM
        readEnum k
          = (Prelude.>>=) (Text.Read.readMaybe k) Data.ProtoLens.maybeToEnum
instance Prelude.Bounded HPrimOp where
        minBound = PNOP
        maxBound = TAGTOENUM
instance Prelude.Enum HPrimOp where
        toEnum k__
          = Prelude.maybe
              (Prelude.error
                 ((Prelude.++) "toEnum: unknown value for enum HPrimOp: "
                    (Prelude.show k__)))
              Prelude.id
              (Data.ProtoLens.maybeToEnum k__)
        fromEnum PNOP = 0
        fromEnum INTADD = 8
        fromEnum INTSUB = 9
        fromEnum INTMUL = 10
        fromEnum INTNEG = 19
        fromEnum INTGT = 22
        fromEnum INTGE = 23
        fromEnum INTEQ = 24
        fromEnum INTNE = 25
        fromEnum INTLT = 26
        fromEnum INTLE = 27
        fromEnum DATATOTAG = 100
        fromEnum TAGTOENUM = 101
        fromEnum (HPrimOp'Unrecognized (HPrimOp'UnrecognizedValue k))
          = Prelude.fromIntegral k
        succ TAGTOENUM
          = Prelude.error
              "HPrimOp.succ: bad argument TAGTOENUM. This value would be out of bounds."
        succ PNOP = INTADD
        succ INTADD = INTSUB
        succ INTSUB = INTMUL
        succ INTMUL = INTNEG
        succ INTNEG = INTGT
        succ INTGT = INTGE
        succ INTGE = INTEQ
        succ INTEQ = INTNE
        succ INTNE = INTLT
        succ INTLT = INTLE
        succ INTLE = DATATOTAG
        succ DATATOTAG = TAGTOENUM
        succ _
          = Prelude.error "HPrimOp.succ: bad argument: unrecognized value"
        pred PNOP
          = Prelude.error
              "HPrimOp.pred: bad argument PNOP. This value would be out of bounds."
        pred INTADD = PNOP
        pred INTSUB = INTADD
        pred INTMUL = INTSUB
        pred INTNEG = INTMUL
        pred INTGT = INTNEG
        pred INTGE = INTGT
        pred INTEQ = INTGE
        pred INTNE = INTEQ
        pred INTLT = INTNE
        pred INTLE = INTLT
        pred DATATOTAG = INTLE
        pred TAGTOENUM = DATATOTAG
        pred _
          = Prelude.error "HPrimOp.pred: bad argument: unrecognized value"
        enumFrom = Data.ProtoLens.Message.Enum.messageEnumFrom
        enumFromTo = Data.ProtoLens.Message.Enum.messageEnumFromTo
        enumFromThen = Data.ProtoLens.Message.Enum.messageEnumFromThen
        enumFromThenTo = Data.ProtoLens.Message.Enum.messageEnumFromThenTo
instance Data.Default.Class.Default HPrimOp where
        def = PNOP
instance Data.ProtoLens.FieldDefault HPrimOp where
        fieldDefault = PNOP