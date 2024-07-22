use super::*;
use anyhow::Result;
use pretty_assertions::assert_eq;
use serde_json::json;

mod sets_the_value {
    use super::*;

    macro_rules! test {
        ($name:ident, $key:expr, $value:tt) => {
            #[async_std::test]
            async fn $name() -> Result<()> {
                let service = Service::default();
                let key = $key;
                let value = json!($value).to_string();

                service.set(key, None, &value).await?;

                self::assert_eq!(value, service.get(key, None).await?);

                Ok(())
            }
        };
    }

    test!(null, "key", null);
    test!(bool_false, "   key   ", false);
    test!(bool_true, "camelCaseKey", true);
    test!(integer_negative, "PascalCaseKey", (-1));
    test!(integer_negative_zero, "snake_case_key", (-0));
    test!(integer_zero, "SNAKE_UPPERCASE_KEY", 0);
    test!(integer_positive, "kebab-case-key", 1);
    test!(float_negative, "dot.case.key", (-1.0));
    test!(float_negative_zero, "float_negative_zero", (-0.0));
    test!(float_zero, "float_zero", 0.0);
    test!(float_positive, "float_positive", 1.0);
    test!(string_empty, "string_empty", "");
    test!(string_normal, "key", "value");
    test!(string_space, "key space", "value space");
    test!(string_tab, "key\ttab", "value\ttab");
    test!(string_newline, "key\nnewline", "value\nnewline");
    test!(array_empty, "array_empty", []);
    test!(array_one_element, "array_one_element", ["value"]);
    test!(
        array_multiple_elements,
        "array_multiple_elements",
        ["value1", "value2", "value3"]
    );
    test!(object_empty, "object_empty", {});
    test!(object_one_key, "object_one_key", {"key":"value"});
    test!(object_multiple_keys, "object_multiple_keys", {"key1":"value1", "key2":"value2", "key3":"value3"});
}
