GRPC_GEN=python -m grpc_tools.protoc
OUTPUT_OPTS=--python_out=apis --grpc_python_out=apis

PROTOS_DIR=../v1
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
PB2S=$(patsubst $(PROTOS_DIR)/%.proto,apis/%_pb2.py,$(PROTOS))

apis/%_pb2.py:$(PROTOS_DIR)/%.proto
	$(GRPC_GEN) -I$(PROTOS_DIR) $(OUTPUT_OPTS) $^

all: $(PB2S)
