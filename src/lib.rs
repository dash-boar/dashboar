pub use json_patch;

use json_patch::Patch;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HeadingFormat {
    H6,
    H5,
    H4,
    H3,
    H2,
    H1,
}

impl Display for HeadingFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LinkStyle {
    Primary,
    Secondary,
    Contrast,
}

impl Display for LinkStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Contrast,
    Outline,
}

impl Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InlineStyle {
    Unstyled,
    Bold,
    Italic,
    Underline,
    Deleted,
    Inserted,
    StrikeThrough,
    Small,
    Sub,
    Sup,
    Abbr,
    Kbd,
    Highlighted,
}

impl InlineStyle {
    pub fn to_node<'a>(&'a self) -> &'a str {
        match self {
            Self::Kbd => "kbd",
            Self::Sub => "sub",
            Self::Sup => "sup",
            Self::Abbr => "abbr",
            Self::Bold => "strong",
            Self::Small => "small",
            Self::Italic => "em",
            Self::Deleted => "del",
            Self::Inserted => "ins",
            Self::Unstyled => "text",
            Self::Underline => "u",
            Self::Highlighted => "mark",
            Self::StrikeThrough => "s",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Value<T> {
    Fixed(T),
    Pointer(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "ui")]
pub enum Ui {
    Heading(Heading),
    Text(Text),
    Link(Link),
    Number(Number),
    Button(Button),
    BoolButton(BoolButton),
    Tabs(Tabs),
    Grid(Grid),
    Div(Div),
    Table(Table),
    TableFromData(TableFromData),
    Form(Form),
    Image(Image),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "td")]
pub enum Td {
    Heading(Heading),
    Text(Text),
    Link(Link),
    Number(Number),
    Button(Button),
    BoolButton(BoolButton),
    Image(Image),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
    F64,
    Percentage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Disabled {
    Disabled,
    Condition(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "version", content = "layout")]
pub enum Layout {
    V0(Vec<Ui>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DashboarRx {
    Layout(Layout),
    DataSnapshot(serde_json::Value),
    DataPatch(Patch),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DashboarTx {
    Msg { template: serde_json::Value },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ButtonState {
    pub value: Value<String>,
    pub color: String,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState {
            value: Value::Fixed(String::from("")),
            color: String::from("var(--secondary)"),
        }
    }
}

impl ButtonState {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn value(&mut self, value: Value<String>) -> &mut Self {
        self.value = value;
        self
    }

    pub fn color(&mut self, color: String) -> &mut Self {
        self.color = color;
        self
    }

    pub fn build(&mut self) -> Self {
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct BoolButtonState {
    pub on: ButtonState,
    pub off: ButtonState,
}

impl BoolButtonState {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn on(&mut self, on: ButtonState) -> &mut Self {
        self.on = on;
        self
    }

    pub fn off(&mut self, off: ButtonState) -> &mut Self {
        self.off = off;
        self
    }

    pub fn build(&mut self) -> Self {
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Heading {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<HeadingFormat>,
}

impl Heading {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn value(&mut self, value: Value<String>) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn format(&mut self, format: HeadingFormat) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Heading(self.to_owned())
    }

    pub fn build_td(&mut self) -> Td {
        Td::Heading(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<InlineStyle>,
}

impl Text {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn value(&mut self, value: Value<String>) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn style(&mut self, style: InlineStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Text(self.to_owned())
    }

    pub fn build_td(&mut self) -> Td {
        Td::Text(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Value<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<LinkStyle>,
}

impl Link {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn value(&mut self, value: Value<String>) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn href(&mut self, href: Value<String>) -> &mut Self {
        self.href = Some(href);
        self
    }

    pub fn style(&mut self, style: LinkStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Link(self.to_owned())
    }

    pub fn build_td(&mut self) -> Td {
        Td::Link(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<Value<String>>,
}

impl Image {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn src(&mut self, src: Value<String>) -> &mut Self {
        self.src = Some(src);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Image(self.to_owned())
    }

    pub fn build_td(&mut self) -> Td {
        Td::Image(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Number {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value<serde_json::Number>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<InlineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<NumberFormat>,
}

impl Number {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn value(&mut self, value: Value<serde_json::Number>) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn style(&mut self, style: InlineStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn format(&mut self, format: NumberFormat) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Number(self.to_owned())
    }

    pub fn build_td(&mut self) -> Td {
        Td::Number(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Button {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_click: Option<DashboarTx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Disabled>,
}

impl Button {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn value(&mut self, value: Value<String>) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn on_click(&mut self, on_click: DashboarTx) -> &mut Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn style(&mut self, style: ButtonStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn disabled(&mut self, disabled: Disabled) -> &mut Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Button(self.to_owned())
    }

    pub fn build_td(&mut self) -> Td {
        Td::Button(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct BoolButton {
    pub pointer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<BoolButtonState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_click: Option<DashboarTx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Disabled>,
}

impl BoolButton {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn pointer(&mut self, path: String) -> &mut Self {
        self.pointer = path;
        self
    }

    pub fn state(&mut self, state: BoolButtonState) -> &mut Self {
        self.state = Some(state);
        self
    }

    pub fn on_click(&mut self, on_click: DashboarTx) -> &mut Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn disabled(&mut self, disabled: Disabled) -> &mut Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::BoolButton(self.to_owned())
    }

    pub fn build_td(&mut self) -> Td {
        Td::BoolButton(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Tab {
    pub name: String,
    pub contents: Vec<Ui>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Tabs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Vec<Tab>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Grid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cell_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Ui>>,
}

impl Grid {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn max_height(&mut self, max_height: usize) -> &mut Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn min_cell_width(&mut self, min_cell_width: usize) -> &mut Self {
        self.min_cell_width = Some(min_cell_width);
        self
    }

    pub fn gap(&mut self, gap: usize) -> &mut Self {
        self.gap = Some(gap);
        self
    }

    pub fn children(&mut self, children: Vec<Ui>) -> &mut Self {
        self.children = Some(children);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Grid(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Div {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Ui>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<usize>,
}

impl Div {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn children(&mut self, children: Vec<Ui>) -> &mut Self {
        self.children = Some(children);
        self
    }

    pub fn max_height(&mut self, max_height: usize) -> &mut Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Div(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Table {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<Vec<Ui>>>,
}

impl Table {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn header(&mut self, header: Vec<String>) -> &mut Self {
        self.header = Some(header);
        self
    }

    pub fn body(&mut self, body: Vec<Vec<Ui>>) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Table(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct TableFromData {
    pub pointer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_template: Option<Vec<Td>>,
}

impl TableFromData {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn pointer(&mut self, pointer: String) -> &mut Self {
        self.pointer = pointer;
        self
    }

    pub fn header(&mut self, header: Vec<String>) -> &mut Self {
        self.header = Some(header);
        self
    }

    pub fn row_template(&mut self, row_template: Vec<Td>) -> &mut Self {
        self.row_template = Some(row_template);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::TableFromData(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Form {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<InputField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<DashboarTx>,
}

impl Form {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn fields(&mut self, fields: Vec<InputField>) -> &mut Self {
        self.fields = Some(fields);
        self
    }

    pub fn on_submit(&mut self, on_submit: DashboarTx) -> &mut Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub fn build_ui(&mut self) -> Ui {
        Ui::Form(self.to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextInputField {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CheckBoxInputField {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SelectInputField {
    pub name: String,
    pub label: String,
    pub options: Vec<SelectOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
pub enum InputField {
    Text(TextInputField),
    CheckBox(CheckBoxInputField),
    Select(SelectInputField),
}

impl InputField {
    pub fn name(&self) -> String {
        match &self {
            InputField::Text(f) => f.name.clone(),
            InputField::CheckBox(f) => f.name.clone(),
            InputField::Select(f) => f.name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SelectOption {
    pub text: String,
    pub value: String,
}

impl SelectOption {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }

    pub fn value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    pub fn build(&mut self) -> Self {
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Ws {
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_on_connect: Option<serde_json::Value>,
}
