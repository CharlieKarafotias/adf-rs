use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Node {
    Blockquote {
        content: Vec<Node>,
    },
    BulletList {
        content: Vec<Node>,
    },
    CodeBlock {
        content: Option<Vec<Node>>,
        attrs: Option<CodeBlockAttrs>,
    },
    Date {
        attrs: DateAttrs,
    },
    Doc {
        version: i8,
        content: Vec<Node>,
    },
    Emoji {
        attrs: EmojiAttrs,
    },
    Expand {
        content: Vec<Node>,
        attrs: ExpandAttrs,
        marks: Option<Vec<Mark>>,
    },
    HardBreak {
        attrs: Option<HardBreakAttrs>,
    },
    Heading {
        content: Vec<Node>,
        attrs: HeadingAttrs,
    },
    InlineCard {
        attrs: InlineCardAttrs,
    },
    ListItem {
        content: Vec<Node>,
    },
    Media {
        attrs: MediaAttrs,
    },
    MediaGroup {
        content: Vec<Node>,
    },
    MediaSingle {
        content: Vec<Node>,
        attrs: MediaSingleAttrs,
    },
    Mention {
        attrs: MentionAttrs,
    },
    NestedExpand {
        content: Vec<Node>,
        attrs: ExpandAttrs,
    },
    OrderedList {
        content: Vec<Node>,
        attrs: Option<OrderedListAttrs>,
    },
    Panel {
        content: Vec<Node>,
        attrs: PanelAttrs,
    },
    Paragraph {
        content: Vec<Node>,
        attrs: Option<ParagraphAttrs>,
    },
    Rule,
    Status {
        attrs: StatusAttrs,
    },
    Table {
        content: Vec<Node>,
        attrs: Option<TableAttrs>,
    },
    TableCell {
        content: Vec<Node>,
        attrs: Option<TableCellAttrs>,
    },
    TableHeader {
        content: Vec<Node>,
        attrs: Option<TableCellAttrs>,
    },
    TableRow {
        content: Vec<Node>,
    },
    Text {
        text: String,
        marks: Option<Vec<Mark>>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Mark {
    BackgroundColor {
        attrs: ColorAttrs,
    },
    Code,
    Em,
    Link {
        attrs: LinkAttrs,
    },
    Strike,
    Strong,
    SubSup {
        attrs: SubSupAttrs,
    },
    TextColor {
        attrs: ColorAttrs,
    },
    Underline,
}

// ------ Attrs -------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ColorAttrs {
    pub(crate) color: String, // defined in HTML hexadecimal format (e.g. #daa520)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CodeBlockAttrs {
    pub(crate) language: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DateAttrs {
    // Example: 1582152559
    // Unix timestamp in seconds
    pub(crate) timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmojiAttrs {
    pub(crate) short_name: String,
    pub(crate) id: Option<String>,
    pub(crate) text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExpandAttrs {
    pub(crate) title: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HardBreakAttrs {
    pub(crate) text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeadingAttrs {
    pub(crate) level: i8,
    pub(crate) local_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineCardAttrs {
    pub(crate) data: Option<String>, // TODO: uses https://json-ld.org/ for structured data
    pub(crate) url: Option<String>, // TODO: according to website this is an object type?
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LinkAttrs {
    pub(crate) collection: Option<String>,
    pub(crate) href: String,
    pub(crate) id: Option<String>,
    pub(crate) occurrence_key: Option<String>,
    pub(crate) title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MediaAttrs {
    #[serde(rename = "type")]
    pub(crate) type_: String,
    pub(crate) width: Option<u32>,
    pub(crate) height: Option<u32>,
    pub(crate) id: String,
    pub(crate) collection: String,
    pub(crate) occurrence_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MediaSingleAttrs {
    pub(crate) layout: String,
    pub(crate) width: Option<f32>, // float between 0 and 100
    pub(crate) width_type: Option<String>, // TODO: ENUM either pixel or percentage
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MentionAttrs {
    pub(crate) id: String,
    pub(crate) text: Option<String>,
    pub(crate) user_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrderedListAttrs {
    pub(crate) order: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PanelAttrs {
    pub(crate) panel_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphAttrs {
    pub(crate) local_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatusAttrs {
    pub(crate) local_id: Option<String>,
    pub(crate) text: String,
    pub(crate) color: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableAttrs {
    pub(crate) display_mode: Option<String>,
    pub(crate) is_number_column_enabled: Option<bool>,
    pub(crate) layout: Option<String>,
    pub(crate) width: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellAttrs {
    pub(crate) background: Option<String>,
    pub(crate) colspan: Option<u16>,
    pub(crate) colwidth: Option<Vec<u16>>,
    pub(crate) rowspan: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum SubSupAttrs {
    Sup,
    Sub,
}

// ------ Attrs -------