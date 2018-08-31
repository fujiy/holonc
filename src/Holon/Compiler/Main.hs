module Holon.Compiler.Main (defaultMain) where

import           Control.Monad

import Data.ProtoLens.TextFormat

import Control.Monad.Except
import qualified Data.ByteString as BS
import Data.ProtoLens.Encoding
import System.Environment (getArgs)
import System.FilePath
import System.Directory

import qualified DynFlags
import           GHC       (Ghc)
import qualified GHC
import           GhcMonad
import           GHC.Paths (libdir)
import qualified HscMain
import HscTypes
import qualified CorePrep
import qualified CoreToStg
import qualified Digraph
import qualified SimplStg
import qualified TidyPgm
import qualified Module
import qualified MkIface
import qualified UniqDFM
import qualified BinIface
import           Outputable
import qualified Panic

import Avail

import Holon.Compiler.CodeGen

defaultMain :: IO ()
defaultMain = GHC.defaultErrorHandler DynFlags.defaultFatalMessager DynFlags.defaultFlushOut $ do
    args <- getArgs
    let ld = "/Users/yuuki.fj/.stack/programs/x86_64-osx/ghc-8.4.3/lib/holon-0.0.1"
        package = args !! 0
        version = args !! 1
        target  = args !! 2
    GHC.runGhc (Just ld) $ do
        liftIO $ print libdir
        dflags <- GHC.getSessionDynFlags
        GHC.setSessionDynFlags $ dflags
            { GHC.hscTarget    = DynFlags.HscNothing
            , GHC.ghcLink      = DynFlags.NoLink
            , GHC.importPaths  = ["src", "."]
            , GHC.includePaths = ["include"]
            , GHC.thisInstalledUnitId = Module.toInstalledUnitId $ Module.stringToUnitId package
            -- , GHC.packageFlags = [DynFlags.ExposePackage "package" (DynFlags.PackageArg "ghc-prim-0.5.2.0") $ DynFlags.ModRenaming True []]
            }
            -- { DynFlags.hscTarget = DynFlags.defaultObjectTarget (DynFlags.targetPlatform dflags)}
        dflags <- GHC.getSessionDynFlags
        target <- GHC.guessTarget target Nothing
        GHC.setTargets [target]

        -- GHC.load GHC.LoadAllTargets

        modGraph  <- GHC.depanal [] False
        let mods = Digraph.flattenSCCs $ GHC.topSortModuleGraph True modGraph Nothing

        -- printOutputable modGraph

        ms <- forM mods $ \msum -> do
            let mn = Module.moduleName $ ms_mod msum
            liftIO . putStrLn . Module.moduleNameString $ mn

            pmod  <- GHC.parseModule msum
            tmod  <- GHC.typecheckModule pmod
            tmod' <- GHC.loadModule tmod
            mguts <- GHC.coreModule <$> GHC.desugarModule tmod'
            env   <- GHC.getSession
            -- cguts <- liftIO $ HscMain.hscSimplify env mguts
            (cguts, md) <- liftIO $ TidyPgm.tidyProgram env mguts
            (prep, _)   <- liftIO $ CorePrep.corePrepPgm env (cg_module cguts) (ms_location msum) (cg_binds cguts) (cg_tycons cguts)
            (iface, _)  <- liftIO $ MkIface.mkIface env Nothing md mguts

            -- printOutputable $ cg_module tguts
            -- printOutputable $ cg_binds dmod
            -- printOutputable $ cg_binds tguts

            -- mapM_ printOutputable $ md_exports md

            -- printOutputable prep

            -- let stg = CoreToStg.coreToStg dflags (cg_module tguts) (cg_binds tguts)
            -- dmods <-
            let (stg, _) = CoreToStg.coreToStg dflags (cg_module cguts) prep
                path = ld </> package ++ "-" ++ version </> Module.moduleNameSlashes mn ++ ".hi"
                -- hmi      = HomeModInfo iface md Nothing

            printOutputable stg

            -- liftIO $ writeFile path ""
            liftIO . createDirectoryIfMissing True $ takeDirectory path
            liftIO $ BinIface.writeBinIface dflags path iface

            -- modifySession $ \e -> e{ hsc_HPT = addToHpt (hsc_HPT e) (Module.moduleName $ mg_module mguts) hmi }

            -- let m = runExcept $ codeGen {-(map fst . dep_mods $ mg_deps mguts)-} mis (mg_module mguts) md stg
            -- liftIO . putStrLn $ either show showMessage m

            return (iface, stg)

        env <- GHC.getSession
        pis <- liftIO $ Module.moduleEnvElts . eps_PIT <$> hscEPS env
        let his = map hm_iface . UniqDFM.eltsUDFM $ hsc_HPT env
            mis = pis ++ his

        -- mapM (printOutputable . map availName . mi_exports) mis
        -- mapM (printOutputable . map availNames . mi_exports) mis

        eps <- liftIO $ hscEPS env

        let r = runExcept $ codeGen eps ms

        liftIO $ either
            (panic . show)
            (mapM $ \(m, hm) -> do
                putStrLn $ showMessage hm
                BS.writeFile
                    (ld </> package ++ "-" ++ version
                        </> Module.moduleNameSlashes (Module.moduleName m) ++ ".ho")
                    $ encodeMessage hm
            ) r

        return ()

        -- forM_ hms $ \(m, eim) -> do
        --     liftIO $ either (panic . show) (putStrLn . unlines . map showMessage) eim

        -- forM_ ms $ \(iface, stg) -> do
        --     let m = runExcept $ codeGen mis iface stg
        --     liftIO $ either (panic . show) (putStrLn . showMessage) m

        -- mis <- UniqDFM.eltsUDFM . hsc_HPT <$> GHC.getSession
        -- mapM_ (printOutputable . mi_module . hm_iface) mis

        -- modSum <- GHC.getModSummary $ GHC.mkModuleName "App"
        -- pmod   <- GHC.parseModule modSum
        -- tmod   <- GHC.typecheckModule pmod
        -- dmod   <- GHC.desugarModule tmod
        -- let core = GHC.coreModule dmod
        -- prep <- liftIO $ corePrepPgm env (mg_module core) (ms_location modSum) (mg_binds core) (mg_tcs core)
        -- let stg  = coreToStg dflags (mg_module core) prep
        -- (stg_binds2, cost_centre_info) <- liftIO $ stg2stg dflags (mg_module core) stg
        --
        -- printOutputable modSum
        -- printOutputable $ parsedSource pmod
        -- printOutputable $ typecheckedSource tmod
        -- printOutputable $ modInfoTyThings $ moduleInfo tmod
        -- printOutputable $ mg_binds core
        -- liftIO $ putStrLn "=== STG ==="
        -- printOutputable stg
        -- liftIO . putStrLn . showGhc $ showPpr dflags stg_binds2
        -- return ()
    -- return ()

printOutputable :: Outputable a => a -> Ghc ()
printOutputable = liftIO . putStrLn . showGhc

showGhc :: Outputable a => a -> String
showGhc = showPpr DynFlags.unsafeGlobalDynFlags
