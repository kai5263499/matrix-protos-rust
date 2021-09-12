protocol-buffers-master.zip:
	wget -O protocol-buffers-master.zip https://github.com/matrix-io/protocol-buffers/archive/refs/heads/master.zip
	unzip -o protocol-buffers-master.zip

bindings: protocol-buffers-master.zip
	protoc --rust_out=src/protos -I protocol-buffers-master protocol-buffers-master/matrix_io/common/entity.proto
	protoc --rust_out=src/protos -I protocol-buffers-master protocol-buffers-master/matrix_io/malos/v1/*.proto
	protoc --rust_out=src/protos -I protocol-buffers-master protocol-buffers-master/matrix_io/recognition/v1/*.proto
	protoc --rust_out=src/protos -I protocol-buffers-master protocol-buffers-master/matrix_io/vision/v1/*.proto

clean:
	rm -rf protocol-buffers-master protocol-buffers-master.zip

all: protocol-buffers-master.zip bindings

.PHONY: bindings