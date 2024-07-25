use super::*;
use anyhow::Result;
use pretty_assertions::assert_eq;
use serde_json::json;

mod sets_the_value {
    use super::*;

    macro_rules! test {
        ($name:ident, $($key:expr, $value:tt),+) => {
            #[async_std::test]
            async fn $name() -> Result<()> {
                let service = Service::default();

                $({
                    let key = $key;
                    let value = json!($value).to_string();

                    service.set(key, None, &value).await?;

                    self::assert_eq!(value, service.get(key, None).await?);
                })+

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
    test!(object_one_key, "object_one_key", { "key": "value" });
    test!(object_multiple_keys, "object_multiple_keys", { "key1": "value1", "key2": "value2", "key3": "value3" });
    test!(
        replaces_value,
        "key",
        "value1",
        "key",
        "value2",
        "key",
        "value3"
    );
}

mod sets_the_value_at_path {
    use super::*;

    macro_rules! test {
        ($name:ident, $($path:expr, $value:tt, $want:tt),+) => {
            #[async_std::test]
            async fn $name() -> Result<()> {
                let service = Service::default();

                $({
                    let key = "key";
                    let path = Some($path);
                    let value = json!($value).to_string();
                    let want = json!($want).to_string();

                    service.set(key, path, &value).await?;

                    self::assert_eq!(want, service.get(key, None).await?);
                })+

                Ok(())
            }
        };
    }

    mod root {
        use super::*;

        macro_rules! root_test {
            ($name:ident, $value:tt) => {
                test!($name, "$", $value, $value);
            };
        }

        root_test!(null, null);
        root_test!(bool_false, false);
        root_test!(bool_true, true);
        root_test!(integer_negative, (-1));
        root_test!(integer_negative_zero, (-0));
        root_test!(integer_zero, 0);
        root_test!(integer_positive, 1);
        root_test!(float_negative, (-1.0));
        root_test!(float_negative_zero, (-0.0));
        root_test!(float_zero, 0.0);
        root_test!(float_positive, 1.0);
        root_test!(string_empty, "");
        root_test!(string_normal, "value");
        root_test!(string_space, "value space");
        root_test!(string_tab, "value\ttab");
        root_test!(string_newline, "value\nnewline");
        root_test!(array_empty, []);
        root_test!(array_one_element, ["value"]);
        root_test!(array_multiple_elements, ["value1", "value2", "value3"]);
        root_test!(object_empty, {});
        root_test!(object_one_key, { "key": "value" });
        root_test!(object_multiple_keys, { "key1": "value1", "key2": "value2", "key3": "value3" });
    }

    mod specific {
        use super::*;

        macro_rules! specific_test {
            ($name:ident, $initial:tt, $path:expr, $value:tt, $want:tt) => {
                test!($name, "$", $initial, $initial, $path, $value, $want);
            };
        }

        specific_test!(last_array_index, [1, 2, 3, 4], "$[#]", 99, [1, 2, 3, 4, 99]);
        specific_test!(
            nested_last_array_index,
            [1, [2, 3], 4],
            "$[1][#]",
            99,
            [1, [2, 3, 99], 4]
        );
        specific_test!(replaces_value, { "a": 2, "c": 4 }, "$.a", 99, { "a": 99, "c": 4 });
        specific_test!(inserts_value, { "a": 2, "c": 4 }, "$.e", 99, { "a": 2, "c": 4, "e": 99 });
        specific_test!(decodes_once, { "a": 2, "c": 4 }, "$.c", [97, 96], { "a": 2, "c": [97, 96] });
        specific_test!(doesnt_decode_twice, { "a": 2, "c": 4 }, "$.c", "[97, 96]", { "a": 2, "c": "[97, 96]" });
    }
}

mod sets_the_value_fail {
    use super::*;

    macro_rules! test {
        ($name:ident, $($key:expr, $path:expr, $value:expr, $want:pat $(if $guard:expr)?),+) => {
            #[async_std::test]
            async fn $name() -> Result<()> {
                let service = Service::default();

                $({
                    let key = $key;
                    let path = $path;
                    let value = $value;

                    let result = service.set(key, path, value).await;

                    assert!(matches!(result, $want $(if $guard)?));
                })+

                Ok(())
            }
        };
    }

    test!(key_not_found, "key", Some("$.key"), json!("value").to_string(), Err(SetError::KeyNotFound(key)) if key == "key");
    test!(
        malformed_json,
        "key",
        None,
        "invalid",
        Err(SetError::InvalidJson(_))
    );
}
