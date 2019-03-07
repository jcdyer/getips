use clap::{Arg, App};
use rusoto_core::region::Region;

use rusoto_ec2::{
    DescribeInstancesRequest,
    DescribeInstancesResult,
    Ec2,
    Ec2Client,
    Filter,
};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("environment")
            .required(true)
            .index(1))
        .get_matches();

    let environment = match matches.value_of("environment").unwrap() {
        "prod" => "production",
        "stage" => "staging",
        env => env
    };

    let ec2 = Ec2Client::new(Region::UsEast1);
    let mut params = DescribeInstancesRequest::default();
    params.filters = Some(
        vec![Filter { name: Some("tag:Environment".to_owned()), values: Some(vec![environment.to_owned()])}]
    );
    match ec2.describe_instances(params).sync() {
        Ok(output) => {
            for name in get_private_dns(output) {
                println!("  {}", name);
            }
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}

pub fn get_private_dns(result: DescribeInstancesResult) -> Vec<String> {
    result.reservations.unwrap_or(Vec::new()).into_iter()
        .flat_map(|res| res.instances.unwrap_or(Vec::new()).into_iter())
        .flat_map(|instance| instance.network_interfaces.unwrap_or(Vec::new()).into_iter())
        .flat_map(|net| net.private_dns_name.clone())
        .collect()
}
