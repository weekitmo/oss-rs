use std::borrow::Cow;
use std::collections::HashMap;
#[cfg(feature = "core")]
use std::env;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use http::header::{HeaderValue, InvalidHeaderValue, ToStrError};
#[cfg(feature = "core")]
use reqwest::Url;

#[cfg(feature = "core")]
use crate::config::BucketBase;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct KeyId(Cow<'static, str>);

impl AsRef<str> for KeyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for KeyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryInto<HeaderValue> for KeyId {
    type Error = InvalidHeaderValue;
    fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.as_ref())
    }
}

impl From<String> for KeyId {
    fn from(s: String) -> KeyId {
        KeyId(Cow::Owned(s))
    }
}

impl From<&'static str> for KeyId {
    fn from(key_id: &'static str) -> Self {
        Self::from_static(key_id)
    }
}

impl KeyId {
    /// Creates a new `KeyId` from the given string.
    pub fn new(key_id: impl Into<Cow<'static, str>>) -> Self {
        Self(key_id.into())
    }

    /// Const function that creates a new `KeyId` from a static str.
    pub const fn from_static(key_id: &'static str) -> Self {
        Self(Cow::Borrowed(key_id))
    }
}

//===================================================================================================

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct KeySecret(Cow<'static, str>);

impl AsRef<str> for KeySecret {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for KeySecret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryInto<HeaderValue> for KeySecret {
    type Error = InvalidHeaderValue;
    fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.as_ref())
    }
}

impl From<String> for KeySecret {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&'static str> for KeySecret {
    fn from(secret: &'static str) -> Self {
        Self::from_static(secret)
    }
}

impl KeySecret {
    /// Creates a new `KeySecret` from the given string.
    pub fn new(secret: impl Into<Cow<'static, str>>) -> Self {
        Self(secret.into())
    }

    /// Const function that creates a new `KeySecret` from a static str.
    pub const fn from_static(secret: &'static str) -> Self {
        Self(Cow::Borrowed(secret))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_ref().as_bytes()
    }
}

//===================================================================================================

/// OSS 的可用区
#[cfg(feature = "core")]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum EndPoint {
    #[default]
    CnHangzhou,
    CnShanghai,
    CnQingdao,
    CnBeijing,
    CnZhangjiakou, // 张家口 lenght=13
    CnHongkong,
    CnShenzhen,
    UsWest1,
    UsEast1,
    ApSouthEast1,
}

pub const HANGZHOU: &str = "cn-hangzhou";
pub const SHANGHAI: &str = "cn-shanghai";
pub const QINGDAO: &str = "cn-qingdao";
pub const BEIJING: &str = "cn-beijing";
pub const ZHANGJIAKOU: &str = "cn-zhangjiakou";
pub const HONGKONG: &str = "cn-hongkong";
pub const SHENZHEN: &str = "cn-shenzhen";
pub const US_WEST1: &str = "us-west1";
pub const US_EAST1: &str = "us-east1";
pub const AP_SOUTH_EAST1: &str = "ap-south-east1";

#[cfg(feature = "core")]
impl AsRef<str> for EndPoint {
    fn as_ref(&self) -> &str {
        match *self {
            Self::CnHangzhou => HANGZHOU,
            Self::CnShanghai => SHANGHAI,
            Self::CnQingdao => QINGDAO,
            Self::CnBeijing => BEIJING,
            Self::CnZhangjiakou => ZHANGJIAKOU,
            Self::CnHongkong => HONGKONG,
            Self::CnShenzhen => SHENZHEN,
            Self::UsWest1 => US_WEST1,
            Self::UsEast1 => US_EAST1,
            Self::ApSouthEast1 => AP_SOUTH_EAST1,
            //_ => "custom",
        }
    }
}

#[cfg(feature = "core")]
impl Display for EndPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub const HANGZHOU_L: &str = "hangzhou";
pub const SHANGHAI_L: &str = "shanghai";
pub const QINGDAO_L: &str = "qingdao";
pub const BEIJING_L: &str = "beijing";
pub const ZHANGJIAKOU_L: &str = "zhangjiakou";
pub const HONGKONG_L: &str = "hongkong";
pub const SHENZHEN_L: &str = "shenzhen";

#[cfg(feature = "core")]
impl From<String> for EndPoint {
    /// 字符串转 endpoint
    /// 举例1 - 产生恐慌
    /// ```should_panic
    /// # use aliyun_oss_client::types::EndPoint;
    /// let e: EndPoint = String::from("weifang").into();
    /// ```
    /// 举例2 - 正常
    /// ```
    /// # use aliyun_oss_client::types::EndPoint;
    /// let e: EndPoint = String::from("qingdao").into();
    /// ```
    fn from(url: String) -> Self {
        Self::new(&url).unwrap()
    }
}

#[cfg(feature = "core")]
impl<'a> From<&'a str> for EndPoint {
    fn from(url: &'a str) -> Self {
        Self::new(url).unwrap()
    }
}

#[cfg(feature = "core")]
impl FromStr for EndPoint {
    type Err = InvalidEndPoint;
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Self::new(url)
    }
}

pub const OSS_DOMAIN_PREFIX: &str = "https://oss-";
pub const OSS_INTERNAL: &str = "-internal";
pub const OSS_DOMAIN_MAIN: &str = ".aliyuncs.com";

#[cfg(feature = "core")]
impl<'a> EndPoint {
    /// 通过字符串字面值初始化 endpoint
    ///
    /// 举例1 - 产生恐慌
    /// ```should_panic
    /// # use aliyun_oss_client::types::EndPoint;
    /// EndPoint::from_static("weifang");
    /// ```
    /// 举例2 - 正常
    /// ```
    /// # use aliyun_oss_client::types::EndPoint;
    /// EndPoint::from_static("qingdao");
    /// ```
    pub fn from_static(url: &'a str) -> Self {
        Self::new(url).unwrap_or_else(|_| panic!("Unknown Endpoint :{}", url))
    }

    /// 初始化 endpoint enum
    /// ```
    /// # use aliyun_oss_client::types::EndPoint;
    /// assert!(matches!(
    ///     EndPoint::new("shanghai"),
    ///     Ok(EndPoint::CnShanghai)
    /// ));
    /// assert!(EndPoint::new("weifang").is_err());
    /// ```
    pub fn new(url: &'a str) -> Result<Self, InvalidEndPoint> {
        if url.contains(SHANGHAI_L) {
            Ok(Self::CnShanghai)
        } else if url.contains(HANGZHOU_L) {
            Ok(Self::CnHangzhou)
        } else if url.contains(QINGDAO_L) {
            Ok(Self::CnQingdao)
        } else if url.contains(BEIJING_L) {
            Ok(Self::CnBeijing)
        } else if url.contains(ZHANGJIAKOU_L) {
            Ok(Self::CnZhangjiakou)
        } else if url.contains(HONGKONG_L) {
            Ok(Self::CnHongkong)
        } else if url.contains(SHENZHEN_L) {
            Ok(Self::CnShenzhen)
        } else if url.contains(US_WEST1) {
            Ok(Self::UsWest1)
        } else if url.contains(US_EAST1) {
            Ok(Self::UsEast1)
        } else if url.contains(AP_SOUTH_EAST1) {
            Ok(Self::ApSouthEast1)
        } else {
            Err(InvalidEndPoint)
        }
    }

    /// 转化成 Url
    /// ```
    /// # use aliyun_oss_client::types::EndPoint;
    /// use reqwest::Url;
    /// let endpoint = EndPoint::new("shanghai").unwrap();
    /// assert_eq!(
    ///     endpoint.to_url(),
    ///     Url::parse("https://oss-cn-shanghai.aliyuncs.com").unwrap()
    /// );
    ///
    /// use std::env::set_var;
    /// set_var("ALIYUN_OSS_INTERNAL", "true");
    /// let endpoint = EndPoint::new("shanghai").unwrap();
    /// assert_eq!(
    ///     endpoint.to_url(),
    ///     Url::parse("https://oss-cn-shanghai-internal.aliyuncs.com").unwrap()
    /// );
    /// ```
    pub fn to_url(&self) -> Url {
        let mut url = String::from(OSS_DOMAIN_PREFIX);
        url.push_str(self.as_ref());

        // internal
        if env::var("ALIYUN_OSS_INTERNAL").is_ok() {
            url.push_str(OSS_INTERNAL);
        }

        url.push_str(OSS_DOMAIN_MAIN);
        Url::parse(&url).unwrap()
    }
}

#[cfg(feature = "core")]
#[derive(Debug)]
#[non_exhaustive]
pub struct InvalidEndPoint;

#[cfg(feature = "core")]
impl Error for InvalidEndPoint {}

#[cfg(feature = "core")]
impl fmt::Display for InvalidEndPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "endpoint must like with https://xxx.aliyuncs.com")
    }
}

#[cfg(feature = "core")]
impl PartialEq<&str> for EndPoint {
    /// # 相等比较
    /// ```
    /// # use aliyun_oss_client::types::EndPoint;
    /// let e: EndPoint = String::from("qingdao").try_into().unwrap();
    /// assert!(e == "cn-qingdao");
    /// ```
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        &self.as_ref() == other
    }
}

#[cfg(feature = "core")]
impl PartialEq<EndPoint> for &str {
    /// # 相等比较
    /// ```
    /// # use aliyun_oss_client::types::EndPoint;
    /// let e: EndPoint = String::from("qingdao").try_into().unwrap();
    /// assert!("cn-qingdao" == e);
    /// ```
    #[inline]
    fn eq(&self, other: &EndPoint) -> bool {
        self == &other.as_ref()
    }
}

#[cfg(feature = "core")]
impl PartialEq<Url> for EndPoint {
    /// # 相等比较
    /// ```
    /// # use aliyun_oss_client::types::EndPoint;
    /// use reqwest::Url;
    /// let endpoint = EndPoint::new("shanghai").unwrap();
    /// assert!(endpoint == Url::parse("https://oss-cn-shanghai.aliyuncs.com").unwrap());
    /// ```
    #[inline]
    fn eq(&self, other: &Url) -> bool {
        &self.to_url() == other
    }
}

//===================================================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BucketName(Cow<'static, str>);

impl AsRef<str> for BucketName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for BucketName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for BucketName {
    fn default() -> BucketName {
        BucketName::new("a").unwrap()
    }
}

// impl TryInto<HeaderValue> for BucketName {
//     type Error = InvalidHeaderValue;
//     fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
//         HeaderValue::from_str(self.as_ref())
//     }
// }
impl From<String> for BucketName {
    fn from(s: String) -> Self {
        Self::new(s).unwrap()
    }
}

impl<'a> From<&'a str> for BucketName {
    fn from(bucket: &'a str) -> Self {
        Self::from_static(bucket).unwrap()
    }
}

impl FromStr for BucketName {
    type Err = InvalidBucketName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_static(s)
    }
}

impl<'a> BucketName {
    /// Creates a new `BucketName` from the given string.
    /// 只允许小写字母、数字、短横线（-），且不能以短横线开头或结尾
    ///
    /// ```
    /// # use aliyun_oss_client::types::BucketName;
    ///
    /// assert!(BucketName::new("").is_err());
    /// assert!(BucketName::new("abc").is_ok());
    /// assert!(BucketName::new("abc-").is_err());
    /// assert!(BucketName::new("-abc").is_err());
    /// assert!(BucketName::new("abc-def234ab").is_ok());
    /// assert!(BucketName::new("abc-def*#$%^ab").is_err());
    /// ```
    pub fn new(bucket: impl Into<Cow<'static, str>>) -> Result<Self, InvalidBucketName> {
        let bucket = bucket.into();

        fn valid_character(c: char) -> bool {
            match c {
                _ if c.is_ascii_lowercase() => true,
                _ if c.is_numeric() => true,
                '-' => true,
                _ => false,
            }
        }
        if !bucket.chars().all(valid_character) {
            return Err(InvalidBucketName);
        }

        if bucket.len() < 1 {
            return Err(InvalidBucketName);
        }

        if bucket.starts_with('-') || bucket.ends_with('-') {
            return Err(InvalidBucketName);
        }

        Ok(Self(bucket))
    }

    /// Const function that creates a new `BucketName` from a static str.
    pub fn from_static(bucket: &'a str) -> Result<Self, InvalidBucketName> {
        fn valid_character(c: char) -> bool {
            match c {
                _ if c.is_ascii_lowercase() => true,
                _ if c.is_numeric() => true,
                '-' => true,
                _ => false,
            }
        }
        if !bucket.chars().all(valid_character) {
            return Err(InvalidBucketName);
        }

        if bucket.is_empty() {
            return Err(InvalidBucketName);
        }

        if bucket.starts_with('-') || bucket.ends_with('-') {
            return Err(InvalidBucketName);
        }

        Ok(Self(Cow::Owned(bucket.to_owned())))
    }
}

impl PartialEq<&str> for BucketName {
    /// 相等比较
    /// ```
    /// # use aliyun_oss_client::types::BucketName;
    /// let path = BucketName::new("abc").unwrap();
    /// assert!(path == "abc");
    /// ```
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<BucketName> for &str {
    /// 相等比较
    /// ```
    /// # use aliyun_oss_client::types::BucketName;
    /// let path = BucketName::new("abc").unwrap();
    /// assert!("abc" == path);
    /// ```
    #[inline]
    fn eq(&self, other: &BucketName) -> bool {
        self == &other.0
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct InvalidBucketName;

impl Error for InvalidBucketName {}

impl fmt::Display for InvalidBucketName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "bucket 名称只允许小写字母、数字、短横线（-），且不能以短横线开头或结尾"
        )
    }
}

//===================================================================================================

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ContentMd5(Cow<'static, str>);

impl AsRef<str> for ContentMd5 {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ContentMd5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryInto<HeaderValue> for ContentMd5 {
    type Error = InvalidHeaderValue;
    fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.as_ref())
    }
}
impl From<String> for ContentMd5 {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl ContentMd5 {
    /// Creates a new `ContentMd5` from the given string.
    pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
        Self(val.into())
    }

    /// Const function that creates a new `ContentMd5` from a static str.
    pub const fn from_static(val: &'static str) -> Self {
        Self(Cow::Borrowed(val))
    }
}

//===================================================================================================

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ContentType(Cow<'static, str>);

impl AsRef<str> for ContentType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryInto<HeaderValue> for ContentType {
    type Error = InvalidHeaderValue;
    fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.as_ref())
    }
}
impl TryFrom<HeaderValue> for ContentType {
    type Error = ToStrError;
    fn try_from(value: HeaderValue) -> Result<Self, Self::Error> {
        Ok(Self(Cow::Owned(value.to_str()?.to_owned())))
    }
}
impl From<String> for ContentType {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl ContentType {
    /// Creates a new `ContentMd5` from the given string.
    pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
        Self(val.into())
    }

    /// Const function that creates a new `ContentMd5` from a static str.
    pub const fn from_static(val: &'static str) -> Self {
        Self(Cow::Borrowed(val))
    }
}

//===================================================================================================

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Date(Cow<'static, str>);

impl AsRef<str> for Date {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryInto<HeaderValue> for Date {
    type Error = InvalidHeaderValue;
    fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.as_ref())
    }
}
impl From<String> for Date {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}
impl From<&'static str> for Date {
    fn from(date: &'static str) -> Self {
        Self::from_static(date)
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(d: DateTime<Utc>) -> Self {
        Self(Cow::Owned(d.format("%a, %d %b %Y %T GMT").to_string()))
    }
}

impl Date {
    /// Creates a new `Date` from the given string.
    pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
        Self(val.into())
    }

    /// Const function that creates a new `Date` from a static str.
    pub const fn from_static(val: &'static str) -> Self {
        Self(Cow::Borrowed(val))
    }
}

//===================================================================================================

/// 计算方式，参考 [aliyun 文档](https://help.aliyun.com/document_detail/31951.htm?spm=a2c4g.11186623.0.0.38d27d22mvQcxj#section-w2k-sw2-xdb)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalizedResource(Cow<'static, str>);

impl AsRef<str> for CanonicalizedResource {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for CanonicalizedResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryInto<HeaderValue> for CanonicalizedResource {
    type Error = InvalidHeaderValue;
    fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.as_ref())
    }
}
impl From<String> for CanonicalizedResource {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl Default for CanonicalizedResource {
    fn default() -> Self {
        Self::new("/")
    }
}

pub const CONTINUATION_TOKEN: &str = "continuation-token";
pub const BUCKET_INFO: &str = "bucketInfo";
#[cfg(feature = "core")]
const QUERY_KEYWORD: [&str; 2] = ["acl", BUCKET_INFO];

impl CanonicalizedResource {
    /// Creates a new `CanonicalizedResource` from the given string.
    pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
        Self(val.into())
    }

    /// Const function that creates a new `CanonicalizedResource` from a static str.
    pub const fn from_static(val: &'static str) -> Self {
        Self(Cow::Borrowed(val))
    }

    /// 获取 bucket 的签名参数
    #[cfg(feature = "core")]
    pub fn from_bucket(bucket: &BucketBase, query: Option<&str>) -> Self {
        match query {
            Some(q) => {
                for k in QUERY_KEYWORD.iter() {
                    if *k == q {
                        return Self::from(format!("/{}/?{}", bucket.name(), q));
                    }
                }

                Self::from(format!("/{}/", bucket.name()))
            }
            None => Self::from_static("/"),
        }
    }

    /// 获取 bucket 的签名参数
    /// 带查询条件的
    ///
    /// 如果查询条件中有翻页的话，则忽略掉其他字段
    #[cfg(feature = "core")]
    pub fn from_bucket_query(bucket: &BucketBase, query: &Query) -> Self {
        match query.get(CONTINUATION_TOKEN) {
            Some(v) => Self::from(format!(
                "/{}/?continuation-token={}",
                bucket.name(),
                v.as_ref()
            )),
            None => Self::from(format!("/{}/", bucket.name())),
        }
    }

    /// 根据 OSS 存储对象（Object）查询签名参数
    #[cfg(feature = "core")]
    pub(crate) fn from_object<Q: IntoIterator<Item = (QueryKey, QueryValue)>>(
        (bucket, path): (&str, &str),
        query: Q,
    ) -> Self {
        let query = Query::from_iter(query);
        if query.is_empty() {
            Self::from(format!("/{}/{}", bucket, path))
        } else {
            let query_value = query.to_url_query();
            Self::from(format!("/{}/{}?{}", bucket, path, query_value))
        }
    }
}

impl PartialEq<&str> for CanonicalizedResource {
    /// # 相等比较
    /// ```
    /// # use aliyun_oss_client::types::CanonicalizedResource;
    /// let res = CanonicalizedResource::new("abc");
    /// assert!(res == "abc");
    /// ```
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<CanonicalizedResource> for &str {
    /// # 相等比较
    /// ```
    /// # use aliyun_oss_client::types::CanonicalizedResource;
    /// let res = CanonicalizedResource::new("abc");
    /// assert!("abc" == res);
    /// ```
    #[inline]
    fn eq(&self, other: &CanonicalizedResource) -> bool {
        self == &other.0
    }
}

//===================================================================================================
/// 查询条件
///
/// ```
/// use aliyun_oss_client::types::Query;
///
/// let query: Query = [("abc", "def")].into_iter().collect();
/// assert_eq!(query.len(), 1);
///
/// let value = query.get("abc");
/// assert!(value.is_some());
/// let value = value.unwrap();
/// assert_eq!(value.as_ref(), "def");
///
/// let str = query.to_oss_string();
/// assert_eq!(str.as_str(), "list-type=2&abc=def");
/// let str = query.to_url_query();
/// assert_eq!(str.as_str(), "abc=def");
/// ```
#[derive(Clone, Debug, Default)]
pub struct Query {
    inner: HashMap<QueryKey, QueryValue>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: impl Into<QueryKey>, value: impl Into<QueryValue>) {
        self.inner.insert(key.into(), value.into());
    }

    pub fn get(&self, key: impl Into<QueryKey>) -> Option<&QueryValue> {
        self.inner.get(&key.into())
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn remove(&mut self, key: impl Into<QueryKey>) -> Option<QueryValue> {
        self.inner.remove(&key.into())
    }

    /// 将查询参数拼成 aliyun 接口需要的格式
    pub fn to_oss_string(&self) -> String {
        let mut query_str = String::from("list-type=2");
        for (key, value) in self.inner.iter() {
            query_str += "&";
            query_str += key.as_ref();
            query_str += "=";
            query_str += value.as_ref();
        }
        query_str
    }

    /// 转化成 url 参数的形式
    /// a=foo&b=bar
    /// 未进行 urlencode 转码
    pub fn to_url_query(&self) -> String {
        self.inner
            .iter()
            .map(|(k, v)| {
                let mut res = String::with_capacity(k.as_ref().len() + v.as_ref().len() + 1);
                res.push_str(k.as_ref());
                res.push('=');
                res.push_str(v.as_ref());
                res
            })
            .collect::<Vec<_>>()
            .join("&")
    }
}

impl IntoIterator for Query {
    type Item = (QueryKey, QueryValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    /// # 使用 Vec 转 Query
    /// 例子
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// # use aliyun_oss_client::QueryValue;
    /// # use assert_matches::assert_matches;
    /// let query = Query::from_iter(vec![("foo", "bar")]);
    /// let list: Vec<_> = query.into_iter().collect();
    /// assert_eq!(list.len(), 1);
    /// assert_matches!(&list[0].0, &QueryKey::Custom(_));
    /// let value: QueryValue = "bar".parse().unwrap();
    /// assert_eq!(list[0].1, value);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().collect::<Vec<_>>().into_iter()
    }
}

impl FromIterator<(QueryKey, QueryValue)> for Query {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (QueryKey, QueryValue)>,
    {
        let mut map = Query::default();
        map.inner.extend(iter);
        map
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for Query {
    /// 转化例子
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// let query = Query::from_iter([("max-keys", "123")]);
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u8.into()));
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u16.into()));
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let inner = iter
            .into_iter()
            .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()));

        let mut map = Query::default();
        map.inner.extend(inner);
        map
    }
}

impl<'a> FromIterator<(&'a str, u8)> for Query {
    /// 转化例子
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// let query = Query::from_iter([("max-keys", 123u8)]);
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u8.into()));
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u16.into()));
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, u8)>,
    {
        let inner = iter
            .into_iter()
            .map(|(k, v)| (k.parse().unwrap(), v.into()));

        let mut map = Query::default();
        map.inner.extend(inner);
        map
    }
}

impl<'a> FromIterator<(&'a str, u16)> for Query {
    /// 转化例子
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// let query = Query::from_iter([("max-keys", 123u16)]);
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u8.into()));
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u16.into()));
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, u16)>,
    {
        let inner = iter
            .into_iter()
            .map(|(k, v)| (k.parse().unwrap(), v.into()));

        let mut map = Query::default();
        map.inner.extend(inner);
        map
    }
}

impl<'a> FromIterator<(QueryKey, &'a str)> for Query {
    /// 转化例子
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// let query = Query::from_iter([(QueryKey::MaxKeys, "123")]);
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u8.into()));
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u16.into()));
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (QueryKey, &'a str)>,
    {
        let inner = iter.into_iter().map(|(k, v)| (k, v.parse().unwrap()));

        let mut map = Query::default();
        map.inner.extend(inner);
        map
    }
}

impl FromIterator<(QueryKey, u8)> for Query {
    /// 转化例子
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// let query = Query::from_iter([(QueryKey::MaxKeys, 123u8)]);
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u8.into()));
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u16.into()));
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (QueryKey, u8)>,
    {
        let inner = iter.into_iter().map(|(k, v)| (k, v.into()));

        let mut map = Query::default();
        map.inner.extend(inner);
        map
    }
}

impl FromIterator<(QueryKey, u16)> for Query {
    /// 转化例子
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// let query = Query::from_iter([(QueryKey::MaxKeys, 123u16)]);
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u8.into()));
    /// assert_eq!(query.get(QueryKey::MaxKeys), Some(&123u16.into()));
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (QueryKey, u16)>,
    {
        let inner = iter.into_iter().map(|(k, v)| (k, v.into()));

        let mut map = Query::default();
        map.inner.extend(inner);
        map
    }
}

// impl<K, V, const N: usize> From<[(K, V); N]> for Query
// where
//     K: Into<QueryKey>,
//     V: Into<QueryValue>,
// {
//     fn from(arr: [(K, V); N]) -> Self {
//         arr.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
//     }
// }

#[cfg(feature = "core")]
pub trait UrlQuery {
    fn set_search_query(&mut self, query: &Query);
}

#[cfg(feature = "core")]
impl UrlQuery for Url {
    /// 将查询参数拼接到 API 的 Url 上
    ///
    /// # 例子
    /// ```
    /// use aliyun_oss_client::types::Query;
    /// use aliyun_oss_client::types::UrlQuery;
    /// use reqwest::Url;
    ///
    /// let query = Query::from_iter([("abc", "def")]);
    /// let mut url = Url::parse("https://exapmle.com").unwrap();
    /// url.set_search_query(&query);
    /// assert_eq!(url.as_str(), "https://exapmle.com/?list-type=2&abc=def");
    /// assert_eq!(url.query(), Some("list-type=2&abc=def"));
    /// ```
    fn set_search_query(&mut self, query: &Query) {
        self.set_query(Some(&query.to_oss_string()));
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QueryKey {
    Delimiter,
    StartAfter,
    ContinuationToken,
    MaxKeys,
    Prefix,
    EncodingType,
    FetchOwner,
    Custom(Cow<'static, str>),
}

impl AsRef<str> for QueryKey {
    fn as_ref(&self) -> &str {
        match *self {
            QueryKey::Delimiter => "delimiter",
            QueryKey::StartAfter => "start-after",
            QueryKey::ContinuationToken => "continuation-token",
            QueryKey::MaxKeys => "max-keys",
            QueryKey::Prefix => "prefix",
            QueryKey::EncodingType => "encoding-type",
            // TODO
            QueryKey::FetchOwner => unimplemented!("parse xml not support fetch owner"),
            QueryKey::Custom(ref str) => str,
        }
    }
}

impl Display for QueryKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

// TODO 需要的时候再开启
// impl TryInto<HeaderValue> for QueryKey {
//     type Error = InvalidHeaderValue;
//     fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
//         HeaderValue::from_str(self.as_ref())
//     }
// }
impl From<String> for QueryKey {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
impl From<&'static str> for QueryKey {
    fn from(date: &'static str) -> Self {
        Self::from_static(date)
    }
}

impl FromStr for QueryKey {
    type Err = InvalidQueryKey;
    /// 示例
    /// ```
    /// # use aliyun_oss_client::types::QueryKey;
    /// let value: QueryKey = "abc".into();
    /// assert!(value == QueryKey::from_static("abc"));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_owned()))
    }
}

#[derive(Debug)]
pub struct InvalidQueryKey;

impl Error for InvalidQueryKey {}

impl Display for InvalidQueryKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid query key")
    }
}

impl QueryKey {
    /// # Examples
    /// ```
    /// # use aliyun_oss_client::QueryKey;
    /// # use assert_matches::assert_matches;
    /// let key = QueryKey::new("delimiter");
    /// assert!(key == QueryKey::Delimiter);
    ///
    /// let key = QueryKey::new("abc");
    /// assert_matches!(key, QueryKey::Custom(_));
    /// ```
    /// `fetch-owner` 功能未实现，特殊说明
    /// ```should_panic
    /// # use aliyun_oss_client::QueryKey;
    /// let key = QueryKey::new("fetch-owner");
    /// ```
    pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        if val.contains("delimiter") {
            Self::Delimiter
        } else if val.contains("start-after") {
            Self::StartAfter
        } else if val.contains("continuation-token") {
            Self::ContinuationToken
        } else if val.contains("max-keys") {
            Self::MaxKeys
        } else if val.contains("prefix") {
            Self::Prefix
        } else if val.contains("encoding-type") {
            Self::EncodingType
        } else if val.contains("fetch-owner") {
            unimplemented!("parse xml not support fetch owner");
        } else {
            Self::Custom(val)
        }
    }

    /// # Examples
    /// ```
    /// # use aliyun_oss_client::QueryKey;
    /// # use assert_matches::assert_matches;
    /// let key = QueryKey::from_static("delimiter");
    /// assert!(key == QueryKey::Delimiter);
    ///
    /// let key = QueryKey::from_static("abc");
    /// assert_matches!(key, QueryKey::Custom(_));
    /// ```
    /// `fetch-owner` 功能未实现，特殊说明
    /// ```should_panic
    /// # use aliyun_oss_client::QueryKey;
    /// let key = QueryKey::from_static("fetch-owner");
    /// ```
    pub fn from_static(val: &'static str) -> Self {
        if val.contains("delimiter") {
            Self::Delimiter
        } else if val.contains("start-after") {
            Self::StartAfter
        } else if val.contains("continuation-token") {
            Self::ContinuationToken
        } else if val.contains("max-keys") {
            Self::MaxKeys
        } else if val.contains("prefix") {
            Self::Prefix
        } else if val.contains("encoding-type") {
            Self::EncodingType
        } else if val.contains("fetch-owner") {
            unimplemented!("parse xml not support fetch owner");
        } else {
            Self::Custom(Cow::Borrowed(val))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct QueryValue(Cow<'static, str>);

impl AsRef<str> for QueryValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for QueryValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO 需要的时候再开启
// impl TryInto<HeaderValue> for QueryValue {
//     type Error = InvalidHeaderValue;
//     fn try_into(self) -> Result<HeaderValue, InvalidHeaderValue> {
//         HeaderValue::from_str(self.as_ref())
//     }
// }
impl From<String> for QueryValue {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}
impl From<&'static str> for QueryValue {
    fn from(date: &'static str) -> Self {
        Self::from_static(date)
    }
}

impl PartialEq<&str> for QueryValue {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl From<u8> for QueryValue {
    /// 数字转 Query 值
    ///
    /// ```
    /// # use aliyun_oss_client::Query;
    /// # use aliyun_oss_client::QueryKey;
    /// let query = Query::from_iter([("max_keys", 100u8)]);
    /// let query = Query::from_iter([(QueryKey::MaxKeys, 100u8)]);
    /// ```
    fn from(num: u8) -> Self {
        Self(Cow::Owned(num.to_string()))
    }
}

impl PartialEq<u8> for QueryValue {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        self.to_string() == other.to_string()
    }
}

impl From<u16> for QueryValue {
    /// 数字转 Query 值
    ///
    /// ```
    /// use aliyun_oss_client::Query;
    /// let query = Query::from_iter([("max_keys", 100u16)]);
    /// ```
    fn from(num: u16) -> Self {
        Self(Cow::Owned(num.to_string()))
    }
}

impl PartialEq<u16> for QueryValue {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.to_string() == other.to_string()
    }
}

impl From<bool> for QueryValue {
    /// bool 转 Query 值
    ///
    /// ```
    /// use aliyun_oss_client::Query;
    /// let query = Query::from_iter([("abc", "false")]);
    /// ```
    fn from(b: bool) -> Self {
        if b {
            Self::from_static("true")
        } else {
            Self::from_static("false")
        }
    }
}

impl FromStr for QueryValue {
    type Err = InvalidQueryValue;
    /// 示例
    /// ```
    /// # use aliyun_oss_client::types::QueryValue;
    /// let value: QueryValue = "abc".parse().unwrap();
    /// assert!(value == "abc");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Cow::Owned(s.to_owned())))
    }
}

#[derive(Debug)]
pub struct InvalidQueryValue;

impl Error for InvalidQueryValue {}

impl Display for InvalidQueryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid query value")
    }
}

impl QueryValue {
    /// Creates a new `QueryValue` from the given string.
    pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
        Self(val.into())
    }

    /// Const function that creates a new `QueryValue` from a static str.
    pub const fn from_static(val: &'static str) -> Self {
        Self(Cow::Borrowed(val))
    }
}

use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

pub struct ContentRange {
    start: Option<u32>,
    end: Option<u32>,
}

impl From<Range<u32>> for ContentRange {
    fn from(r: Range<u32>) -> Self {
        Self {
            start: Some(r.start),
            end: Some(r.end),
        }
    }
}

impl From<RangeFull> for ContentRange {
    fn from(_f: RangeFull) -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}

impl From<RangeFrom<u32>> for ContentRange {
    fn from(f: RangeFrom<u32>) -> Self {
        Self {
            start: Some(f.start),
            end: None,
        }
    }
}

impl From<RangeTo<u32>> for ContentRange {
    fn from(t: RangeTo<u32>) -> Self {
        Self {
            start: None,
            end: Some(t.end),
        }
    }
}

impl From<ContentRange> for HeaderValue {
    /// # 转化成 OSS 需要的格式
    /// @link [OSS 文档](https://help.aliyun.com/document_detail/31980.html)
    ///
    /// ```
    /// use reqwest::header::HeaderValue;
    /// # use aliyun_oss_client::types::ContentRange;
    /// fn abc<R: Into<ContentRange>>(range: R) -> HeaderValue {
    ///     range.into().into()
    /// }
    ///
    /// assert_eq!(abc(..), HeaderValue::from_str("bytes=0-").unwrap());
    /// assert_eq!(abc(1..), HeaderValue::from_str("bytes=1-").unwrap());
    /// assert_eq!(abc(10..20), HeaderValue::from_str("bytes=10-20").unwrap());
    /// assert_eq!(abc(..20), HeaderValue::from_str("bytes=0-20").unwrap());
    /// ```
    fn from(con: ContentRange) -> HeaderValue {
        let string = match (con.start, con.end) {
            (Some(ref start), Some(ref end)) => format!("bytes={}-{}", start, end),
            (Some(ref start), None) => format!("bytes={}-", start),
            (None, Some(ref end)) => format!("bytes=0-{}", end),
            (None, None) => format!("bytes=0-"),
        };

        HeaderValue::from_str(&string).expect(&format!(
            "content-range into header-value failed, content-range is : {}",
            string
        ))
    }
}
