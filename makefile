# proto:
# 	protoc --rust_out=../../runtime/src/protocol/ *.proto
# 	protoc --plugin=protoc-gen-haskell=`which proto-lens-protoc` --haskell_out=../../compiler/src/ *.proto
# 	protoc --doc_out=../ --doc_opt=markdown,doc.md *.proto

out = "out"

doc:
	cd src/; protoc --doc_out=../docs --doc_opt=markdown,proto.md *.proto;

haskell:
	rm -rf haskell/*
	cd src/; protoc --plugin=protoc-gen-haskell=`which proto-lens-protoc` --haskell_out="../${out}" *.proto;

rust:
	rm -rf rust/*
	cd src/; protoc --rust_out="../${out}" *.proto;
