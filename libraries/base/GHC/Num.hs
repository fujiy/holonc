
{-# LANGUAGE NoImplicitPrelude, MagicHash, UnboxedTuples #-}

-----------------------------------------------------------------------------
-- |
-- Module      :  GHC.Num
-- Copyright   :  (c) The University of Glasgow 1994-2002
-- License     :  see libraries/base/LICENSE
--
-- Maintainer  :  cvs-ghc@haskell.org
-- Stability   :  internal
-- Portability :  non-portable (GHC Extensions)
--
-- The 'Num' class and the 'Integer' type.
--
-----------------------------------------------------------------------------

module GHC.Num (module GHC.Num, module GHC.Integer) where

-- import Base
import GHC.Prim
import GHC.Base
import GHC.Integer

infixl 7  *
infixl 6  +, -

default ()

-- data Int = I# Int#

class Num a where
    {-# MINIMAL (+), (*), abs, signum, fromInteger, (negate | (-)) #-}

    (+), (-), (*) :: a -> a -> a
    negate        :: a -> a
    abs           :: a -> a
    signum        :: a -> a
    fromInteger   :: Integer -> a

    -- {-# INLINE (-) #-}
    -- {-# INLINE negate #-}
    -- x - y    = x + negate y
    -- negate x = 0 - x

{-# INLINE subtract #-}
subtract :: (Num a) => a -> a -> a
subtract x y = y - x

-- geInt :: Int -> Int -> Bool
-- geInt (I# x) (I# y) = isTrue# (x >=# y)
--
-- eqInt :: Int -> Int -> Bool
-- eqInt (I# x) (I# y) = isTrue# (x ==# y)
--
-- ltInt :: Int -> Int -> Bool
-- ltInt (I# x) (I# y) = isTrue# (x <# y)

instance  Num Int  where
    I# x + I# y = I# (x +# y)
    I# x - I# y = I# (x -# y)
    negate (I# x) = I# (negateInt# x)
    I# x * I# y = I# (x *# y)
    abs n  = if n `geInt` 0 then n else negate n

    signum n | n `ltInt` 0 = negate 1
             | n `eqInt` 0 = 0
             | otherwise   = 1

    {-# INLINE fromInteger #-}
    fromInteger i = I# (integerToInt i)

foo :: Int -> Int
foo n  = if n `geInt` 0 then n else negate n

-- instance  Num Integer  where
--     (+) = plusInteger
--     (-) = minusInteger
--     (*) = timesInteger
--     negate         = negateInteger
--     fromInteger x  =  x
--
--     abs = absInteger
--     signum = signumInteger
