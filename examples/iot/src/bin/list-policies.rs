/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iot::{Client, Error, Region, PKG_VERSION};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Lists your IoT policies.
// snippet-start:[iot.rust.list-policies]
async fn show_policies(client: &Client) -> Result<(), Error> {
    let resp = client.list_policies().send().await?;

    println!("Policies:");

    for policy in resp.policies.unwrap() {
        println!(
            "  Name:  {}",
            policy.policy_name.as_deref().unwrap_or_default()
        );
        println!(
            "  ARN:   {}",
            policy.policy_arn.as_deref().unwrap_or_default()
        );
        println!();
    }

    println!();

    Ok(())
}
// snippet-end:[iot.rust.list-policies]

/// Lists the name, type, and ARN of your IoT policies in the Region.
///
/// # Arguments
///
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt { region, verbose } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();
    if verbose {
        println!("IoT client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    show_policies(&client).await
}
