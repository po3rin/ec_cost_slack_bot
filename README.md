# ec_cost_slack_bot

[![ci](https://github.com/po3rin/ec_cost_slack_bot/actions/workflows/ci.yaml/badge.svg)](https://github.com/po3rin/ec_cost_slack_bot/actions/workflows/ci.yaml) [![Crate](https://img.shields.io/crates/v/ec_cost_slack_bot.svg)](https://crates.io/crates/ec_cost_slack_bot)

this cli lets you to warn Elastic Cloud cost to Slack.

```bash
$ cargo install ec_cost_slack_bot

# set env EC_API_KEY, EC_ORGANAIZATION_ID, SLACK_WEBHOOK_URL, HOURLY_LATE_THRESHOLD
$ ec_cost_slack_bot 
```

```
USAGE:
    ec_cost_slack_bot [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -k, --ec-api-key <key>
            Elastic Cloud api key [env: EC_API_KEY=]

    -o, --organization-id <organization>
            Elastic Cloud organaization id [env: EC_ORGANAIZATION_ID=]

    -s, --slack-webhook-url <slack>
            Slack webhook url [env: SLACK_WEBHOOK_URL=]

    -t, --hourly-rate-threshold <threshold>
            Slack webhook url [env: HOURLY_LATE_THRESHOLD=]

    -V, --version
            Print version information
```

output example

<img src="./sample.png">

