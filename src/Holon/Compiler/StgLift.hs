{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE TupleSections #-}

module Holon.Compiler.StgLift
    ( Srt
    , HasCode
    , lambdaLift
    -- , vn
    -- , vns
    ) where

import Data.Maybe
import Data.Set (Set)
import qualified Data.Set as Set
import Control.Monad.Reader
import Control.Monad.State
import Control.Monad.Extra

import CostCentre
import StgSyn
import Var
import Name hiding (varName)
import Unique
import Type
import TyCon
import PrelNames
import TysWiredIn

import Debug.Trace
import Outputable

type Srt     = [Var] -- Static reference table
type HasCode = Bool  -- Whether expr has code

type Lifter = StateT [(Srt, Var, StgRhs, HasCode)] (Reader (Set Var))

localBinds :: [Var] -> Lifter a -> Lifter a
localBinds ns = local $ Set.union $ Set.fromList ns

addBind :: Srt -> Var -> StgRhs -> HasCode -> Lifter ()
addBind srt x rhs hc = modify ((srt, x, rhs, hc) :)

-- Lambda lifting and list static references
lambdaLift :: StgTopBinding -> [(Srt, Var, StgRhs, HasCode)]
lambdaLift = \case
    StgTopLifted b ->
        let (b', bs) = runReader (runStateT (liftBinding b) []) Set.empty
        in  bs ++ b'

liftBinding :: StgBinding -> Lifter [(Srt, Var, StgRhs, HasCode)]
liftBinding = \case
    StgNonRec x rhs ->
        let tn = maybe wildCardName tyConName $ tyConAppTyCon_maybe $ varType x
            ta = maybe wildCardName tyConName $ tyConAppArgs_maybe (varType x) >>= listToMaybe >>= tyConAppTyCon_maybe
        in  if tn `elem` [typeRepTyConName, kindRepTyConName, trTyConTyConName]
            || ta `elem` [kindRepTyConName]
            then return []
            else (: []) <$> liftRhs x rhs
    StgRec bs -> localBinds (map fst bs) $ mapM (uncurry liftRhs) bs

liftRhs :: Var -> StgRhs -> Lifter (Srt, Var, StgRhs, HasCode)
liftRhs x rhs = case rhs of
    StgRhsClosure _ _ _ _ [] a@(StgApp f []) -> do
        (srt, a') <- liftExpr a
        return (srt, x, updateClosureBody rhs a', True)
    StgRhsClosure _ _ _ _ [] a -> do
        (srt, a') <- liftExpr a
        return (srt, x, updateClosureBody rhs a', False)
    StgRhsClosure _ _ _ _ ps a -> do
        (srt, a') <- localBinds ps $ liftExpr a
        return (srt, x, updateClosureBody rhs a', True)
    StgRhsCon _ _ as -> do
        srt <- concatMapM liftArg as
        return (srt, x, rhs, False)

liftExpr :: StgExpr -> Lifter (Srt, StgExpr)
liftExpr e = case e of
    StgApp f as -> do
        fsrt  <- listSrt [f]
        asrts <- mapM liftArg as
        return (fsrt ++ concat asrts, e)
    StgLit _ -> return ([], e)
    StgConApp dc as ts -> do
        asrts <- concatMapM liftArg as
        return (asrts, e)
    StgOpApp _ as _ -> do
        asrts <- concatMapM liftArg as
        return (asrts, e)
    StgCase a x t as -> do
        (asrt,  a')  <- liftExpr a
        (asrts, as') <- localBinds [x] $ unzip <$> mapM liftAlt as
        return (asrt ++ concat asrts,
                StgCase a' x t as')
    StgLet (StgNonRec x rhs) a -> case rhs of
        StgRhsClosure _ _ _ _ [] (StgLet bn c) ->
            liftExpr $ StgLet bn $ StgLet
                (StgNonRec x $ updateClosureBody rhs c) a
        StgRhsClosure _ _ _ _ [] ba -> do
            (bsrt, ba') <- liftExpr ba
            (asrt, a')  <- localBinds [x] $ liftExpr a
            return (bsrt ++ bsrt,
                    StgLet (StgNonRec x $ updateClosureBody rhs ba') a')
        StgRhsClosure _ _ fvs uf ps ba -> do
            (bsrt, ba') <- localBinds ps $ liftExpr ba
            let var = setVarUnique x $ newTagUnique (varUnique x) 'l'
                rhs = StgRhsClosure currentCCS noBinderInfo [] Updatable
                                    (fvs ++ ps) ba'
                pap = StgApp var $ map StgVarArg fvs
                cls = StgRhsClosure currentCCS noBinderInfo [] uf [] pap
            addBind bsrt var rhs True
            (asrt, a') <- localBinds [var] $ liftExpr a
            return (asrt, StgLet (StgNonRec x cls) a')
        StgRhsCon _ _ as -> do
            srt <- concatMapM liftArg as
            (asrt, a') <- localBinds [x] $ liftExpr a
            return (srt ++ asrt, StgLet (StgNonRec x rhs) a')
        --     in  ((srt, bind) : bs ++ bsa, srta, StgLet (StgNonRec x cls) a')
    _ -> error $ showO e

liftAlt :: StgAlt -> Lifter (Srt, StgAlt)
liftAlt (ac, xs, b) = do
    (bsrt, b') <- localBinds xs $ liftExpr b
    return (bsrt, (ac, xs, b'))

liftArg :: StgArg -> Lifter Srt
liftArg = \case
    StgVarArg x -> listSrt [x]
    a           -> return []

listSrt :: [Var] -> Lifter Srt
listSrt ns = asks
           $ filter (\x -> notElem (varName x) [starKindRepName,
                                                starArrStarKindRepName,
                                                starArrStarArrStarKindRepName]
                        && take 3 (getOccString x) /= "$tc")
           . Set.toList . Set.difference (Set.fromList ns)
{-$ filter ((/= "$WKindRepVar") . getOccString)-}

updateClosureBody :: StgRhs -> StgExpr -> StgRhs
updateClosureBody (StgRhsClosure ccs bi fvs uf ps _) =
    StgRhsClosure ccs bi fvs uf ps


-- vn :: Var -> Name
-- vn = varName
--
-- vns :: [Var] -> [Name]
-- vns = map vn

showO :: Outputable a => a -> String
showO = showSDocUnsafe . ppr
