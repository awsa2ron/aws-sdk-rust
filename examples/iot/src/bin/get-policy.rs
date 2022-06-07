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

    /// The Component name.
    #[structopt(short, long)]
    name: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Get your IoT policy.
// snippet-start:[iot.rust.get-policy]
async fn show_the_policy(client: &Client, name: &str) -> Result<(), Error> {
    let resp = client.get_policy().policy_name(name).send().await?;

    println!(
        "  Name:  {}",
        resp.policy_name.as_deref().unwrap_or_default()
    );
    println!(
        "  ARN:   {}",
        resp.policy_arn.as_deref().unwrap_or_default()
    );
    println!(
        "  Document:   {}",
        resp.policy_document().as_deref().unwrap_or_default()
    );
    println!();

    Ok(())
}
// snippet-end:[iot.rust.get-policy]

/// Lists the name, type, and document of your IoT policy in the Region.
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
    let Opt {
        region,
        name,
        verbose,
    } = Opt::from_args();

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

    show_the_policy(&client, &name).await
}
