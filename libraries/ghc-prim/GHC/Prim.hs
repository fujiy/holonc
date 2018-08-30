
-----------------------------------------------------------------------------
-- |
-- Module      :  GHC.Prim
--
-- Maintainer  :  ghc-devs@haskell.org
-- Stability   :  internal
-- Portability :  non-portable (GHC extensions)
--
-- GHC's primitive types and operations.
-- Use GHC.Exts from the base package instead of importing this
-- module directly.
--
-----------------------------------------------------------------------------
{-# LANGUAGE Unsafe #-}
{-# LANGUAGE MagicHash #-}
{-# LANGUAGE MultiParamTypeClasses #-}
{-# LANGUAGE NoImplicitPrelude #-}
{-# LANGUAGE UnboxedTuples #-}
module GHC.Prim (

        Char#,

-- * Int#
-- |Operations on native-size integers (30+ bits).


        Int#,
        (+#),
        (-#),
        (*#),
        mulIntMayOflo#,
        quotInt#,
        remInt#,
        quotRemInt#,
        andI#,
        orI#,
        xorI#,
        notI#,
        negateInt#,
        addIntC#,
        subIntC#,
        (>#),
        (>=#),
        (==#),
        (/=#),
        (<#),
        (<=#),
        -- chr#,
        -- int2Word#,
        -- int2Float#,
        -- int2Double#,
        -- word2Float#,
        -- word2Double#,
        -- uncheckedIShiftL#,
        -- uncheckedIShiftRA#,
        -- uncheckedIShiftRL#,
) where

import GHC.Types
--
data Char#

gtChar# :: Char# -> Char# -> Int#
gtChar# = gtChar#

geChar# :: Char# -> Char# -> Int#
geChar# = geChar#

eqChar# :: Char# -> Char# -> Int#
eqChar# = eqChar#

neChar# :: Char# -> Char# -> Int#
neChar# = neChar#

ltChar# :: Char# -> Char# -> Int#
ltChar# = ltChar#

leChar# :: Char# -> Char# -> Int#
leChar# = leChar#

ord# :: Char# -> Int#
ord# = ord#

data Int#

infixl 6 +#
(+#) :: Int# -> Int# -> Int#
(+#) = (+#)

infixl 6 -#
(-#) :: Int# -> Int# -> Int#
(-#) = (-#)

-- |Low word of signed integer multiply.

infixl 7 *#
(*#) :: Int# -> Int# -> Int#
(*#) = (*#)

-- |Return non-zero if there is any possibility that the upper word of a
--     signed integer multiply might contain useful information. Return
--     zero only if you are completely sure that no overflow can occur.
--     On a 32-bit platform, the recommmended implementation is to do a
--     32 x 32 -> 64 signed multiply, and subtract result[63:32] from
--     (result[31] >>signed 31). If this is zero, meaning that the
--     upper word is merely a sign extension of the lower one, no
--     overflow can occur.
--     On a 64-bit platform it is not always possible to
--     acquire the top 64 bits of the result. Therefore, a recommended
--     implementation is to take the absolute value of both operands, and
--     return 0 iff bits[63:31] of them are zero, since that means that their
--     magnitudes fit within 31 bits, so the magnitude of the product must fit
--     into 62 bits.
--     If in doubt, return non-zero, but do make an effort to create the
--     correct answer for small args, since otherwise the performance of
--     @(*) :: Integer -> Integer -> Integer@ will be poor.
--

mulIntMayOflo# :: Int# -> Int# -> Int#
mulIntMayOflo# = mulIntMayOflo#

-- |Rounds towards zero.

quotInt# :: Int# -> Int# -> Int#
quotInt# = quotInt#

-- |Satisfies @(quotInt\# x y) *\# y +\# (remInt\# x y) == x@.

remInt# :: Int# -> Int# -> Int#
remInt# = remInt#

-- |Rounds towards zero.

quotRemInt# :: Int# -> Int# -> (# Int#,Int# #)
quotRemInt# = quotRemInt#

andI# :: Int# -> Int# -> Int#
andI# = andI#

orI# :: Int# -> Int# -> Int#
orI# = orI#

xorI# :: Int# -> Int# -> Int#
xorI# = xorI#

notI# :: Int# -> Int#
notI# = notI#

negateInt# :: Int# -> Int#
negateInt# = negateInt#

-- |Add signed integers reporting overflow.
--           First member of result is the sum truncated to an @Int#@;
--           second member is zero if the true sum fits in an @Int#@,
--           nonzero if overflow occurred (the sum is either too large
--           or too small to fit in an @Int#@).

addIntC# :: Int# -> Int# -> (# Int#,Int# #)
addIntC# = addIntC#

-- |Subtract signed integers reporting overflow.
--           First member of result is the difference truncated to an @Int#@;
--           second member is zero if the true difference fits in an @Int#@,
--           nonzero if overflow occurred (the difference is either too large
--           or too small to fit in an @Int#@).

subIntC# :: Int# -> Int# -> (# Int#,Int# #)
subIntC# = subIntC#

infix 4 >#
(>#) :: Int# -> Int# -> Int#
(>#) = (>#)

infix 4 >=#
(>=#) :: Int# -> Int# -> Int#
(>=#) = (>=#)

infix 4 ==#
(==#) :: Int# -> Int# -> Int#
(==#) = (==#)

infix 4 /=#
(/=#) :: Int# -> Int# -> Int#
(/=#) = (/=#)

infix 4 <#
(<#) :: Int# -> Int# -> Int#
(<#) = (<#)

infix 4 <=#
(<=#) :: Int# -> Int# -> Int#
(<=#) = (<=#)

-- |Shift left. Result undefined if shift amount is not
--           in the range 0 to word size - 1 inclusive.

uncheckedIShiftL# :: Int# -> Int# -> Int#
uncheckedIShiftL# = uncheckedIShiftL#

-- |Shift right arithmetic. Result undefined if shift amount is not
--           in the range 0 to word size - 1 inclusive.

uncheckedIShiftRA# :: Int# -> Int# -> Int#
uncheckedIShiftRA# = uncheckedIShiftRA#

-- |Shift right logical. Result undefined if shift amount is not
--           in the range 0 to word size - 1 inclusive.

uncheckedIShiftRL# :: Int# -> Int# -> Int#
uncheckedIShiftRL# = uncheckedIShiftRL#
