# sample_simple_websockets_chat_app

This repository is a Rust implementation of [aws-samples/simple-websockets-chat-app](https://github.com/aws-samples/simple-websockets-chat-app).

## Build Environment

The execution environment is assumed to be [provided.al2 (Amazon Linux 2)/arm64](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html). Therefore, the default build target is `aarch64-unknown-linux-musl`.
If you are building on macOS, use [FiloSottile/homebrew-musl-cross](https://github.com/FiloSottile/homebrew-musl-cross) or [emk/rust-musl-builder](https://github.com/emk/rust-musl-builder).

## Build

Packages are built for each of [API Gateway's default websocket routes](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-route-keys-connect-disconnect.html) and custom routes.

```bash
# Build all packages
cargo build --release --workspace

# Build per package
cargo build --release --package <package_name>
```

## Deployment

### ws_sendmessage

Set the following environment variables.
The environment variables can also be obtained from the WebSocket URL (no need to set `AWS_REGION`).

`wss://{AWS_API_ID}.execute-api.{AWS_REGION}.amazonaws.com/{AWS_STAGE}`

```
AWS_API_ID
AWS_STAGE
DYNAMO_TABLE_NAME
```

### DynamoDB

The partition key for the DynamoDB table should be set as `connectionId`.

### IAM

Configure the Execution role of Lambda functions as follows:

#### Permissions

Grant the following permissions to enable access to the related services:

- AmazonAPIGatewayInvokeFullAccess
- AmazonDynamoDBFullAccess
- AWSXRayDaemonWriteAccess
- AWSLambdaBasicExecutionRole

#### Trust relationships

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
```

## Testing the chat API

To test the WebSocket API, you can use [wscat](https://github.com/websockets/wscat), an open-source command line tool.

1. [Install NPM](https://www.npmjs.com/get-npm).
2. Install wscat:

```bash
$ npm install -g wscat
```

3. On the console, connect to your published API endpoint by executing the following command:

```bash
$ wscat -c wss://{YOUR-API-ID}.execute-api.{YOUR-REGION}.amazonaws.com/{STAGE}
```

4. To test the sendMessage function, send a JSON message like the following example. The Lambda function sends it back
   using the callback URL:

```bash
$ wscat -c wss://{YOUR-API-ID}.execute-api.{YOUR-REGION}.amazonaws.com/prod
connected (press CTRL+C to quit)
> {"action":"sendmessage", "data":"hello world"}
< hello world
```
