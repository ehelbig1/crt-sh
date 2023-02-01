use chrono;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Certificate {
    issuer_ca_id: usize,
    issuer_name: String,
    common_name: String,
    name_value: String,
    id: usize,
    entry_timestamp: chrono::NaiveDateTime,
    not_before: chrono::NaiveDateTime,
    not_after: chrono::NaiveDateTime,
    serial_number: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    const JSON: &str = r#"[
{
    "issuer_ca_id": 1,
    "issuer_name": "C=US, O=\"Test\", CN=Test SSL CA",
    "common_name": "test.com",
    "name_value": "test.com\nTest\nTest Inc",
    "id": 1,
    "entry_timestamp": "1970-01-01T00:00:00.000",
    "not_before": "1970-01-01T00:00:00",
    "not_after": "1970-01-01T00:00:00",
    "serial_number": "1a"
}]"#;

    #[test]
    fn it_parses() {
        let expected = vec![Certificate {
            issuer_ca_id: 1,
            issuer_name: String::from("C=US, O=\"Test\", CN=Test SSL CA"),
            common_name: String::from("test.com"),
            name_value: String::from("test.com\nTest\nTest Inc"),
            id: 1,
            entry_timestamp: chrono::NaiveDateTime::parse_from_str(
                "1970-01-01T00:00:00.000",
                "%Y-%m-%dT%H:%M:%S%.f",
            )
            .unwrap(),
            not_before: chrono::NaiveDateTime::parse_from_str(
                "1970-01-01T00:00:00",
                "%Y-%m-%dT%H:%M:%S",
            )
            .unwrap(),
            not_after: chrono::NaiveDateTime::parse_from_str(
                "1970-01-01T00:00:00",
                "%Y-%m-%dT%H:%M:%S",
            )
            .unwrap(),
            serial_number: String::from("1a"),
        }];

        let got: Vec<Certificate> = serde_json::from_str(JSON).unwrap();

        assert_eq!(expected, got)
    }
}
