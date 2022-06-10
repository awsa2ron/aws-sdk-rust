/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iam::{Client, Error, Region, PKG_VERSION};
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the role.
    #[structopt(short, long)]
    name: String,

    /// The ARN of the file containing the policy document.
    #[structopt(short, long)]
    policy_arn: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Creates a role.
// snippet-start:[iam.rust.create-role]
async fn attach_role_policy(client: &Client, policy_arn: &str, name: &str) -> Result<(), Error> {
    // Read policy doc from file as a string

    let _resp = client
        .attach_role_policy()
        .policy_arn(policy_arn)
        .role_name(name)
        .send()
        .await?;


    Ok(())
}
// snippet-end:[iam.rust.create-role]

/// Creates an IAM role in the Region.
///
/// # Arguments
///
/// * `-a ACCOUNT-ID` - Your account ID.
/// * `-b BUCKET` - The name of the bucket where Config stores information about resources.
/// * `-n NAME` - The name of the role.
/// * `-p POLICY-NAME` - The name of the JSON file containing the policy document.
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {
        name,
        policy_arn,
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
        println!("Role name:          {}", &name);
        println!("Policy doc filename {}", &policy_arn);
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    attach_role_policy(&client, &policy_arn, &name).await
}
