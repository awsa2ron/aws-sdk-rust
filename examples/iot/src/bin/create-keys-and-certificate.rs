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

    /// The Cert name.
    #[structopt(long)]
    certificate_pem_outfile: String,

    /// The Public Key name.
    #[structopt(long)]
    public_key_outfile: String,

    /// The Policy name.
    #[structopt(long)]
    private_key_outfile: String,

    /// Whether to active the certificate.
    #[structopt(short, long)]
    active: bool,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Get your IoT policy.
// snippet-start:[iot.rust.get-policy]
async fn create_certificates(
    client: &Client,
    cert: &str,
    pub_key: &str,
    key: &str,
    active: bool,
) -> Result<(), Error> {
    let resp = client
        .create_keys_and_certificate()
        .set_as_active(active)
        .send()
        .await?;

    let cert_content = &resp.certificate_pem().unwrap_or_default();
    let keys_content = &resp.key_pair().unwrap();

    fs::write(cert, &cert_content).expect("Unable to write file");
    fs::write(pub_key, &keys_content.public_key().unwrap()).expect("Unable to write file");
    fs::write(key, &keys_content.private_key().unwrap()).expect("Unable to write file");

    println!(
        "  certificate:  {}",
        &cert_content
    );
    println!(
        "  ARN:   {}",
        resp.certificate_arn().as_deref().unwrap_or_default()
    );
    println!(
        "  key pair:   {:#?}",
        resp.key_pair().as_deref().unwrap()
    );
    println!(
        "  Id:   {}",
        resp.certificate_id().as_deref().unwrap_or_default()
    );
    println!();

    Ok(())
}
// snippet-end:[iot.rust.get-policy]

/// Create the name, type, and doc of your IoT policy in the Region.
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
        certificate_pem_outfile,
        public_key_outfile,
        private_key_outfile,
        active,
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

    create_certificates(
        &client,
        &certificate_pem_outfile,
        &public_key_outfile,
        &private_key_outfile,
        active,
    )
    .await
}
