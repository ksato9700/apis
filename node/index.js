const grpc = require('grpc')
const protoLoader = require('@grpc/proto-loader')
const PROTO_PATH = __dirname + '/v1/helloworld.proto'

const packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {
        keepCase: true,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true
    }
);
module.exports = {
    grpc: grpc,
    apis: grpc.loadPackageDefinition(packageDefinition).apis,
};
