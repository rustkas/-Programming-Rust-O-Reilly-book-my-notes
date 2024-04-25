use std::collections::HashMap;

#[macro_use]
mod macros;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
                        usize isize f32 f64);

#[cfg(test)]
mod tests {
    use crate::Json;

    #[test]
    fn json_with_rust_expressions() {
        const HELLO: &'static str = "hello";
        let macro_generated_value = json!({
            "math_works": (4 - 2 == 2),
            "en": HELLO,
            HELLO: "bonjour!"
        });
        let hand_coded_value = Json::Object(Box::new(
            vec![
                ("math_works".to_string(), Json::Boolean(true)),
                ("en".to_string(), Json::String("hello".to_string())),
                ("hello".to_string(), Json::String("bonjour!".to_string())),
            ]
            .into_iter()
            .collect(),
        ));
        assert_eq!(macro_generated_value, hand_coded_value);
    }

    // Tests from earlier in the chapter should actually pass with this macro.

    #[test]
    fn original_example() {
        let hand_coded_value = {
            let students = Json::Array(vec![
                Json::Object(Box::new(
                    vec![
                        ("name".to_string(), Json::String("Jim Blandy".to_string())),
                        ("class_of".to_string(), Json::Number(1926.0)),
                        (
                            "major".to_string(),
                            Json::String("Tibetan throat singing".to_string()),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                )),
                Json::Object(Box::new(
                    vec![
                        (
                            "name".to_string(),
                            Json::String("Jason Orendorff".to_string()),
                        ),
                        ("class_of".to_string(), Json::Number(1702.0)),
                        ("major".to_string(), Json::String("Knots".to_string())),
                    ]
                    .into_iter()
                    .collect(),
                )),
            ]);
            students
        };

        let macro_generated_value = {
            let students = json!([
                {
                    "name": "Jim Blandy",
                    "class_of": 1926,
                    "major": "Tibetan throat singing"
                },
                {
                    "name": "Jason Orendorff",
                    "class_of": 1702,
                    "major": "Knots"
                }
            ]);
            students
        };

        assert_eq!(macro_generated_value, hand_coded_value);
    }

    #[test]
    fn json_array_with_json_element() {
        let macro_generated_value = json!(
            [
                // valid JSON that doesn't match `$element:expr`
                {
                    "pitch": 440.0
                }
            ]
        );
        let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
            vec![("pitch".to_string(), Json::Number(440.0))]
                .into_iter()
                .collect(),
        ))]);
        assert_eq!(macro_generated_value, hand_coded_value);
    }

    #[test]
    fn json_monolith() {
        let width = 4.0;
        let desc = json!({
            "width": width,
            "height": (width * 9.0 / 4.0)
        });

        let hand_coded_value = Json::Object(Box::new(
            vec![
                ("width".to_string(), Json::Number(width)),
                ("height".to_string(), Json::Number(width * 9.0 / 4.0)),
            ]
            .into_iter()
            .collect(),
        ));
        assert_eq!(desc, hand_coded_value);
    }

    #[test]
    fn hygiene() {
        // The surprise is that *the macro works as-is*.
        // Rust renames the variable for you!

        let fields = "Fields, W.C.";
        let role = json!({
            "name": "Larson E. Whipsnade",
            "actor": fields
        });

        let hand_coded_value = Json::Object(Box::new(
            vec![
                (
                    "name".to_string(),
                    Json::String("Larson E. Whipsnade".to_string()),
                ),
                (
                    "actor".to_string(),
                    Json::String("Fields, W.C.".to_string()),
                ),
            ]
            .into_iter()
            .collect(),
        ));
        assert_eq!(role, hand_coded_value);
    }
}
