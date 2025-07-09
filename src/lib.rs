use arrayvec::ArrayString;
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArnParseError {
    #[error("Invalid ARN format: expected at least 6 parts separated by ':' but got {0}")]
    InvalidFormat(usize),
    #[error("Service name too long (max 32 characters)")]
    ServiceTooLong,
    #[error("Account ID too long (max 12 characters)")]
    AccountTooLong,
    #[error("Resource ID too long (max 64 characters)")]
    ResourceIdTooLong,
    #[error("Invalid region: {0}")]
    InvalidRegion(String),
}

#[derive(Default, PartialEq, Eq, Hash, Clone, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Arn {
    pub service: Component<ArrayString<32>>,
    pub region: Component<Region>,
    pub account: Component<ArrayString<12>>,
    pub resource_id: Component<ArrayString<64>>,
}

impl Arn {
    pub const ANY: Arn = Arn {
        service: Component::Any,
        region: Component::Any,
        account: Component::Any,
        resource_id: Component::Any,
    };
}

impl FromStr for Arn {
    type Err = ArnParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() < 6 {
            return Err(ArnParseError::InvalidFormat(parts.len()));
        }

        let service = if parts[2].is_empty() {
            Component::None
        } else {
            Component::Value(
                ArrayString::from(parts[2]).map_err(|_| ArnParseError::ServiceTooLong)?,
            )
        };

        let region = if parts[3].is_empty() {
            Component::None
        } else {
            Component::Value(
                parts[3]
                    .parse()
                    .map_err(|_| ArnParseError::InvalidRegion(parts[3].to_string()))?,
            )
        };

        let account = if parts[4].is_empty() {
            Component::None
        } else {
            Component::Value(
                ArrayString::from(parts[4]).map_err(|_| ArnParseError::AccountTooLong)?,
            )
        };

        let resource_part = parts[5..].join(":");
        let resource_id = if resource_part.is_empty() {
            Component::None
        } else {
            Component::Value(
                ArrayString::from(&resource_part).map_err(|_| ArnParseError::ResourceIdTooLong)?,
            )
        };

        Ok(Arn {
            service,
            region,
            account,
            resource_id,
        })
    }
}

impl TryFrom<String> for Arn {
    type Error = ArnParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<Arn> for String {
    fn from(arn: Arn) -> String {
        arn.to_string()
    }
}

impl fmt::Display for Arn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let service = match &self.service {
            Component::Value(s) => s.as_str(),
            Component::Any => "*",
            Component::None => "",
        };

        let region = match &self.region {
            Component::Value(r) => r.as_ref(),
            Component::Any => "*",
            Component::None => "",
        };

        let account = match &self.account {
            Component::Value(a) => a.as_str(),
            Component::Any => "*",
            Component::None => "",
        };

        let resource_id = match &self.resource_id {
            Component::Value(id) => id.as_str(),
            Component::Any => "*",
            Component::None => "",
        };

        write!(
            f,
            "arn:aws:{}:{}:{}:{}",
            service, region, account, resource_id
        )
    }
}

impl fmt::Debug for Arn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Component<V> {
    #[default]
    None,
    Any,
    Value(V),
}

#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub enum Region {
    #[default]
    UsEast1,
    UsEast2,
    UsWest1,
    UsWest2,
    AfSouth1,
    ApEast1,
    ApEast2,
    ApSouth1,
    ApSouth2,
    ApSoutheast1,
    ApSoutheast2,
    ApSoutheast3,
    ApSoutheast4,
    ApSoutheast5,
    ApSoutheast7,
    ApNortheast1,
    ApNortheast2,
    ApNortheast3,
    CaCentral1,
    CaWest1,
    EuCentral1,
    EuCentral2,
    EuWest1,
    EuWest2,
    EuSouth1,
    EuSouth2,
    EuNorth1,
    EuNorth2,
    IlCentral1,
    MxCentral1,
    MeSouth1,
    MeCentral1,
    SaEast1,
}

impl Region {
    pub const GLOBAL: Region = Region::UsEast1;
}

#[derive(Error, Debug)]
pub enum RegionError {
    #[error("Region does not exist: {0}")]
    DoesNotExist(String),
}

impl AsRef<str> for Region {
    fn as_ref(&self) -> &str {
        use Region::*;

        match self {
            UsEast1 => "us-east-1",
            UsEast2 => "us-east-2",
            UsWest1 => "us-west-1",
            UsWest2 => "us-west-2",
            AfSouth1 => "af-south-1",
            ApEast1 => "ap-east-1",
            ApEast2 => "ap-east-2",
            ApSouth1 => "ap-south-1",
            ApSouth2 => "ap-south-2",
            ApSoutheast1 => "ap-southeast-1",
            ApSoutheast2 => "ap-southeast-2",
            ApSoutheast3 => "ap-southeast-3",
            ApSoutheast4 => "ap-southeast-4",
            ApSoutheast5 => "ap-southeast-5",
            ApSoutheast7 => "ap-southeast-7",
            ApNortheast1 => "ap-northeast-1",
            ApNortheast2 => "ap-northeast-2",
            ApNortheast3 => "ap-northeast-3",
            CaCentral1 => "ca-central-1",
            CaWest1 => "ca-west-1",
            EuCentral1 => "eu-central-1",
            EuCentral2 => "eu-central-2",
            EuWest1 => "eu-west-1",
            EuWest2 => "eu-west-2",
            EuSouth1 => "eu-south-1",
            EuSouth2 => "eu-south-2",
            EuNorth1 => "eu-north-1",
            EuNorth2 => "eu-north-2",
            IlCentral1 => "il-central-1",
            MxCentral1 => "mx-central-1",
            MeSouth1 => "me-south-1",
            MeCentral1 => "me-central-1",
            SaEast1 => "sa-east-1",
        }
    }
}

impl FromStr for Region {
    type Err = RegionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Region::*;

        match s {
            "us-east-1" => Ok(UsEast1),
            "us-east-2" => Ok(UsEast2),
            "us-west-1" => Ok(UsWest1),
            "us-west-2" => Ok(UsWest2),
            "af-south-1" => Ok(AfSouth1),
            "ap-east-1" => Ok(ApEast1),
            "ap-east-2" => Ok(ApEast2),
            "ap-south-1" => Ok(ApSouth1),
            "ap-south-2" => Ok(ApSouth2),
            "ap-southeast-1" => Ok(ApSoutheast1),
            "ap-southeast-2" => Ok(ApSoutheast2),
            "ap-southeast-3" => Ok(ApSoutheast3),
            "ap-southeast-4" => Ok(ApSoutheast4),
            "ap-southeast-5" => Ok(ApSoutheast5),
            "ap-southeast-7" => Ok(ApSoutheast7),
            "ap-northeast-1" => Ok(ApNortheast1),
            "ap-northeast-2" => Ok(ApNortheast2),
            "ap-northeast-3" => Ok(ApNortheast3),
            "ca-central-1" => Ok(CaCentral1),
            "ca-west-1" => Ok(CaWest1),
            "eu-central-1" => Ok(EuCentral1),
            "eu-central-2" => Ok(EuCentral2),
            "eu-west-1" => Ok(EuWest1),
            "eu-west-2" => Ok(EuWest2),
            "eu-south-1" => Ok(EuSouth1),
            "eu-south-2" => Ok(EuSouth2),
            "eu-north-1" => Ok(EuNorth1),
            "eu-north-2" => Ok(EuNorth2),
            "il-central-1" => Ok(IlCentral1),
            "mx-central-1" => Ok(MxCentral1),
            "me-south-1" => Ok(MeSouth1),
            "me-central-1" => Ok(MeCentral1),
            "sa-east-1" => Ok(SaEast1),
            _ => Err(RegionError::DoesNotExist(s.to_string())),
        }
    }
}

impl From<Region> for String {
    fn from(value: Region) -> Self {
        value.as_ref().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_arn() {
        let arn: Arn = "arn:aws:s3:us-east-1:123456789012:bucket".parse().unwrap();
        assert_eq!(
            arn.service,
            Component::Value(ArrayString::from("s3").unwrap())
        );
        assert_eq!(arn.region, Component::Value(Region::UsEast1));
        assert_eq!(
            arn.account,
            Component::Value(ArrayString::from("123456789012").unwrap())
        );
        assert_eq!(
            arn.resource_id,
            Component::Value(ArrayString::from("bucket").unwrap())
        );
    }

    #[test]
    fn test_parse_arn_with_slash() {
        let arn: Arn = "arn:aws:s3:us-east-1:123456789012:bucket/folder/file.txt"
            .parse()
            .unwrap();
        assert_eq!(
            arn.resource_id,
            Component::Value(ArrayString::from("bucket/folder/file.txt").unwrap())
        );
    }

    #[test]
    fn test_parse_arn_with_colon_in_resource() {
        let arn: Arn = "arn:aws:lambda:us-east-1:123456789012:function:my-function:$LATEST"
            .parse()
            .unwrap();
        assert_eq!(
            arn.resource_id,
            Component::Value(ArrayString::from("function:my-function:$LATEST").unwrap())
        );
    }

    #[test]
    fn test_parse_arn_with_empty_fields() {
        let arn: Arn = "arn:aws:iam::123456789012:role/my-role".parse().unwrap();
        assert_eq!(
            arn.service,
            Component::Value(ArrayString::from("iam").unwrap())
        );
        assert_eq!(arn.region, Component::None);
        assert_eq!(
            arn.account,
            Component::Value(ArrayString::from("123456789012").unwrap())
        );
        assert_eq!(
            arn.resource_id,
            Component::Value(ArrayString::from("role/my-role").unwrap())
        );
    }

    #[test]
    fn test_parse_invalid_arn_format() {
        let result = "arn:aws:s3".parse::<Arn>();
        assert!(matches!(result, Err(ArnParseError::InvalidFormat(3))));
    }

    #[test]
    fn test_parse_invalid_region() {
        let result = "arn:aws:s3:invalid-region:123456789012:bucket".parse::<Arn>();
        assert!(matches!(result, Err(ArnParseError::InvalidRegion(_))));
    }

    #[test]
    fn test_parse_service_too_long() {
        let long_service = "a".repeat(33);
        let arn_str = format!("arn:aws:{}:us-east-1:123456789012:bucket", long_service);
        let result = arn_str.parse::<Arn>();
        assert!(matches!(result, Err(ArnParseError::ServiceTooLong)));
    }

    #[test]
    fn test_parse_account_too_long() {
        let long_account = "1".repeat(13);
        let arn_str = format!("arn:aws:s3:us-east-1:{}:bucket", long_account);
        let result = arn_str.parse::<Arn>();
        assert!(matches!(result, Err(ArnParseError::AccountTooLong)));
    }

    #[test]
    fn test_parse_resource_id_too_long() {
        let long_resource = "a".repeat(65);
        let arn_str = format!("arn:aws:s3:us-east-1:123456789012:{}", long_resource);
        let result = arn_str.parse::<Arn>();
        assert!(matches!(result, Err(ArnParseError::ResourceIdTooLong)));
    }

    #[test]
    fn test_display_basic_arn() {
        let arn = Arn {
            service: Component::Value(ArrayString::from("s3").unwrap()),
            region: Component::Value(Region::UsEast1),
            account: Component::Value(ArrayString::from("123456789012").unwrap()),
            resource_id: Component::Value(ArrayString::from("bucket").unwrap()),
        };
        assert_eq!(arn.to_string(), "arn:aws:s3:us-east-1:123456789012:bucket");
    }

    #[test]
    fn test_display_arn_with_wildcards() {
        let arn = Arn {
            service: Component::Any,
            region: Component::Any,
            account: Component::Any,
            resource_id: Component::Any,
        };
        assert_eq!(arn.to_string(), "arn:aws:*:*:*:*");
    }

    #[test]
    fn test_display_arn_with_empty_fields() {
        let arn = Arn {
            service: Component::Value(ArrayString::from("iam").unwrap()),
            region: Component::None,
            account: Component::Value(ArrayString::from("123456789012").unwrap()),
            resource_id: Component::Value(ArrayString::from("role/my-role").unwrap()),
        };
        assert_eq!(arn.to_string(), "arn:aws:iam::123456789012:role/my-role");
    }

    #[test]
    fn test_arn_any_constant() {
        assert_eq!(Arn::ANY.to_string(), "arn:aws:*:*:*:*");
    }

    #[test]
    fn test_roundtrip_parsing() {
        let original = "arn:aws:s3:us-east-1:123456789012:bucket/folder/file.txt";
        let arn: Arn = original.parse().unwrap();
        assert_eq!(arn.to_string(), original);
    }

    #[test]
    fn test_roundtrip_parsing_with_empty_region() {
        let original = "arn:aws:iam::123456789012:role/my-role";
        let arn: Arn = original.parse().unwrap();
        assert_eq!(arn.to_string(), original);
    }

    #[test]
    fn test_serde_serialization() {
        let arn = Arn {
            service: Component::Value(ArrayString::from("s3").unwrap()),
            region: Component::Value(Region::UsEast1),
            account: Component::Value(ArrayString::from("123456789012").unwrap()),
            resource_id: Component::Value(ArrayString::from("bucket").unwrap()),
        };
        let json = serde_json::to_string(&arn).unwrap();
        assert_eq!(json, "\"arn:aws:s3:us-east-1:123456789012:bucket\"");
    }

    #[test]
    fn test_serde_deserialization() {
        let json = "\"arn:aws:s3:us-east-1:123456789012:bucket\"";
        let arn: Arn = serde_json::from_str(json).unwrap();
        assert_eq!(
            arn.service,
            Component::Value(ArrayString::from("s3").unwrap())
        );
        assert_eq!(arn.region, Component::Value(Region::UsEast1));
        assert_eq!(
            arn.account,
            Component::Value(ArrayString::from("123456789012").unwrap())
        );
        assert_eq!(
            arn.resource_id,
            Component::Value(ArrayString::from("bucket").unwrap())
        );
    }

    #[test]
    fn test_serde_deserialization_error() {
        let json = "\"invalid-arn\"";
        let result = serde_json::from_str::<Arn>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_equality_and_hashing() {
        let arn1: Arn = "arn:aws:s3:us-east-1:123456789012:bucket".parse().unwrap();
        let arn2: Arn = "arn:aws:s3:us-east-1:123456789012:bucket".parse().unwrap();
        let arn3: Arn = "arn:aws:s3:us-east-1:123456789012:other-bucket"
            .parse()
            .unwrap();

        assert_eq!(arn1, arn2);
        assert_ne!(arn1, arn3);

        // Test that they can be used in hash-based collections
        let mut set = std::collections::HashSet::new();
        set.insert(arn1.clone());
        assert!(set.contains(&arn2));
        assert!(!set.contains(&arn3));
    }

    #[test]
    fn test_debug_display() {
        let arn: Arn = "arn:aws:s3:us-east-1:123456789012:bucket".parse().unwrap();
        assert_eq!(
            format!("{:?}", arn),
            "arn:aws:s3:us-east-1:123456789012:bucket"
        );
    }

    #[test]
    fn test_region_parsing() {
        let arn: Arn = "arn:aws:s3:eu-west-1:123456789012:bucket".parse().unwrap();
        assert_eq!(arn.region, Component::Value(Region::EuWest1));
    }

    #[test]
    fn test_from_string_conversion() {
        let arn_string = "arn:aws:s3:us-east-1:123456789012:bucket".to_string();
        let arn: Arn = arn_string.try_into().unwrap();
        assert_eq!(
            arn.service,
            Component::Value(ArrayString::from("s3").unwrap())
        );
    }

    #[test]
    fn test_into_string_conversion() {
        let arn: Arn = "arn:aws:s3:us-east-1:123456789012:bucket".parse().unwrap();
        let arn_string: String = arn.into();
        assert_eq!(arn_string, "arn:aws:s3:us-east-1:123456789012:bucket");
    }
}
