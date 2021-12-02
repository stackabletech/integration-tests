use anyhow::Result;
use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};

/// Creates a bucket with test data
pub fn create_s3_test_storage(
    s3_endpoint: &str,
    s3_access_key: &str,
    s3_secret_key: &str,
) -> Result<()> {
    let credentials = Credentials::new(Some(s3_access_key), Some(s3_secret_key), None, None, None)?;

    let region = Region::Custom {
        region: String::new(),
        endpoint: s3_endpoint.to_owned(),
    };

    let create_bucket_response = Bucket::create_with_path_style_blocking(
        "test",
        region,
        credentials,
        BucketConfiguration::default(),
    )?;
    let bucket = create_bucket_response.bucket;

    bucket.put_object_blocking("csv/companies.csv", b"Stackable")?;

    Ok(())
}
