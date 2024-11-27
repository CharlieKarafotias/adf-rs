use crate::model;

#[allow(dead_code)]
pub fn from_str<S: Into<String>>(input: S) -> model::Node {
    serde_json::from_str(&input.into()).expect("Failed to parse JSON")
}

pub fn from_value(input: serde_json::Value) -> model::Node {
    serde_json::from_value(input).expect("Failed to parse JSON")
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn from_value_and_from_str_should_return_same_result() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                      {
                        "type": "text",
                        "text": "Hello world"
                      }
                    ]
                }
            ]
        }"#;

        let from_str_result = from_str(input);
        let from_value_result = from_value(serde_json::from_str(input).unwrap());

        assert_eq!(from_str_result, from_value_result);
    }

    #[test]
    fn should_read_blockquote() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "blockquote",
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": "Hello world"
                            }
                        ]
                    }
                ]
            }]
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![model::Node::Blockquote {
                    content: vec![model::Node::Paragraph {
                        content: vec![model::Node::Text {
                            text: "Hello world".to_string(),
                            marks: None
                        }],
                        attrs: None
                    }]
                }]
            }
        );
    }

    #[test]
    fn should_read_bullet_list() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "bulletList",
                "content": [
                    {
                    "type": "listItem",
                    "content": [
                        {
                        "type": "paragraph",
                        "content": [
                            {
                            "type": "text",
                            "text": "Hello world"
                            }
                        ]
                        }
                    ]
                    }
                ]
            }]
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![model::Node::BulletList {
                    content: vec![model::Node::ListItem {
                        content: vec![model::Node::Paragraph {
                            content: vec![model::Node::Text {
                                text: "Hello world".to_string(),
                                marks: None
                            }],
                            attrs: None
                        }]
                    }]
                }]
            }
        );
    }

    #[test]
    fn should_read_code_block() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "codeBlock",
                "attrs": {
                    "language": "javascript"
                },
                "content": [
                    {
                    "type": "text",
                    "text": "var foo = {};\nvar bar = [];"
                    }
                ]
            }]
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![
                    model::Node::CodeBlock {
                        content: vec![
                            model::Node::Text {
                                text: "var foo = {};\nvar bar = [];".to_string(),
                                marks: None
                            }
                        ].into(),
                        attrs: Some(
                            model::CodeBlockAttrs {
                                language: "javascript".to_string(),
                            }
                        )
                    }
                ]
            }
        );
    }

    #[test]
    fn should_read_date() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [
                    {
                        "type": "date",
                        "attrs": {
                            "timestamp": "1582152559"
                        }
                    }
                ]
            }]
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![
                    model::Node::Paragraph {
                        content: vec![
                            model::Node::Date {
                                attrs: model::DateAttrs {
                                    timestamp: "1582152559".to_string(),
                                }
                            }
                        ],
                        attrs: None,
                    }
                ]
            }
        );
    }

    #[test]
    fn should_read_doc() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": []
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![]
            }
        );
    }

    #[test]
    fn should_read_unicode_emoji() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [
                    {
                       "type": "emoji",
                        "attrs": {
                            "shortName": ":grinning:",
                            "text": "😀"
                        }
                    }
                ]
            }]
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![
                    model::Node::Paragraph {
                        content: vec![
                            model::Node::Emoji {
                                attrs: model::EmojiAttrs {
                                    id: None,
                                    short_name: ":grinning:".to_string(),
                                    text: Some("😀".to_string()),
                                }
                            }
                        ],
                        attrs: None,
                    }
                ]
            }
        );
    }

    #[test]
    fn should_read_non_standard_atlassian_emoji() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [
                    {
                        "type": "emoji",
                        "attrs": {
                            "shortName": ":awthanks:",
                            "id": "atlassian-awthanks",
                            "text": ":awthanks:"
                        }
                    }
                ]
            }]
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![
                    model::Node::Paragraph {
                        content: vec![
                            model::Node::Emoji {
                                attrs: model::EmojiAttrs {
                                    id: Some("atlassian-awthanks".to_string()),
                                    short_name: ":awthanks:".to_string(),
                                    text: Some(":awthanks:".to_string()),
                                }
                            }
                        ],
                        attrs: None,
                    }
                ]
            }
        );
    }

    #[test]
    fn should_read_non_standard_customer_emoji() {
        let input = r#"{
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [
                    {
                        "type": "emoji",
                        "attrs": {
                            "shortName": ":thumbsup::skin-tone-2:",
                            "id": "1f44d",
                            "text": "👍🏽"
                        }
                    }
                ]
            }]
        }"#;

        let from_str_result = from_str(input);
        assert_eq!(
            from_str_result,
            model::Node::Doc {
                version: 1,
                content: vec![
                    model::Node::Paragraph {
                        content: vec![
                            model::Node::Emoji {
                                attrs: model::EmojiAttrs {
                                    id: Some("1f44d".to_string()),
                                    short_name: ":thumbsup::skin-tone-2:".to_string(),
                                    text: Some("👍🏽".to_string()),
                                }
                            }
                        ],
                        attrs: None,
                    }
                ]
            }
        );
    }

    // TODO: let off testing at expand: https://developer.atlassian.com/cloud/jira/platform/apis/document/nodes/expand/
}