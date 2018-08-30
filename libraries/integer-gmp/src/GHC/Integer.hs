{-# LANGUAGE MagicHash #-}
{-# LANGUAGE NoImplicitPrelude #-}


-- |
-- Module      :  GHC.Integer.Type
-- Copyright   :  (c) Herbert Valerio Riedel 2014
-- License     :  BSD3
--
-- Maintainer  :  ghc-devs@haskell.org
-- Stability   :  provisional
-- Portability :  non-portable (GHC Extensions)
--
-- The 'Integer' type.
--
-- This module exposes the /portable/ 'Integer' API.  See
-- "GHC.Integer.GMP.Internals" for the @integer-gmp@-specific internal
-- representation of 'Integer' as well as optimized GMP-specific
-- operations.

module GHC.Integer (
    Integer,
    integerToInt
    ) where

import GHC.Integer.Type

default ()
