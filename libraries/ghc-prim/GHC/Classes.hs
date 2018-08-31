
{-# LANGUAGE NoImplicitPrelude, MagicHash #-}

module GHC.Classes (
    eqInt, neInt,
    gtInt, geInt, leInt, ltInt,
    ) where

import GHC.Prim
import GHC.Types

{-# INLINE [1] eqInt #-}
{-# INLINE [1] neInt #-}
eqInt, neInt :: Int -> Int -> Bool
(I# x) `eqInt` (I# y) = isTrue# (x ==# y)
(I# x) `neInt` (I# y) = isTrue# (x /=# y)

{-# INLINE [1] gtInt #-}
{-# INLINE [1] geInt #-}
{-# INLINE [1] ltInt #-}
{-# INLINE [1] leInt #-}
gtInt, geInt, ltInt, leInt :: Int -> Int -> Bool
(I# x) `gtInt` (I# y) = isTrue# (x >#  y)
(I# x) `geInt` (I# y) = isTrue# (x >=# y)
(I# x) `ltInt` (I# y) = isTrue# (x <#  y)
(I# x) `leInt` (I# y) = isTrue# (x <=# y)
