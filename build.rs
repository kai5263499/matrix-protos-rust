extern crate protobuf_codegen_pure;

fn main() {
	protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &[
            "protocol-buffers-master/matrix_io/common/entity.proto",
            "protocol-buffers-master/matrix_io/malos/v1/comm.proto",
            "protocol-buffers-master/matrix_io/malos/v1/driver.proto",
            "protocol-buffers-master/matrix_io/malos/v1/io.proto",
            "protocol-buffers-master/matrix_io/malos/v1/maloseye.proto",
            "protocol-buffers-master/matrix_io/malos/v1/sense.proto",
            "protocol-buffers-master/matrix_io/malos/v1/zwave_commands.proto",
            "protocol-buffers-master/matrix_io/recognition/v1/recognition_service.proto",
            "protocol-buffers-master/matrix_io/recognition/v1/recognition.proto",
            "protocol-buffers-master/matrix_io/vision/v1/vision_service.proto",
            "protocol-buffers-master/matrix_io/vision/v1/vision.proto"
        ],
        includes: &["protocol-buffers-master"],
        customize: protobuf_codegen_pure::Customize {
          ..Default::default()
        },
    }).expect("protoc");
}