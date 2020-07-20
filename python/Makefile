OUTDIR=apis
GRPC_GEN=python -m grpc_tools.protoc
OUTPUT_OPTS=--python_out=$(OUTDIR) --grpc_python_out=$(OUTDIR)

PROTOS_DIR=../v1
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
PB2S=$(patsubst $(PROTOS_DIR)/%.proto,$(OUTDIR)/%_pb2.py,$(PROTOS))

$(OUTDIR)/%_pb2.py:$(PROTOS_DIR)/%.proto
	$(GRPC_GEN) -I$(PROTOS_DIR) $(OUTPUT_OPTS) $^

all: $(PB2S)
