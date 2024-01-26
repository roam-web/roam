#[derive(Default, Clone)]
struct Document {
    elements: Vec<Element>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element>
#[derive(Clone, Copy)]
enum ElementKind {
    // Main root
    Html,

    // Document metadata
    Base,
    Head,
    Link,
    Meta,
    Style,
    Title,

    // Sectioning root
    Body,

    // Content sectioning
    Address,
    Article,
    Aside,
    Footer,
    Header,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    HGroup,
    Main,
    Nav,
    Section,
    Search,

    // Text content
    BlockQuote,
    Dd,
    Div,
    Dl,
    Dt,
    FigCaption,
    Figure,
    Hr,
    Li,
    Menu,
    Ol,
    P,
    Pre,
    Ul,

    // Inline text semantics
    A,
    Abbr,
    B,
    Bdi,
    Bdo,
    Br,
    Cite,
    Code,
    Data,
    Dfn,
    Em,
    I,
    Kbd,
    Mark,
    Q,
    Rp,
    Rt,
    Ruby,
    S,
    Samp,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Time,
    U,
    Var,
    Wbr,

    // Image and multimedia
    Area,
    Audio,
    Img,
    Map,
    Track,
    Video,

    // Embedded content
    Embed,
    IFrame,
    Object,
    Picture,
    Portal,
    Source,

    // SVG and MathML
    Svg,
    Math,

    // Scripting
    Canvas,
    NoScript,
    Script,

    // Demarcating edits
    Del,
    Ins,

    // Table content
    Caption,
    Col,
    ColGroup,
    Table,
    TBody,
    Td,
    TFoot,
    Th,
    Thead,
    Tr,

    // Forms
    Button,
    DataList,
    FieldSet,
    Form,
    Input,
    Label,
    Legend,
    Meter,
    OptGroup,
    Option,
    Output,
    Progress,
    Select,
    TextArea,

    // Interactive elements
    Details,
    Dialog,
    Summary,

    // Web components
    Slot,
    Template,
    // TODO: Should we support depcrated DOM element types?
}

#[derive(Clone)]
struct Element {
    kind: ElementKind,
    attributes: Vec<String>,
    children: usize,
    parent: usize,
}

const TEST_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en_US">
        <body>
            <p>This is a paragraph.</p>
            <p>This is another paragraph.</p>
            <a href="https://www.roblox.com/">Test</a>
            <a href="https://www.google.com/" />
            <br>
        </body>
    </html>
"#;

use quick_xml::{
    events::Event,
    reader::Reader,
};

pub fn test_parse_html() {
    let mut reader = Reader::from_str(TEST_HTML);
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    reader.check_end_names(false);

    let mut count = 0;

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at {}: {:?}", reader.buffer_position(), e),
            Ok(Event::DocType(e)) => {
                println!("DOCTYPE: {}", e.unescape().unwrap().into_owned());
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let n = e.name();
                let name = String::from_utf8_lossy(n.as_ref());
                println!("OPEN: {name}");

                for attr in e.attributes() {
                    let attr = attr.unwrap();
                    let key = String::from_utf8_lossy(attr.key.as_ref());
                    let value = String::from_utf8_lossy(attr.value.as_ref());
                    println!("ATTR: '{}' = '{}'", key, value);
                }
            },
            Ok(Event::Text(e)) => println!("TEXT: {}", e.unescape().unwrap().into_owned()),
            Ok(Event::End(e)) => {
                let n = e.name();
                let name = String::from_utf8_lossy(n.as_ref());
                println!("CLOSE: {name}");
            }
            _ => {}
        }
    }
}
