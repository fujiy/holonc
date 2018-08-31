{-# LANGUAGE LambdaCase, TupleSections #-}


module Holon.Compiler.CodeGen where

-- import Data.Char
import Data.Bits
import Data.List
import Data.Maybe
-- import Data.Either
import           Data.Tuple
import qualified Data.Text as Text
import           Data.Map (Map)
import qualified Data.Map as Map
import           Data.Set (Set)
import qualified Data.Set as Set
import           Data.ByteString (ByteString)
import qualified Data.ByteString as BS
import Data.Word
import Data.Default
import Data.Int
-- import qualified Data.ByteString as BS
-- import Data.ByteString (pack)
-- import Data.ByteString.Builder
-- import Data.ByteString.Lazy (unpack)

import Data.Binary.IEEE754
import Control.Applicative
import Control.Monad
import Control.Monad.Reader
import Control.Monad.State
import Control.Monad.Except
import Control.Monad.Extra
import Lens.Micro
import Lens.Micro.Mtl

import Debug.Trace
import Data.Data
import Data.ProtoLens.TextFormat

import Util
import Avail
import HscTypes
import StgSyn
import Module
import Var
import Name hiding (varName)
import DataCon
import CoreSyn (AltCon(..))
import Literal
import Outputable
import DynFlags
import PrelNames
import Unique
import Type
import TyCon
import TysWiredIn
import IdInfo
import PrimOp
import IfaceSyn
import UniqDFM
import InstEnv

import Holon.Compiler.StgLift

import qualified Proto.Module as M
import qualified Proto.Module_Fields as M
import qualified Proto.Expr   as E
import qualified Proto.Instr  as I
import qualified Proto.Prim   as P
-- import Proto.Instr_Fields as I

type HWord   = Word64
type Operand = Word16
type Payload = HWord
type Ref     = HWord  -- Global object reference
type Layout  = Word32 -- Layout bitmap
-- type Cr      = HWord

type CP      = HWord -- Code pointer

type Sr      = Word16 -- Stack relative address
type Mr      = Word32 -- Module reference
type Pr      = Word32 -- Package reference

-- data Payload = St Mr Ref -- Static object reference (module, symbol)
--              | Im HWord  -- Immidiate

-- data CodeRef = Cr Mr CP -- (module, symbol)

type Mods = Map ModuleName Mr

type GlobalGen = ReaderT GlobalEnv (Except GenError)
data GlobalEnv = GE { gcafs  :: Map Var Ref
                    , gfuncs :: Map Var CP
                    } deriving Show

type LocalGen = StateT LocalEnv GlobalGen
type LocalEnv = (Map Var Sr, [[Var]])

data GenError = UnboundName         String
              | UnboundLocalName    String
              | UnboundGlobalName   String
              | UnboundFuncName     String
              | UnboundPrimName     String
              | UnboundExternalName String
              | UnknownPackage      String
              | UnknownModule       String
              | MultipleError [GenError]
              | NoGenError
    deriving Show
instance Semigroup GenError where
    MultipleError as <> MultipleError bs = MultipleError (as ++ bs)
    MultipleError as <> b                = MultipleError (as ++ [b])
    a                <> MultipleError bs = MultipleError (a : bs)
    a                <> b                = MultipleError [a, b]
instance Monoid GenError where
    mempty = NoGenError


type GenEnv = Map Module (Pr, Mr, Map Name Ref)

-- packSt :: St -> HWord
-- packSt (x, y) = fi x .&. shift (fi y) 32

-- class Bytes a where
--     bytes :: a -> [Word8]
-- instance Bytes Word8 where
--     bytes w = [w]
-- instance Bytes Word16 where
--     bytes = unpack . toLazyByteString . word16LE
-- instance Bytes Word32 where
--     bytes = unpack . toLazyByteString . word32LE
-- instance Bytes Word64 where
--     bytes = unpack . toLazyByteString . word64LE
--
-- bytess :: Bytes a => [a] -> [Word8]
-- bytess = concatMap bytes


localStateT :: Monad m => (s -> s) -> StateT s m a -> StateT s m a
localStateT f m = StateT $ \s -> do
    (a, _) <- runStateT m $ f s
    return (a, s)

genLocal :: Srt -> LocalGen a -> GlobalGen a
genLocal srt m = evalStateT m (Map.fromList $ zip srt [0..], [srt])

addLocal :: [Var] -> LocalGen ()
addLocal vs = modify $ \(m, xs:xss) ->
    (Map.union m $ Map.fromList (zip vs [msize m..]), (vs ++ xs):xss)

localRef :: Var -> LocalGen Sr
localRef n = gets (Map.lookup n . fst)
         >>= maybe (throwError . UnboundLocalName $ showO n) return

cafRef :: Var -> GlobalGen Ref
cafRef x = (asks (Map.lookup x . gcafs)
       >>= maybe (throwError . UnboundGlobalName $ showO x) return)
       <|> case varName x of
           n | occNameFS (occName n) == occNameFS (occName nilDataConName)
                                               -> return 110000
           n | n == starKindRepName               -> return 110001
           n | n == starArrStarKindRepName        -> return 110002
           n | n == starArrStarArrStarKindRepName -> return 110003
           n | n == listTyConName                 -> return 110004
           _ -> throwError . UnboundGlobalName $ showO x

funcRef :: Var -> GlobalGen CP
-- funcRef x | getOccString x == "wKindRepVar"
--           = funcRef $ setVarName x kindRepVarDataConName
funcRef x = asks (Map.lookup x . gfuncs)
       >>= maybe (throwError . UnboundFuncName $ showO x) return

genBlock :: LocalGen a -> LocalGen (a, [Var])
genBlock gen =
    localStateT (\(m, vs) -> (m, []:vs)) $ do
        a <- gen
        (_, vs:_) <- get
        return (a, vs)


codeGen :: ExternalPackageState -> [(ModIface, [StgTopBinding])]
         -> Except GenError [(Module, M.HObject)]
codeGen eps mods =
    let home :: GenEnv
        home = Map.fromList $ map (\(x, (mi, _)) ->
                (mi_module mi, (0, x, Map.fromList $ zip (ifaceExports mi) [0..])))
             $ zip [0..] mods

        prm  :: Map UnitId Pr     -- Package references
        mrm  :: Map ModuleName Mr -- Module references
        ((prm, mrm), ml) = mapAccumL (\(pm, mm) mi ->
                let mo         = mi_module mi
                    (mpr, pm') = Map.insertLookupWithKey (\_ _ a -> a)
                                 (moduleUnitId mo) (msize pm) pm
                    (mmr, mm') = Map.insertLookupWithKey (\_ _ a -> a)
                                 (moduleName mo) (msize mm) mm
                    pr         = fromMaybe (msize pm) mpr
                    mr         = fromMaybe (msize mm) mmr
                    exps       = Map.fromList $ zip
                                 (concatMap availNames $ mi_exports mi) [0..]
                in  ((pm, mm), (mo, (pr, mr, exps))))
               ( Map.singleton (moduleUnitId . mi_module . fst $ head mods) 0
                           :: Map UnitId Pr
               , Map.empty :: Map ModuleName Mr )
             $ moduleEnvElts (eps_PIT eps)

        ext  :: GenEnv
        ext  = Map.fromList ml

        genv :: GenEnv
        genv = foldl' (\mods ci ->
                let n  = varName $ is_dfun ci
                in  Map.update (\(pr, mr, m) ->
                    Just (pr, mr, Map.insert n (msize m) m))
                    (nameModule n) mods)
               (home `Map.union` ext)
             $ instEnvElts $ eps_inst_env eps
    in  mapM (uncurry $ genModule genv) mods

depModsPkgs :: Dependencies -> (Map UnitId Pr, Map ModuleName Mr)
depModsPkgs deps =
    ( Map.fromList $ zip
        (map (DefiniteUnitId . DefUnitId . fst) $ dep_pkgs deps) [0..]
    , Map.fromList $ zip
        (map fst $ dep_mods deps) [1..] )

genModule :: GenEnv -> ModIface -> [StgTopBinding] -> Except GenError (Module, M.HObject)
genModule gem mi bs = do
    let ls    :: [(Srt, Var, StgRhs, HasCode)] -- Lifted Stgs
        ls    = concatMap lambdaLift bs

        srs   :: [Var]                         -- static references
        srs   = concatMap (\(a,_,_,_) -> a) ls

        exps  :: [Name]                        -- Export names
        exps  = ifaceExports mi

        exts  :: Map Var Ref                   -- External names
        exts  = fst $ foldr (\s (m, x) ->
                let (ms, m') = Map.insertLookupWithKey (\_ a _ -> a) s x m
                    mn       = nameModule_maybe (varName s)
                in  if isJust mn && mn /= Just (mi_module mi) && isNothing ms
                    then (m', x + 1) else (m, x))
                (Map.empty, 0)
                $ srs ++ map (\n -> mkGlobalVar VanillaId n anyTy vanillaIdInfo) exps

        cafs  :: Map Var Ref                   -- Local constant exprs
        cafs  = Map.union exts $
                Map.fromList $ zip (map (\(_,x,_,_) -> x) ls)
                [msize exts..]

        funcs :: Map Var CP                    -- Local Functions
        funcs = Map.union exts $
                Map.fromList $ zip (foldr (\(_,x,_,hc) cs ->
                    if hc then x:cs else cs) [] ls)
                [msize exts..]

        ge    :: GlobalEnv
        ge    = GE cafs funcs

        cafexps  :: [Ref]
        cafexps  = map (flip (Map.findWithDefault 99001) $ Map.mapKeys varName cafs)  exps

        codeexps :: [CP]
        codeexps = map (flip (Map.findWithDefault 99003) $ Map.mapKeys varName funcs) exps

    -- Import symbols
    imps <- Map.fromList . zip [0..] <$> forM (Map.keys exts) (\v -> do
        let mo = nameModule $ varName v
        (pr, mr, m) <- maybe (throwError . UnknownPackage $ showO $ moduleUnitId mo)
                       return $ gem Map.!? mo
        x           <- maybe (throwError . UnboundExternalName $ showO v)
                       return $ m Map.!? varName v
        return $ M.Symbol (fi pr) (fi mr) x [])

    (es, cs) <- runReaderT (foldM goGen ([], []) ls) ge

    return (mi_module mi, def
        -- $ traceShow ge $ traceShow (exps, cafs, funcs)
            -- & M.name        .~ Text.pack (moduleNameString . moduleName $ mg_module mg)
        & M.name        .~ Text.pack (moduleNameString (moduleName $ mi_module mi))
        & M.localOffset .~ fi (length exts)
        & M.imports     .~ imps
        & M.cafExports  .~ cafexps
        & M.codeExports .~ codeexps
        & M.cafs        .~ reverse es
        & M.text        .~ reverse cs)
    where
        goGen :: ([E.Expr], [I.Func]) -> (Srt, Var, StgRhs, HasCode) -> GlobalGen ([E.Expr], [I.Func])
        goGen (es, cs) (srt, x, rhs, hc) = do
            (x, ei) <- genBind srt x rhs
            -- traceShow (x, either showMessage showMessage ei) return ()
            e <- either return (const $ expr E.FUNC . (: []) <$> funcRef x) ei
            if hc then let Right c = ei
                       in  return (e:es, c:cs)
                  else return (e:es, cs)

ifaceExports :: ModIface -> [Name]
ifaceExports mi = concatMap availNames (mi_exports mi)
               ++ map ifDFun (mi_insts mi)


genBind :: Srt -> Var -> StgRhs -> GlobalGen (Var, Either E.Expr I.Func)
genBind srt x = \case
    StgRhsClosure _ _ _ _ [] (StgApp f []) -> do
        cp <- funcRef f
        return (x, Right $ func x 0 []
                           [block [] [instr I.JUMP $ splitCP cp]])
    StgRhsClosure _ _ _ _ [] a -> do
        e <- genExpr a
        return (x, Left e)
    StgRhsClosure _ _ _ _ ps a -> do
        -- c <- instr I.SREF (length srt)
        --     <$> concatMapM (fmap splitRef . cafRef) srt
         -- <$> concatMapM (fmap (\(St x y) -> [x, y]) . cafRef) srt
        -- cs <- flip runReaderT (initEnv srt)
        --     $ localBinds ps $ genExprCode True a
        -- return (x, Right $ I.Code $ if null srt then cs else c : cs)
        sr <- mapM cafRef srt
        ((cs, bls), vs) <- genLocal srt $ genBlock $ do
            addLocal ps
            genExprCode True a
        return (x, Right $ func x (len ps) sr $ block vs cs : bls)
    StgRhsCon _ccs dc args -> do
        as <- mapM genArgExpr args
        return (x, Left $ exprC E.CONSTR (dataConTagZ dc) 0 as)

-- initEnv :: Srt -> LocalEnv
-- initEnv srt = Map.fromList $ zip srt [0..]

genExpr :: StgExpr -> GlobalGen E.Expr
genExpr = \case
    StgApp f [] -> (do
        cp <- funcRef f
        return $ expr E.FUNC [cp])
        <|> (do
        r <- cafRef f
        return $ expr E.INDIR [r])
    StgApp f args -> (do
        cp <- funcRef f
        rs <- mapM genArgExpr args
        return $ expr' E.THUNK_F 0 $ fi cp : rs)
        <|> (do
        rc <- cafRef f
        rs <- mapM genArgExpr args
        return $ expr' E.THUNK_C 0 $ rc : rs)

genExprCode :: Bool -- ^ ? return : push
            -> StgExpr -> LocalGen ([I.Instr], [I.Block])
genExprCode rt = \case
    -- StgApp f _ | last () == '#' -> do
    --     p <- lift $ packPrim <$> genPrim (occNameString . occName $ varName f)
    --     return [instr I.PRIMOP 1 $ splitCP cp])
    StgApp f [] -> (do
        cp <- lift $ funcRef f
        return ([instr (retOp rt I.FUNC_P) $ splitCP cp], []))
        <|> (do
        r <- localRef f
        return ([instr (retOp rt I.PUSH) [r]], []))
    StgApp f args -> (do
        cp <- lift $ funcRef f
        rs <- mapM genArg args
        -- trace "pf: " $ traceShow f $ traceShow (map showO args) $ trace (showO $ splitFunTys $ varType f) return ()
        return ([instr (retOp rt I.THUNK_F_P) $ splitCP cp ++ rs], []))
        <|> (do
        rf <- localRef f
        rs <- mapM genArg args
        return ([instr (retOp rt I.THUNK_C_P) $ rf : rs], []))

    -- StgLit l -> do
    --     genLit l

    StgConApp dc as _ -> do
        rs <- mapM genArg as
        return ([instr (retOp rt I.CONSTR_P) $ fi (dataConTagZ dc) : rs], [])

    StgOpApp (StgPrimOp op) as _ -> do
        rs <- mapM genArg as
        return ([instr (retOp rt I.PRIMOP_P) $ packPrim (prim op) : rs], [])

    StgCase a x _ [(DEFAULT, _, b)] -> do
        (ais, als) <- genExprCode False a
        addLocal [x]
        (bis, bls) <- genExprCode rt b
        return (ais ++ bis, als ++ bls)
    StgCase a x (AlgAlt t) as -> do
        (is, bs) <- genExprCode False a
        addLocal [x]
        bss <- mapM (genAlt rt) as
        let acs  = map (\(x,_,_) -> x) as
            ats  = altTable (length $ tyConDataCons t) acs
            bss' = if head acs == DEFAULT then bss
                   else [block [] [instr (retOp rt I.BOTTOM_P) []]] : bss
            ofs  = scanl (+) 0 $ map length (init bss') -- offsets
            jt   = map (ofs !!) ats  -- jump table
        return (is ++ [instr (retOp rt I.MATCH_A_E_P) $ map fi jt],
                bs ++ concat bss)

    StgLet (StgNonRec x rhs) a -> do
        is <- case rhs of
            StgRhsClosure _ _ _ _ [] b -> case b of
                StgApp f args -> (do
                    cp <- lift $ funcRef f
                    rs <- mapM genArg args
                    return [instr I.THUNK_F_P $ splitCP cp ++ rs])
                    <|> (do
                    rf <- localRef f
                    rs <- mapM genArg args
                    return [instr I.THUNK_C_P $ rf : rs])
                _ -> error $ showO b
            StgRhsCon _ dc as -> do
                rs <- mapM genArg as
                return [instr I.CONSTR_P $ fi (dataConTagZ dc) : rs]
        addLocal [x]
        (is', bls) <- genExprCode rt a
        return (is ++ is', bls)
    e -> error $ showO e

-- genPushExpr :: StgExpr -> LocalGen [I.Instr]

genAlt :: Bool -- ^ ? return : push
       -> StgAlt -> LocalGen [I.Block]
genAlt rt (_, ps, b) = do
    ((cs, bls), vs) <- genBlock $ do
        addLocal ps
        genExprCode rt b
    return $ block vs cs : bls

    -- localBinds ps (genExprCode rt b)

altTable :: Int -> [AltCon] -> [Int]
altTable n = go n 0 1
    where
        go :: Int -> Int -> Int -> [AltCon] -> [Int]
        go m n s [] | m < n     = []
                  | otherwise = 0 : go m (n+1) s []
        go m n s (a:as) = case a of
            DEFAULT -> 0 : go m (n+1) s as
            DataAlt dc
                | dataConTag dc == n -> s : go m (n+1) (s+1) as
                | otherwise          -> 0 : go m (n+1) s (a:as)

-- altTable n as =
--     let xs = foldr (\ac (x, ns) -> case ac of
--                     DEFAULT -> (x, 0:ns)
--                     DataAlt dc
--                         | dataConTagZ dc == x -> (x-1, x:ns)
--                         | otherwise           -> (x-1, x:)
--
--                                   in  ) n as

genArg :: StgArg -> LocalGen Sr
genArg = \case
    StgVarArg x -> localRef x
    StgLitArg l -> return $ case l of
        MachInt i   -> fi i
        MachInt64 i -> fi i
        MachStr xs  -> 65001

genArgExpr :: StgArg -> GlobalGen Payload
genArgExpr = \case
    StgVarArg x -> cafRef x
    StgLitArg l -> return $ genLit l

genLit :: Literal -> Payload
genLit = \case
    MachChar c   -> fi $ fromEnum c
    MachStr xs   -> 91001
    MachInt i    -> fi i
    MachInt64 i  -> fi i
    MachWord i   -> fi i
    MachWord64 i -> fi i
    MachFloat r  -> fi $ floatToWord $ realToFrac r
    MachDouble r -> doubleToWord $ realToFrac r

prim :: PrimOp -> P.HPrimOp
prim = \case
    IntAddOp    -> P.INTADD
    IntSubOp    -> P.INTSUB
    IntMulOp    -> P.INTMUL
    IntNegOp    -> P.INTNEG
    IntGtOp     -> P.INTGT
    IntGeOp     -> P.INTGE
    IntEqOp     -> P.INTEQ
    IntNeOp     -> P.INTNE
    IntLtOp     -> P.INTLT
    IntLeOp     -> P.INTLE
    TagToEnumOp -> P.TAGTOENUM
    op  -> panic $ "Unknown PrimOp: " ++ showO op


packPrim :: P.HPrimOp -> Operand
packPrim = fi . fromEnum

-- genPushCArg :: StgArg -> Gen I.Code
-- genPushCArg = \case
--     StgVarArg x -> (code C.PUSH_R_C <$> lookupLocal  (vn x))
--                <|> (code C.PUSH_S_C <$> lookupGlobal (vn x))

-- genPushLiteral :: Literal -> I.Code

-- localRef :: [Id] -> Id -> HWord
-- localRef ids x = fi . fromMaybe 90001 $ elemIndex x ids

-- genSRef :: Gen Instr
-- genSRef = do
--     rs <- lift . gets $ map fst . sort . map swap . Map.toList . srefs
--     return $ instr I.SREF (length rs) $ bytess rs

-- indir :: Payload -> Expr
-- -- indir p = expr (Expr'Indir $ IndirHeader []) [p]
-- indir p = expr  []
--
-- constr :: Int -> [Payload] -> Expr
-- constr k as = expr (Expr'Constr $ ConstrHeader (fi $ length as) (fi k) []) as
--
-- func :: CP -> Expr
-- func cp = expr (Expr'Func $ FuncHeader (fi cp) []) []
--
-- thunkF :: CP -> [Payload] -> Expr
-- thunkF cp as = expr (Expr'ThunkF $ ThunkFHeader (fi $ length as) (fi cp) []) $ map packSt as
--
-- thunkC :: Payload -> [Payload] -> Expr
-- thunkC fr as = expr (Expr'ThunkC $ ThunkCHeader (fi $ length as) []) $ map packSt (fr : as)

-- code :: Instr -> HWord -> I.Code
-- code i w = C.Code i w []

isPointer :: Type -> Bool
isPointer = fromMaybe False . isLiftedType_maybe

-- pointerFirst :: [Bool] -- ^ is pointer
--              -> [a] -> [a]
-- pointerFirst ps = go ps []
--     where
--         go :: [Bool] -> [a] -> [a] -> [a]
--         go [] xs _ = reverse xs
--         go (p:ps) xs (a:as) | p
--                    = a : go ps xs as
--         go (_:ps) xs (a:as)
--                    = go ps (a:xs) as
--
-- pfVars :: [Var] -> [Var]
-- pfVars vs = pointerFirst (map (isPointer . varType) vs) vs
--
-- pfArgs :: [StgArg] -> [StgArg]
-- pfArgs as = pointerFirst (map (\case StgVarArg x -> varType x) as) as
--
-- pfDC :: DataCon -> [a] -> [a]
-- pfDC = pointerFirst . dataConRepArgTys
--
-- pfFun :: Var -> [a] -> [a]
-- pfFun v = pointerFirst . fst . splitFunTys $ varType v
--
-- pointerNum :: [Type] -> Int
-- pointerNum = foldr (\t x -> if fromMaybe False $ isLiftedType_maybe t
--                             then x + 1 else x) 0
--
-- pnDC :: DataCon -> Int
-- pnDC = pointerNum . dataConRepArgTys
--
-- pnFun :: Var -> Int
-- pnFun = pointerNum . fst . splitFunTys . varType

stackLayout :: [Var] -> ByteString
stackLayout vs = BS.pack . map listBitmap . chunkList 8
               $ map (isPointer . varType) vs

listBitmap :: [Bool] -> Word8
listBitmap = foldr (\(i, b) w -> if b then setBit w i else w) 0 . zip [0..]

block :: [Var] -> [I.Instr] -> I.Block
block vs is = I.Block (stackLayout vs) is []

func :: Var -> Int -> [Ref] -> [I.Block] -> I.Func
func var arity srt text =
    I.Func (fi arity) srt text
           (Text.pack . occNameString $ occName var) []

-- code :: Int32 -> [Var] -> [Ref] -> [I.Instr] -> I.Code
-- code n xs srt cs = I.Code n BS.empty srt cs []

-- pointerFirst :: [Payload] -> [HWord]
-- pointerFirst = map (\case
--     St x y -> fi x .&. shift (fi y) 32
--     Im x   -> x ) . sortBy comp
--     where
--         comp (St _ _) (Im _) = GT
--         comp (Im _) (St _ _) = LT
--         comp _        _      = EQ


-- pointerNum :: [Payload] -> Int
-- pointerNum = foldr (\p n ->
--     case p of St _ _ -> n + 1
--               _      -> n) 0

retOp :: Bool -> I.Opcode -> I.Opcode
retOp rt op | rt        = succ op
            | otherwise = op

opers :: [Operand] -> [Word32]
opers = map fi

instr :: I.Opcode -> [Operand] -> I.Instr
instr op bs = I.Instr op 0 (opers bs) []

instr' :: I.Opcode -> Int -> [Operand] -> I.Instr
instr' op pn bs = I.Instr op (fi pn) (opers bs) []

-- expr :: Expr'Header -> [Payload] -> Expr
-- expr h ps = E.Expr ps (Just h) []

expr :: E.ExprType -> [Payload] -> E.Expr
expr t as = E.Expr t 0 0 as []

expr' :: E.ExprType -> Int -> [Payload] -> E.Expr
expr' t pn as = E.Expr t (fi pn) 0 as []

-- expr' :: E.ExprType -> Int -> [Payload] -> E.Expr
-- expr' t n as = E.Expr t (fi $ length as) (fi n) 0 as []

exprC :: E.ExprType -> Int -> Int -> [Payload] -> E.Expr
exprC t pn k as = E.Expr t (fi pn) (fi k) as []

-- exprC :: E.ExprType -> Int -> Int -> [Payload] -> E.Expr
-- exprC t n x as = E.Expr t (fi $ length as) (fi n) (fi x) as []

-- code :: [I.Instr] -> I.Code
-- code is = I.Code is []

-- opCP :: CP -> [Operand]
-- opCP cp = [fi cp, fi (shiftR cp 32)]

splitRef :: Ref -> [Word32]
splitRef x = [fi x, fi (shiftR x 32)]

splitCP :: CP -> [Operand]
splitCP x = [fi x, fi (shiftR x 32)]

-- kindRepVarDataCon :: Var
-- kindRepVarDataCon = mkGlobalVar VanillaId kindRepVarDataConName undefined

msize :: Num b => Map k a -> b
msize = fi . Map.size

len :: Num b => [a] -> b
len = fromIntegral . length

fi :: (Integral a, Num b) => a -> b
fi = fromIntegral

-- localStateT :: Monad m => (s -> s) -> StateT s m a -> StateT s m a
-- localStateT f m = do
--     s <- get
--     a <- withStateT f m
--     put s
--     return a

showO :: Outputable a => a -> String
showO = showSDocUnsafe . ppr

instance Show Name where
    show = showO

instance Show OccName where
    show = showO

instance Show Var where
    show = showO

instance Show ModuleName where
    show = showO
