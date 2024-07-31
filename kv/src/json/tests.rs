use super::*;
use anyhow::Result;
use pretty_assertions::assert_eq;
use serde_json::json;

mod gets_the_value {
    use super::*;

    macro_rules! test {
        ($name:ident, $key:expr, $initial:tt, $paths:expr, $want:tt) => {
            #[async_std::test]
            async fn $name() -> Result<()> {
                let service = Service::default();
                let key = $key;
                let initial = json!($initial).to_string();
                let paths: Option<&[&str]> = $paths;
                let want = json!($want).to_string();
                service.set(key, None, initial).await?;

                let result = service.get(key, paths).await?;

                self::assert_eq!(want, result);

                Ok(())
            }
        };
    }

    mod without_path {
        use super::*;

        macro_rules! without_path_test {
            ($name:ident, $key:expr, $value:tt) => {
                test!($name, $key, $value, None, $value);
            };
        }

        without_path_test! {
            without_path,
            "without_path",
            { "a": 2, "c": [4, 5, { "f": 7 }] }
        }
    }

    mod single_path {
        use super::*;

        macro_rules! single_path_test {
            ($name:ident, $key:expr, $initial:tt, $path:expr, $want:tt) => {
                test!($name, $key, $initial, Some(&[$path]), $want);
            };
        }

        single_path_test! {
            root_path,
            "root_path",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            "$",
            { "a": 2, "c": [4, 5, { "f": 7 }] }
        }
        single_path_test! {
            simple_key,
            "simple_key",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            "$.c",
            [4, 5, { "f": 7 }]
        }
        single_path_test! {
            simple_key_without_dollar,
            "simple_key_without_dollar",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            "c",
            [4, 5, { "f": 7 }]
        }
        single_path_test! {
            array_index,
            "array_index",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            "$.c[2]",
            { "f": 7 }
        }
        single_path_test! {
            array_index_with_key,
            "array_index_with_key",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            "$.c[2].f",
            7
        }
        single_path_test! {
            last_array_index,
            "last_array_index",
            { "a": 2, "c": [4, 5], "f": 7 },
            "$.c[#-1]",
            5
        }
        single_path_test! {
            bool,
            "bool",
            [false, false, false, true],
            "3",
            true
        }
        single_path_test! {
            number,
            "number",
            [11, 22, 33, 44],
            "3",
            44
        }
        single_path_test! {
            string,
            "string",
            { "a": "xyz" },
            "$.a",
            "xyz"
        }
        single_path_test! {
            null,
            "null",
            { "a": null },
            "$.a",
            null
        }
    }

    mod multiple_paths {
        use super::*;

        macro_rules! multiple_paths_test {
            ($name:ident, $key:expr, $initial:tt, $paths:expr, $want:tt) => {
                test!($name, $key, $initial, Some(&$paths), $want);
            };
        }

        multiple_paths_test! {
            multiple_roots,
            "multiple_roots",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            ["$", "$"],
            { "$": { "a": 2, "c": [4, 5, { "f": 7 }] } }
        }
        multiple_paths_test! {
            simple_keys,
            "simple_keys",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            ["$.a", "$.c"],
            { "$.a": 2, "$.c": [4, 5, { "f": 7 }] }
        }
        multiple_paths_test! {
            simple_keys_without_dollar,
            "simple_keys_without_dollar",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            ["a", "c"],
            { "a": 2, "c": [4, 5, { "f": 7 }] }
        }
        multiple_paths_test! {
            simple_keys_with_and_without_dollar,
            "simple_keys_with_and_without_dollar",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            ["$.a", "a", "c"],
            { "$.a": 2, "a": 2, "c": [4, 5, { "f": 7 }] }
        }
        multiple_paths_test! {
            array_indexes,
            "array_indexes",
            { "a": 2, "c": [4, 5, { "f": 7 }] },
            ["$.c[1]", "$.c[2]"],
            { "$.c[1]": 5, "$.c[2]": { "f": 7 } }
        }
        multiple_paths_test! {
            array_indexes_with_key,
            "array_indexes_with_key",
            { "a": 2, "c": [4, { "b": 5 }, { "f": 7 }] },
            ["$.c[1].b", "$.c[2].f"],
            { "$.c[1].b": 5, "$.c[2].f": 7 }
        }
        multiple_paths_test! {
            last_array_indexes,
            "last_array_indexes",
            { "a": 2, "c": [4, 5], "f": 7 },
            ["$.c[#-2]", "$.c[#-1]"],
            { "$.c[#-2]": 4, "$.c[#-1]": 5 }
        }
        multiple_paths_test! {
            bools,
            "bools",
            [false, false, false, true],
            ["2", "3"],
            { "2": false, "3": true }
        }
        multiple_paths_test! {
            numbers,
            "numbers",
            [11, 22, 33, 44],
            ["2", "3"],
            { "2": 33, "3": 44 }
        }
        multiple_paths_test! {
            strings,
            "strings",
            { "a": "xyz", "b": "abc" },
            ["$.a", "$.b"],
            { "$.a": "xyz", "$.b": "abc" }
        }
        multiple_paths_test! {
            nulls,
            "nulls",
            { "a": null, "b": null },
            ["$.a", "$.b"],
            { "$.a": null, "$.b": null }
        }
    }

    mod fail {
        use super::*;

        macro_rules! test {
            ($name:ident, $key:expr, $value:expr, $paths:expr, $want:pat $(if $guard:expr)?) => {
                #[async_std::test]
                async fn $name() -> Result<()> {
                    let service = Service::default();
                    let key = $key;
                    let value: Option<String> = $value;
                    let paths: Option<&[&str]> = $paths;

                    match value {
                        Some(value) => {
                            service.set(key, None, value).await?;
                        }
                        None => {}
                    }

                    let result = service.get(key, paths).await;
                    dbg!(&result);

                    assert!(matches!(result, $want $(if $guard)?));

                    Ok(())
                }
            };
        }

        test! {
            key_not_found_without_path,
            "key_not_found_without_path",
            None,
            None,
            Err(GetError::KeyNotFound(key)) if key == "key_not_found_without_path"
        }
        test! {
            key_not_found_with_root_path,
            "key_not_found_with_root_path",
            None,
            Some(&["$"]),
            Err(GetError::KeyNotFound(key)) if key == "key_not_found_with_root_path"
        }
        test! {
            key_not_found_with_specific_path,
            "key_not_found_with_specific_path",
            None,
            Some(&["$.key"]),
            Err(GetError::KeyNotFound(key)) if key == "key_not_found_with_specific_path"
        }
        test! {
            paths_not_found_single,
            "paths_not_found_single",
            Some(json!({}).to_string()),
            Some(&["$.key"]),
            Err(GetError::PathsNotFound(paths)) if paths == ["$.key"]
        }
        test! {
            paths_not_found_multiple_without_match,
            "paths_not_found_multiple_without_match",
            Some(json!({}).to_string()),
            Some(&["$.key1", "$.key2", "$.key3"]),
            Err(GetError::PathsNotFound(paths)) if paths == ["$.key1", "$.key2", "$.key3"]
        }
        test! {
            paths_not_found_multiple_with_match,
            "paths_not_found_multiple_with_match",
            Some(json!({ "key1": "value1" }).to_string()),
            Some(&["$.key1", "$.key2", "$.key3"]),
            Err(GetError::PathsNotFound(paths)) if paths == ["$.key2", "$.key3"]
        }
        test! {
            paths_not_found_multiple_with_repetition,
            "paths_not_found_multiple_with_repetition",
            Some(json!({ "key1": "value1" }).to_string()),
            Some(&["$.key1", "$.key2", "$.key2"]),
            Err(GetError::PathsNotFound(paths)) if paths == ["$.key2"]
        }
    }
}

mod sets_the_value {
    use super::*;

    mod without_path {
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

        test! {
            null,
            "key",
            null
        }
        test! {
            bool_false,
            "   key   ",
            false
        }
        test! {
            bool_true,
            "camelCaseKey",
            true
        }
        test! {
            integer_negative,
            "PascalCaseKey",
            (-1)
        }
        test! {
            integer_negative_zero,
            "snake_case_key",
            (-0)
        }
        test! {
            integer_zero,
            "SNAKE_UPPERCASE_KEY",
            0
        }
        test! {
            integer_positive,
            "kebab-case-key",
            1
        }
        test! {
            float_negative,
            "dot.case.key",
            (-1.0)
        }
        test! {
            float_negative_zero,
            "float_negative_zero",
            (-0.0)
        }
        test! {
            float_zero,
            "float_zero",
            0.0
        }
        test! {
            float_positive,
            "float_positive",
            1.0
        }
        test! {
            string_empty,
            "string_empty",
            ""
        }
        test! {
            string_normal,
            "key",
            "value"
        }
        test! {
            string_space,
            "key space",
            "value space"
        }
        test! {
            string_tab,
            "key\ttab",
            "value\ttab"
        }
        test! {
            string_newline,
            "key\nnewline",
            "value\nnewline"
        }
        test! {
            array_empty,
            "array_empty",
            []
        }
        test! {
            array_one_element,
            "array_one_element",
            ["value"]
        }
        test! {
            array_multiple_elements,
            "array_multiple_elements",
            ["value1", "value2", "value3"]
        }
        test! {
            object_empty,
            "object_empty",
            {}
        }
        test! {
            object_one_key,
            "object_one_key",
            { "key": "value" }
        }
        test! {
            object_multiple_keys,
            "object_multiple_keys",
            { "key1": "value1", "key2": "value2", "key3": "value3" }
        }
        test! {
            replaces_value,
            "key",
            "value1", "key", "value2", "key", "value3"
        }
    }

    mod with_path {
        use super::*;

        macro_rules! test {
            ($name:ident, $($key:expr, $path:expr, $value:tt, $want:tt),+) => {
                #[async_std::test]
                async fn $name() -> Result<()> {
                    let service = Service::default();

                    $({
                        let key = $key;
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
                ($name:ident, $key:expr, $value:tt) => {
                    test!($name, $key, "$", $value, $value);
                };
            }

            root_test! {
                null,
                "key",
                null
            }
            root_test! {
                bool_false,
                "   key   ",
                false
            }
            root_test! {
                bool_true,
                "camelCaseKey",
                true
            }
            root_test! {
                integer_negative,
                "PascalCaseKey",
                (-1)
            }
            root_test! {
                integer_negative_zero,
                "snake_case_key",
                (-0)
            }
            root_test! {
                integer_zero,
                "SNAKE_UPPERCASE_KEY",
                0
            }
            root_test! {
                integer_positive,
                "kebab-case-key",
                1
            }
            root_test! {
                float_negative,
                "dot.case.key",
                (-1.0)
            }
            root_test! {
                float_negative_zero,
                "float_negative_zero",
                (-0.0)
            }
            root_test! {
                float_zero,
                "float_zero",
                0.0
            }
            root_test! {
                float_positive,
                "float_positive",
                1.0
            }
            root_test! {
                string_empty,
                "string_empty",
                ""
            }
            root_test! {
                string_normal,
                "key",
                "value"
            }
            root_test! {
                string_space,
                "key space",
                "value space"
            }
            root_test! {
                string_tab,
                "key\ttab",
                "value\ttab"
            }
            root_test! {
                string_newline,
                "key\nnewline",
                "value\nnewline"
            }
            root_test! {
                array_empty,
                "array_empty",
                []
            }
            root_test! {
                array_one_element,
                "array_one_element",
                ["value"]
            }
            root_test! {
                array_multiple_elements,
                "array_multiple_elements", ["value1",
                "value2", "value3"]
            }
            root_test! {
                object_empty,
                "object_empty",
                {}
            }
            root_test! {
                object_one_key,
                "object_one_key",
                { "key": "value" }
            }
            root_test! {
                object_multiple_keys,
                "object_multiple_keys", { "key1": "value1",
                "key2": "value2", "key3": "value3" }
            }
        }

        mod specific {
            use super::*;

            macro_rules! specific_test {
                ($name:ident, $key:expr, $initial:tt, $path:expr, $value:tt, $want:tt) => {
                    test!($name, $key, "$", $initial, $initial, $key, $path, $value, $want);
                };
            }

            specific_test!(
                last_array_index,
                "last_array_index",
                [1, 2, 3, 4],
                "$[#]",
                99,
                [1, 2, 3, 4, 99]
            );
            specific_test!(
                nested_last_array_index,
                "nested_last_array_index",
                [1, [2, 3], 4],
                "$[1][#]",
                99,
                [1, [2, 3, 99], 4]
            );
            specific_test! {
                replaces_value,
                "replaces_value",
                { "a": 2, "c": 4 },
                "$.a",
                99,
                { "a": 99, "c": 4 }
            }
            specific_test! {
                inserts_value,
                "inserts_value",
                { "a": 2, "c": 4 },
                "$.e",
                99,
                { "a": 2, "c": 4, "e": 99 }
            }
            specific_test! {
                decodes_once,
                "decodes_once",
                { "a": 2, "c": 4 },
                "$.c",
                [97, 96],
                { "a": 2, "c": [97, 96] }
            }
            specific_test! {
                doesnt_decode_twice,
                "doesnt_decode_twice",
                { "a": 2, "c": 4 },
                "$.c",
                "[97, 96]",
                { "a": 2, "c": "[97, 96]" }
            }
        }
    }

    mod fail {
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

        test! {
            key_not_found,
            "key_not_found",
            Some("$.key"),
            json!("value").to_string(),
            Err(SetError::KeyNotFound(key)) if key == "key_not_found"
        }
        test! {
            malformed_json,
            "malformed_json",
            None,
            "invalid",
            Err(SetError::InvalidJson(_))
        }
    }
}
