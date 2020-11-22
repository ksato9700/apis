gendoc: v1/*.proto
	(cd v1; protoc --doc_out=../doc --doc_opt=markdown,index.md:./ *.proto)
