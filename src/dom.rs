struct Document {
    elements: Vec<Element>,
}

/// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
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

struct Element {
    kind: ElementKind,
    // TODO: Define tag type soon :tm:
    tags: Vec<String>,
}
