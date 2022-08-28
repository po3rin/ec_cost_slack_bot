# ec_cost_slack_bot

[![ci](https://github.com/po3rin/ec_cost_slack_bot/actions/workflows/ci.yaml/badge.svg)](https://github.com/po3rin/ec_cost_slack_bot/actions/workflows/ci.yaml) [![Crate](https://img.shields.io/crates/v/ec_cost_slack_bot.svg)](https://crates.io/crates/ec_cost_slack_bot) [![API](https://docs.rs/ec_cost_slack_bot/badge.svg)](https://docs.rs/ec_cost_slack_bot)

this cli lets you to warn Elastic Cloud cost to Slack.

```bash
$ cargo install ec_cost_slack_bot

# set env EC_API_KEY, EC_ORGANAIZATION_ID, SLACK_WEBHOOK_URL, HOURLY_LATE_THRESHOLD
$ ec_cost_slack_bot 
```

output example

<img src="./sample.png">
