extern crate clap;
extern crate surf;
use clap::{App, Arg};
use std::fmt::Write;
use thousands::Separable;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct Res {
    costs: Costs,
    hourly_rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dimension {
    #[serde(rename = "type")]
    typ: String,
    cost: f64,
}

#[derive(Serialize, Deserialize)]
struct Costs {
    total: f64,
    dimensions: Vec<Dimension>,
}

#[derive(Serialize, Deserialize)]
struct Msg {
    text: String,
}

async fn es_cost(ec_api_key: &str, organization_id: &str) -> Result<Res, surf::Error> {
    let header_val = "ApiKey ".to_string() + ec_api_key;

    let Res { costs, hourly_rate } = surf::get(
        "https://api.elastic-cloud.com/api/v1/billing/costs/".to_string() + organization_id,
    )
    .header("Authorization", header_val)
    .recv_json()
    .await?;
    Ok(Res { costs, hourly_rate })
}

async fn post_slack(slack_webhook_url: &str, msg: &str) -> Result<(), surf::Error> {
    let data = &Msg {
        text: msg.to_string(),
    };
    let _ = surf::post(slack_webhook_url).body_json(data)?.await?;
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let app = App::new("ec_cost_slack_bot")
        .version("0.1.0")
        .author("hiromu.nakamura <abctail30@gmail.com>")
        .about("Output Elastic Cloud cost report")
        .arg(
            Arg::new("key")
                .short('k')
                .long("ec-api-key")
                .env("EC_API_KEY")
                .takes_value(true)
                .help("Elastic Cloud api key"),
        )
        .arg(
            Arg::new("organization")
                .short('o')
                .long("organization-id")
                .env("EC_ORGANAIZATION_ID")
                .takes_value(true)
                .help("Elastic Cloud organaization id"),
        )
        .arg(
            Arg::new("slack")
                .short('s')
                .long("slack-webhook-url")
                .env("SLACK_WEBHOOK_URL")
                .takes_value(true)
                .help("Slack webhook url"),
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("hourly-rate-threshold")
                .env("HOURLY_LATE_THRESHOLD")
                .takes_value(true)
                .help("Slack webhook url"),
        );

    let matches = app.get_matches();
    let ec_api_key = matches.value_of("key").unwrap();
    let organization_id = matches.value_of("organization").unwrap();
    let slack_url = matches.value_of("slack").unwrap();
    let hourly_rate_threshold = matches.value_of("threshold").unwrap();

    let res = es_cost(ec_api_key, organization_id).await.unwrap();

    let hourly_rate_threshold_f = hourly_rate_threshold.parse::<f64>().unwrap();

    if hourly_rate_threshold_f < res.costs.total {
        let mut target_string = String::new();
        writeln!(
            target_string,
            ":warning: 料金が予測より超過している恐れがあります :warning:"
        )
        .unwrap();
        writeln!(target_string, "---- Elastic Cloud コストレポート ----").unwrap();
        writeln!(
            target_string,
            "今月のトータル: ${}",
            res.costs.total.separate_with_commas()
        )
        .unwrap();
        writeln!(
            target_string,
            "コスト/時間: ${}",
            res.hourly_rate.separate_with_commas()
        )
        .unwrap();
        writeln!(target_string).unwrap();
        writeln!(target_string, "## コスト種別").unwrap();
        for dimension in &res.costs.dimensions {
            writeln!(
                target_string,
                "{}: ${}",
                dimension.typ,
                dimension.cost.separate_with_commas()
            )
            .unwrap();
        }

        post_slack(slack_url, &target_string).await.unwrap();
    }

    Ok(())
}
