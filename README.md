# An echo app for Slack with Fastly Compute@Edge

Inspired by https://github.com/yusukebe/fastly-compute-slack-command, but implemented in Rust.

## Develop

[Fastly CLI](https://developer.fastly.com/reference/cli/) is required for development.

Prepare `fastly.toml` as you like (`fastly.toml.sample` is a template without the service id).

```shell-session
# serve a local app server
fastly compute serve

# publish the app
fastly compute publish
```

## A demo URL set to Slack commands

```
https://legally-guiding-stork.edgecompute.app/command/fastly-echo
```

## License

This is an open source software licensed under ISC. See LICENSE for details.
