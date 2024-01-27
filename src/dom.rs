use std::collections::HashMap;

#[derive(Default, Clone)]
struct Document {
    elements: Vec<Element>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element>
#[derive(Debug, Clone)]
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
    THead,
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

    // Deprecated elements
    Acronym,
    Big,
    Center,
    Content,
    Dir,
    Font,
    Frame,
    FrameSet,
    Image,
    Marquee,
    MenuItem,
    NoBr,
    NoEmbed,
    NoFrames,
    Param,
    PlainText,
    Rb,
    Rtc,
    Shadow,
    Strike,
    Tt,
    Xmp,

    // Custom
    Custom(String),
}

#[derive(Debug, Clone)]
struct Element {
    kind: ElementKind,
    attributes: HashMap<String, String>,
    children: usize,
    parent: usize,
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        // TODO: Probably find a better way of doing this but I'm lazy
        let kind = match s {
            "html" => ElementKind::Html,
            
            "base" => ElementKind::Base,
            "head" => ElementKind::Head,
            "link" => ElementKind::Link,
            "meta" => ElementKind::Meta,
            "style" => ElementKind::Style,
            "title" => ElementKind::Title,

            "body" => ElementKind::Body,

            "address" => ElementKind::Address,
            "article" => ElementKind::Article,
            "aside" => ElementKind::Aside,
            "footer" => ElementKind::Footer,
            "header" => ElementKind::Header,
            "h1" => ElementKind::H1,
            "h2" => ElementKind::H2,
            "h3" => ElementKind::H3,
            "h4" => ElementKind::H4,
            "h5" => ElementKind::H5,
            "h6" => ElementKind::H6,
            "hgroup" => ElementKind::HGroup,
            "main" => ElementKind::Main,
            "nav" => ElementKind::Nav,
            "section" => ElementKind::Section,
            "search" => ElementKind::Search,

            "blockquote" => ElementKind::BlockQuote,
            "dd" => ElementKind::Dd,
            "div" => ElementKind::Div,
            "dl" => ElementKind::Dl,
            "dt" => ElementKind::Dt,
            "figcaption" => ElementKind::FigCaption,
            "figure" => ElementKind::Figure,
            "hr" => ElementKind::Hr,
            "li" => ElementKind::Li,
            "menu" => ElementKind::Menu,
            "ol" => ElementKind::Ol,
            "p" => ElementKind::P,
            "pre" => ElementKind::Pre,
            "ul" => ElementKind::Ul,

            "a" => ElementKind::A,
            "abbr" => ElementKind::Abbr,
            "b" => ElementKind::B,
            "bdi" => ElementKind::Bdi,
            "bdo" => ElementKind::Bdo,
            "br" => ElementKind::Br,
            "cite" => ElementKind::Cite,
            "code" => ElementKind::Code,
            "data" => ElementKind::Data,
            "dfn" => ElementKind::Dfn,
            "em" => ElementKind::Em,
            "i" => ElementKind::I,
            "kbd" => ElementKind::Kbd,
            "mark" => ElementKind::Mark,
            "q" => ElementKind::Q,
            "rp" => ElementKind::Rp,
            "rt" => ElementKind::Rt,
            "ruby" => ElementKind::Ruby,
            "s" => ElementKind::S,
            "samp" => ElementKind::Samp,
            "small" => ElementKind::Small,
            "span" => ElementKind::Span,
            "strong" => ElementKind::Strong,
            "sub" => ElementKind::Sub,
            "sup" => ElementKind::Sup,
            "time" => ElementKind::Time,
            "u" => ElementKind::U,
            "var" => ElementKind::Var,
            "wbr" => ElementKind::Wbr,

            "area" => ElementKind::Area,
            "audio" => ElementKind::Audio,
            "img" => ElementKind::Img,
            "map" => ElementKind::Map,
            "track" => ElementKind::Track,
            "video" => ElementKind::Video,

            "embed" => ElementKind::Embed,
            "iframe" => ElementKind::IFrame,
            "object" => ElementKind::Object,
            "picture" => ElementKind::Picture,
            "portal" => ElementKind::Portal,
            "source" => ElementKind::Source,

            "svg" => ElementKind::Svg,
            "math" => ElementKind::Math,

            "canvas" => ElementKind::Canvas,
            "noscript" => ElementKind::NoScript,
            "script" => ElementKind::Script,

            "del" => ElementKind::Del,
            "ins" => ElementKind::Ins,

            "caption" => ElementKind::Caption,
            "col" => ElementKind::Col,
            "colgroup" => ElementKind::ColGroup,
            "table" => ElementKind::Table,
            "tbody" => ElementKind::TBody,
            "td" => ElementKind::Td,
            "tfoot" => ElementKind::TFoot,
            "th" => ElementKind::Th,
            "thead" => ElementKind::THead,
            "tr" => ElementKind::Tr,

            "button" => ElementKind::Button,
            "datalist" => ElementKind::DataList,
            "fieldset" => ElementKind::FieldSet,
            "form" => ElementKind::Form,
            "input" => ElementKind::Input,
            "label" => ElementKind::Label,
            "legend" => ElementKind::Legend,
            "meter" => ElementKind::Meter,
            "optgroup" => ElementKind::OptGroup,
            "option" => ElementKind::Option,
            "output" => ElementKind::Output,
            "progress" => ElementKind::Progress,
            "select" => ElementKind::Select,
            "textarea" => ElementKind::TextArea,

            "details" => ElementKind::Details,
            "dialog" => ElementKind::Dialog,
            "summary" => ElementKind::Summary,

            "slot" => ElementKind::Slot,
            "template" => ElementKind::Template,

            "acronym" => ElementKind::Acronym,
            "big" => ElementKind::Big,
            "center" => ElementKind::Center,
            "content" => ElementKind::Content,
            "dir" => ElementKind::Dir,
            "font" => ElementKind::Font,
            "frame" => ElementKind::Frame,
            "frameset" => ElementKind::FrameSet,
            "image" => ElementKind::Image,
            "marquee" => ElementKind::Marquee,
            "menuitem" => ElementKind::MenuItem,
            "nobr" => ElementKind::NoBr,
            "noembed" => ElementKind::NoEmbed,
            "noframes" => ElementKind::NoFrames,
            "param" => ElementKind::Param,
            "plaintext" => ElementKind::PlainText,
            "rb" => ElementKind::Rb,
            "rtc" => ElementKind::Rtc,
            "shadow" => ElementKind::Shadow,
            "strike" => ElementKind::Strike,
            "tt" => ElementKind::Tt,
            "xmp" => ElementKind::Xmp,

            _ => ElementKind::Custom(String::from(s)),
        };

        Self {
            kind,
            attributes: HashMap::default(),
            children: 0,
            parent: 0,
        }
    }
}

const TEST_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en_US">
        <body>
            <p>This is a paragraph.</p>
            <p>This is another paragraph.</p>
            <a href="https://www.roblox.com/">Test</a>
            <a href="https://www.google.com/" />
            <br />
            <div>
                Hello
                <h1>Test</h1>
            </div>
        </body>
    </html>
"#;

use quick_xml::{
    events::Event,
    reader::Reader,
};

// TODO: Abstract everything into an API, also remove unecessary allocations
pub fn test_parse_html() {
    let mut reader = Reader::from_file("test.html").unwrap();
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    reader.check_end_names(false);

    let mut count = 0;
    let mut document = Document::default();

    let mut current_parent = Vec::<usize>::new();
    let mut index = 0;

    let mut buf = Vec::new();

    let mut in_script = false;
    let mut in_svg = false;

    loop {
        // println!("---");
        // println!("INDEX: {index}");
        // for e in &document.elements {
        //     println!("{e:?}");
        // }
        // println!("---");

        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at {}: {:?}", reader.buffer_position(), e),
            Ok(Event::DocType(e)) => {
                println!("DOCTYPE: {}", e.unescape().unwrap().into_owned());
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                if in_script || in_svg {
                    continue;
                }

                let n = e.name();
                let name = String::from_utf8_lossy(n.as_ref()).to_lowercase();

                // println!("{}, {}", reader.buffer_position(), in_svg);
                let mut element = Element::from(name.as_str());

                for attr in e.html_attributes().with_checks(false) {
                    let attr = attr.unwrap();
                    let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
                    let value = String::from_utf8_lossy(attr.value.as_ref()).into_owned();

                    element.attributes.insert(key, value);
                }

                match element.kind {
                    ElementKind::Script => in_script = true,
                    ElementKind::Svg => in_svg = true,
                    _ => {},
                }

                // println!("OPEN: {:?}, {}", element.kind, in_svg);

                if current_parent.len() > 0 {
                    document.elements[*current_parent.last().unwrap()].children += 1;
                }

                element.parent = *current_parent.last().unwrap_or(&0);
                current_parent.push(index);

                document.elements.push(element);

                index += 1;
            },
            // Ok(Event::Text(e)) => println!("TEXT: {}", e.unescape().unwrap().into_owned()),
            Ok(Event::End(e)) => {
                let n = e.name();
                let name = String::from_utf8_lossy(n.as_ref()).into_owned();

                let temp = Element::from(name.as_str());

                // println!("CLOSE: {:?}", temp.kind);
                
                match temp.kind {
                    ElementKind::Script => in_script = false,
                    ElementKind::Svg => in_svg = false,
                    _ => {}
                }

                // TODO: Check closing tag
                if !in_svg && !in_script {
                    current_parent.pop().unwrap();
                }
            }
            _ => {}
        }
    }

    println!("---");
    for e in &document.elements {
        println!("{e:?}");
    }
    println!("---");

    println!("ELEMENT COUNT: {}", document.elements.len());
    println!("DOM MEMORY CONSUMPTION: {} bytes", document.elements.len() * core::mem::size_of::<Element>());
}
