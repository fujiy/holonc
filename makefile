# proto:
# 	protoc --rust_out=../../runtime/src/protocol/ *.proto
# 	protoc --plugin=protoc-gen-haskell=`which proto-lens-protoc` --haskell_out=../../compiler/src/ *.proto
# 	protoc --doc_out=../ --doc_opt=markdown,doc.md *.proto

all:
	rm -rf haskell/*
	rm -rf rust/*
	cd src/; protoc --rust_out=../rust/ *.proto; protoc --plugin=protoc-gen-haskell=`which proto-lens-protoc` --haskell_out=../haskell/ *.proto; protoc --doc_out=../docs --doc_opt=markdown,proto.md *.proto;
