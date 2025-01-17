//! # STS
//! 阿里云STS（Security Token Service）是阿里云提供的一种临时访问权限管理服务。RAM提供RAM用户和RAM角色两种身份。
//! 其中，RAM角色不具备永久身份凭证，而只能通过STS获取可以自定义时效和访问权限的临时身份凭证，即安全令牌（STS Token）。
//! @link [文档](https://help.aliyun.com/document_detail/28756.html)
//!
//! ## 用法
//!
//! ```
//! # async fn run() {
//! use aliyun_oss_client::{sts::STS, BucketName, Client, EndPoint};
//! let client = Client::new_with_sts(
//!     "STS.xxxxxxxx".into(),                         // KeyId
//!     "EVd6dXew6xxxxxxxxxxxxxxxxxxxxxxxxxxx".into(), // KeySecret
//!     EndPoint::CnShanghai,
//!     BucketName::new("yyyyyy").unwrap(),
//!     "CAIS4gF1q6Ft5Bxxxxxxxxxxx".to_string(), // STS Token
//! );
//!
//! let builder = client.get_bucket_list().await;
//! println!("{:?}", builder);
//! # }
//! ```

use crate::{auth::AuthBuilder, client::Client, BucketName, EndPoint, KeyId, KeySecret};

pub trait STS {
    fn new_with_sts(
        access_key_id: KeyId,
        access_key_secret: KeySecret,
        endpoint: EndPoint,
        bucket: BucketName,
        security_token: String,
    ) -> Self;
}

const SECURITY_TOKEN: &str = "x-oss-security-token";

impl<M: Default + Clone> STS for Client<M> {
    fn new_with_sts(
        access_key_id: KeyId,
        access_key_secret: KeySecret,
        endpoint: EndPoint,
        bucket: BucketName,
        security_token: String,
    ) -> Self {
        let mut auth_builder = AuthBuilder::default();
        auth_builder.key(access_key_id);
        auth_builder.secret(access_key_secret);
        auth_builder.header_insert(SECURITY_TOKEN, security_token.try_into().unwrap());

        Self::from_builder(auth_builder, endpoint, bucket)
    }
}

#[cfg(test)]
mod tests {
    use http::{HeaderValue, Method};

    use crate::{file::AlignBuilder, types::CanonicalizedResource, BucketName, Client, EndPoint};

    use super::STS;

    #[tokio::test]
    async fn test_sts() {
        let client = Client::new_with_sts(
            "foo1".into(),
            "foo2".into(),
            EndPoint::CnShanghai,
            BucketName::new("abc").unwrap(),
            "bar".to_string(),
        );

        let builder = client
            .builder(
                Method::GET,
                "https://abc.oss-cn-shanghai.aliyuncs.com/"
                    .try_into()
                    .unwrap(),
                CanonicalizedResource::default(),
            )
            .unwrap();

        let request = builder.build().unwrap();

        let headers = request.headers();
        let sts_token = headers.get("x-oss-security-token");

        assert_eq!(sts_token, Some(&HeaderValue::from_static("bar")));
    }
}
