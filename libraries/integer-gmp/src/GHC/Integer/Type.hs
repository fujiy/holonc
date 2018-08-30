{-# LANGUAGE NoImplicitPrelude #-}
{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE CPP #-}
{-# LANGUAGE DeriveDataTypeable #-}
{-# LANGUAGE GHCForeignImportPrim #-}
{-# LANGUAGE MagicHash #-}
{-# LANGUAGE UnboxedTuples #-}
{-# LANGUAGE UnliftedFFITypes #-}
{-# LANGUAGE RebindableSyntax #-}
{-# LANGUAGE NegativeLiterals #-}
{-# LANGUAGE ExplicitForAll #-}

-- |
-- Module      :  GHC.Integer.Type
-- Copyright   :  (c) Herbert Valerio Riedel 2014
-- License     :  BSD3
--
-- Maintainer  :  ghc-devs@haskell.org
-- Stability   :  provisional
-- Portability :  non-portable (GHC Extensions)
--
-- GHC needs this module to be named "GHC.Integer.Type" and provide
-- all the low-level 'Integer' operations.

module GHC.Integer.Type where

-- #include "MachDeps.h"
-- #include "HsIntegerGmp.h"

-- import GHC.Classes
-- import GHC.Magic
import GHC.Prim
import GHC.Types

default ()

data Integer = S# !Int#

integerToInt :: Integer -> Int#
integerToInt (S# x) = x
-- integerToInt _      = 0#

mkInteger :: Bool -> [Int] -> Integer
mkInteger p (I# i:_) | p    = S# i
                     | True = S# (negateInt# i)
mkInteger _ _ = S# 0#
