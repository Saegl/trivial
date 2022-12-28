use crate::parser_controller::ParserController;

/// Representation of HTML Tag
#[derive(Debug)]
pub struct Tag {
    pub name: String,
    inner: Vec<Node>,
}

impl Tag {
    pub fn text(&self) -> String {
        let mut output = String::new();
        for node in &self.inner {
            let inner_text_chunk = match node {
                Node::Tag(t) => t.text(),
                Node::Text(t) => t.to_string(),
                _ => String::new(),
            };
            output += &inner_text_chunk;
        }
        output
    }
}

#[derive(Debug)]
pub struct NoEndTag {
    pub name: String,
}

impl NoEndTag {
    fn is_no_end_name(name: &str) -> bool {
        name == "input" || name == "meta" || name == "br"
    }
}

#[derive(Debug)]
pub enum Node {
    Tag(Tag),
    NoEndTag(NoEndTag),
    Text(String),
}

impl Node {
    pub fn name(&self) -> String {
        match self {
            Node::Tag(t) => t.name.clone(),
            Node::NoEndTag(t) => t.name.clone(),
            Node::Text(_) => "text".to_string(),
        }
    }
}

fn identifier(parser: &mut ParserController) -> String {
    let mut resp = String::new();
    while parser.current().is_alphabetic()
        || parser.current().is_digit(10)
    {
        resp.push(parser.current());
        parser.index += 1;
    }
    resp
}

/// closed_tag = "<" identifier "/>"
fn closed_tag(parser: &mut ParserController) -> String {
    parser.eat("</");
    parser.skip_ws();

    let tag_name = identifier(parser);

    parser.skip_ws();
    parser.eat(">");

    tag_name
}

/// open_tag = "<" identifier ">"
fn open_tag(parser: &mut ParserController) -> String {
    parser.eat("<");
    parser.skip_ws();

    let tag_name = identifier(parser);

    // println!("open tag {}", tag_name);

    // TODO parse attributes
    let _attributes_content = parser.take_until_not_equal('>');

    parser.skip_ws();
    parser.eat(">");

    tag_name
}

/// tag = open_tag content closed_tag
fn tag(parser: &mut ParserController) -> Node {
    let open_tag_name = open_tag(parser);

    if NoEndTag::is_no_end_name(&open_tag_name) {
        // println!("Short");
        return Node::NoEndTag(NoEndTag {
            name: open_tag_name,
        });
    }

    let content = content(parser);
    let closed_tag_name = closed_tag(parser);

    if open_tag_name != closed_tag_name {
        panic!(
            "Tag <{}> closed by </{}>",
            open_tag_name, closed_tag_name
        );
    }

    Node::Tag(Tag {
        name: open_tag_name,
        inner: content,
    })
}

fn text(parser: &mut ParserController) -> String {
    let value = parser.take_until_not_equal('<');
    value
        .trim()
        .replace("\n", " ")
        .replace("\r", " ")
        .to_string()
}

fn comment(parser: &mut ParserController) {
    while !parser.expect("-->") {
        parser.index += 1;
    }
    parser.eat("-->");
    parser.skip_ws();
}

/// content = (text | tag)*
fn content(parser: &mut ParserController) -> Vec<Node> {
    let mut resp = vec![];
    parser.skip_ws();
    while parser.index < parser.chars.len() {
        if parser.expect("<!--") {
            comment(parser);
        }
        if parser.expect("</") {
            break;
        } else if parser.chars[parser.index] != '<' {
            let text = text(parser);
            resp.push(Node::Text(text));
        } else {
            let tag = tag(parser);
            resp.push(tag);
        }
        parser.skip_ws();
    }
    resp
}

/// Parse HTML string
pub fn parse(source: &str) -> Vec<Node> {
    let mut parser = ParserController::from_str(source);
    let tags = content(&mut parser);
    tags
}

#[test]
fn empty_source() {
    let tags = parse("");
    assert_eq!(tags.len(), 0);
}

#[test]
fn empty_paragraph() {
    let tags = parse("<p></p>");
    assert_eq!(tags.len(), 1);

    let tag = &tags[0];
    assert_eq!(tag.name(), "p");
}

#[test]
fn p_in_p() {
    let tags = parse("<p><p></p></p>");
    assert_eq!(tags.len(), 1);
}

#[test]
fn one_paragraph() {
    let tags = parse("<p>Hi</p>");
    assert_eq!(tags.len(), 1);
}

#[test]
fn two_paragraphs() {
    let tags = parse("<p>First</p> <p>Second</p>");
    assert_eq!(tags.len(), 2);
}

#[test]
fn div() {
    let tags = parse("<div class=\"content\"></div>");
    assert_eq!(tags.len(), 1);
}

#[test]
fn input() {
    let tags = parse("<input class=\"content\">");
    assert_eq!(tags.len(), 1);

    let tag = &tags[0];
    assert_eq!(tag.name(), "input");
}
