
all:
	make proto
	make stack

stack:
	stack build --color always

proto:
	git subtree pull --prefix=proto proto master --squash -m "proto"
	cd proto; make haskell out=../src/
