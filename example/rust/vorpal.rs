use anyhow::Result;
use vorpal::api::build_service_client::BuildServiceClient;
use vorpal::api::{PackageRequest, PackageSource, PackageSourceKind};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let mut client = BuildServiceClient::connect("http://[::1]:23151").await?;

    client
        .package(PackageRequest {
            build_deps: Vec::new(),
            build_phase: r#"
                cd coreutils-9.5
                test -f configure || ./bootstrap
                ./configure --prefix=$OUTPUT
                make
            "#
            .to_string(),
            install_phase: "make install".to_string(),
            install_deps: Vec::new(),
            name: "coreutils".to_string(),
            source: Some(PackageSource {
                hash: Some(
                    "af6d643afd6241ec35c7781b7f999b97a66c84bea4710ad2bb15e75a5caf11b4".to_string(),
                ),
                ignore_paths: vec![],
                kind: PackageSourceKind::Http.into(),
                uri: "https://ftp.gnu.org/gnu/coreutils/coreutils-9.5.tar.gz".to_string(),
            }),
        })
        .await?;

    Ok(())
}
