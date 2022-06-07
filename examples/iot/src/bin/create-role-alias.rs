/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iot::{Client, Error, Region, PKG_VERSION};
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The arn of the role.
    #[structopt(short, long)]
    arn: String,

    /// The arn of the file containing the policy document.
    #[structopt(short, long)]
    alias: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Creates a role.
// snippet-start:[iam.rust.create-role]
async fn make_role_alias(client: &Client, alias: &str, arn: &str) -> Result<(), Error> {

    let resp = client
        .create_role_alias()
        .role_alias(alias)
        .role_arn(arn)
        .send()
        .await?;

    println!(
        "Created role with ARN {}",
        resp.role_alias().unwrap()
    );

    println!(
        "Created role with ARN {}",
        resp.role_alias_arn().unwrap()
    );
    Ok(())
}
// snippet-end:[iam.rust.create-role]

/// Creates an IAM role in the Region.
///
/// # Arguments
///
/// * `-a ACCOUNT-ID` - Your account ID.
/// * `-b BUCKET` - The arn of the bucket where Config stores information about resources.
/// * `-n NAME` - The arn of the role.
/// * `-p POLICY-NAME` - The arn of the JSON file containing the policy document.
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {
        arn,
        alias,
        region,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("IAM client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!("Role arn:          {}", &arn);
        println!("Policy doc filename {}", &alias);
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    make_role_alias(&client, &alias, &arn).await
}
