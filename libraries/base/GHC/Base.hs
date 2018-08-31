
{-# LANGUAGE CPP
           , NoImplicitPrelude
  #-}

{-# OPTIONS_GHC -Wno-orphans #-}

module GHC.Base (
    module GHC.Base,
    module GHC.Classes,
    module GHC.Types,
    -- module GHC.Prim,
    ) where


import GHC.Types
import GHC.Classes
import GHC.Prim

import GHC.Integer ()

-- |'otherwise' is defined as the value 'True'.  It helps to make
-- guards more readable.  eg.
--
-- >  f x | x < 0     = ...
-- >      | otherwise = ...
otherwise               :: Bool
otherwise               =  True
