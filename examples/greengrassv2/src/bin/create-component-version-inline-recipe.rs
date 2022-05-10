/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_greengrassv2::{Client, Error, Region, PKG_VERSION};
use aws_smithy_types::Blob;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The Component ARN.
    #[structopt(short, long)]
    inline_recipe: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Lists your IoT components.
// snippet-start:[iot.rust.list-component-devices]
async fn show_components(client: &Client, inline_recipe: &str) -> Result<(), Error> {
       let blob = Blob::new(inline_recipe);

    let resp = client.create_component_version().inline_recipe(blob).send().await?;

    println!("components:");

    println!(
        "    ARN:  {}",
        resp.arn().unwrap_or_default()
    );
    println!(
        "   Name:  {:?}",
        resp.component_name().unwrap()
    );
    println!(
        "Version:  {:?}",
        resp.component_version().unwrap()
    );
    println!(
        "    creation timestamp:  {:?}",
        resp.creation_timestamp().unwrap()
    );

    println!();

    println!();

    Ok(())
}
// snippet-end:[iot.rust.list-component-devices]

/// Lists the name, type, and ARN of your IoT components in the Region.
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
    let Opt { region,  inline_recipe, verbose } = Opt::from_args();

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

    show_components(&client, &inline_recipe).await
}
