# Link for Later Service as an AWS Lambda Function

🚀 https://3kl56jatulipthwjepsv2wm67q0vhnql.lambda-url.ap-southeast-1.on.aws/v1/links

## Development

[Cargo Lambda](https://www.cargo-lambda.info/) is one of the options for development of this service and it is pre-installed as part of the devcontainer. Use [`cargo lambda watch`](https://www.cargo-lambda.info/commands/watch.html) to run the app as lambda:

```sh
cargo lambda watch
```

You will be able to send requests to the lambda using the following URL:

```sh
http://localhost:9000/lambda-url/link-for-later-lambda/
```
