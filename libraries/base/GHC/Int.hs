{-# LANGUAGE Trustworthy #-}
{-# LANGUAGE CPP, NoImplicitPrelude #-}

-----------------------------------------------------------------------------
-- |
-- Module      :  GHC.Int
-- Copyright   :  (c) The University of Glasgow 1997-2002
-- License     :  see libraries/base/LICENSE
--
-- Maintainer  :  cvs-ghc@haskell.org
-- Stability   :  internal
-- Portability :  non-portable (GHC Extensions)
--
-- The sized integral datatypes, 'Int8', 'Int16', 'Int32', and 'Int64'.
--
-----------------------------------------------------------------------------

module GHC.Int (
    Int(..),
    eqInt, neInt, gtInt, geInt, ltInt, leInt,
    ) where

import GHC.Base
import GHC.Num
