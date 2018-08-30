{-# LANGUAGE MagicHash, NoImplicitPrelude, TypeFamilies, UnboxedTuples,
             MultiParamTypeClasses, RoleAnnotations, CPP, TypeOperators,
             PolyKinds #-}
-----------------------------------------------------------------------------
-- |
-- Module      :  GHC.Types
-- Copyright   :  (c) The University of Glasgow 2009
-- License     :  see libraries/ghc-prim/LICENSE
--
-- Maintainer  :  cvs-ghc@haskell.org
-- Stability   :  internal
-- Portability :  non-portable (GHC Extensions)
--
-- GHC type definitions.
-- Use GHC.Exts from the base package instead of importing this
-- module directly.
--
-----------------------------------------------------------------------------

module GHC.Types (
        -- Data types that are built-in syntax
        -- They are defined here, but not explicitly exported
        --
        --    Lists:          []( [], (:) )
        --    Type equality:  (~)( Eq# )

        Bool(..), Char(..), Int(..),
        -- isTrue#,
        -- TYPE, RuntimeRep(..), Type, Constraint,
        --   -- The historical type * should ideally be written as
        --   -- `type *`, without the parentheses. But that's a true
        --   -- pain to parse, and for little gain.
        -- VecCount(..), VecElem(..),
        --
        -- -- * Runtime type representation
        -- Module(..), TrName(..), TyCon(..), TypeLitSort(..),
        -- KindRep(..), KindBndr,
        -- module GHC.Types
    ) where

import GHC.Prim

infixr 5 :

{- *********************************************************************
*                                                                      *
                  Kinds
*                                                                      *
********************************************************************* -}

-- | The kind of constraints, like @Show a@
data Constraint

-- | The kind of types with values. For example @Int :: Type@.
type Type = TYPE 'LiftedRep

data [] a = [] | a : [a]

-- data Ordering = LT | EQ | GT

data {-# CTYPE "HsChar" #-} Char = C# Char#

data {-# CTYPE "HsInt" #-} Int = I# Int#

data {-# CTYPE "HsBool" #-} Bool = False | True

{-# INLINE isTrue# #-}
isTrue# :: Int# -> Bool
isTrue# x = tagToEnum# x

{- *********************************************************************
*                                                                      *
                    Levity polymorphism
*                                                                      *
********************************************************************* -}


-- | GHC maintains a property that the kind of all inhabited types
-- (as distinct from type constructors or type-level data) tells us
-- the runtime representation of values of that type. This datatype
-- encodes the choice of runtime value.
-- Note that 'TYPE' is parameterised by 'RuntimeRep'; this is precisely
-- what we mean by the fact that a type's kind encodes the runtime
-- representation.
--
-- For boxed values (that is, values that are represented by a pointer),
-- a further distinction is made, between lifted types (that contain ⊥),
-- and unlifted ones (that don't).
data RuntimeRep = VecRep VecCount VecElem   -- ^ a SIMD vector type
                | TupleRep [RuntimeRep]     -- ^ An unboxed tuple of the given reps
                | SumRep [RuntimeRep]       -- ^ An unboxed sum of the given reps
                | LiftedRep       -- ^ lifted; represented by a pointer
                | UnliftedRep     -- ^ unlifted; represented by a pointer
                | IntRep          -- ^ signed, word-sized value
                | WordRep         -- ^ unsigned, word-sized value
                | Int64Rep        -- ^ signed, 64-bit value (on 32-bit only)
                | Word64Rep       -- ^ unsigned, 64-bit value (on 32-bit only)
                | AddrRep         -- ^ A pointer, but /not/ to a Haskell value
                | FloatRep        -- ^ a 32-bit floating point number
                | DoubleRep       -- ^ a 64-bit floating point number

-- See also Note [Wiring in RuntimeRep] in TysWiredIn

-- | Length of a SIMD vector type
data VecCount = Vec2
              | Vec4
              | Vec8
              | Vec16
              | Vec32
              | Vec64
-- Enum, Bounded instances in GHC.Enum

-- | Element of a SIMD vector type
data VecElem = Int8ElemRep
             | Int16ElemRep
             | Int32ElemRep
             | Int64ElemRep
             | Word8ElemRep
             | Word16ElemRep
             | Word32ElemRep
             | Word64ElemRep
             | FloatElemRep
             | DoubleElemRep
-- Enum, Bounded instances in GHC.Enum

data Module = Module
                TrName   -- Package name
                TrName   -- Module name

data TrName
  = TrNameS Addr#  -- Static
  | TrNameD [Char] -- Dynamic

-- | A de Bruijn index for a binder within a 'KindRep'.
type KindBndr = Int

#define WORD64_TY Word64#

-- | The representation produced by GHC for conjuring up the kind of a
-- 'TypeRep'.  See Note [Representing TyCon kinds: KindRep] in TcTypeable.
data KindRep = KindRepTyConApp TyCon [KindRep]
             | KindRepVar !KindBndr
             | KindRepApp KindRep KindRep
             | KindRepFun KindRep KindRep
             | KindRepTYPE !RuntimeRep
             | KindRepTypeLitS TypeLitSort Addr#
             | KindRepTypeLitD TypeLitSort [Char]

-- wKindRepVar = KindRepVar

data TypeLitSort = TypeLitSymbol
                 | TypeLitNat

-- Show instance for TyCon found in GHC.Show
data TyCon = TyCon WORD64_TY WORD64_TY   -- Fingerprint
                   Module                -- Module in which this is defined
                   TrName                -- Type constructor name
                   Int#                  -- How many kind variables do we accept?
                   KindRep               -- A representation of the type's kind
