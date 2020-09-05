PROTOS_DIR=../../v1
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
OUTDIR=apis

TWIRP_GEN=protoc
TWIRP_OUTPUT_OPTS=--python_out=$(OUTDIR) --twirpy_out=$(OUTDIR)

gen-twirp: $(PROTOS)
	$(TWIRP_GEN) -I$(PROTOS_DIR) $(TWIRP_OUTPUT_OPTS) $^
