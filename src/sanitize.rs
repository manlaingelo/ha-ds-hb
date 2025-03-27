use html5ever::QualName;
use html5ever::rcdom::{Element, NodeData, RcDom};
use html5ever::serialize::{SerializeOpts, TraversalScope, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::TreeSink;
use markup5ever_arcdom::SerializableHandle;
use scraper::{Html, Node, Selector};
use std::error::Error;
use std::fs;

fn sanitize_html_attributes(html_content: &str) -> Result<String, Box<dyn Error>> {
    let document = Html::parse_document(html_content);
    let mut dom = RcDom::default();
    let mut sink = dom.get_sink();

    // Re-parse the html, and then modify the DOM.
    sink.process(document.tree.as_ref().iter().cloned());

    let handle = dom.document;

    fn sanitize_node(handle: SerializableHandle) {
        match handle.borrow().node.data {
            NodeData::Element(ref mut element) => {
                element.attrs.retain(|attr| {
                    // Whitelist allowed attributes.
                    matches!(
                        attr.name.local.as_ref(),
                        "class" | "id" | "style" | "href" | "src" | "alt"
                    )
                });

                for child in handle.borrow().children.borrow().iter().cloned() {
                    sanitize_node(child);
                }
            }
            _ => {
                for child in handle.borrow().children.borrow().iter().cloned() {
                    sanitize_node(child);
                }
            }
        }
    }

    sanitize_node(handle.clone());

    let mut output = Vec::new();
    serialize(
        &mut output,
        handle,
        SerializeOpts {
            traversal_scope: TraversalScope::IncludeNode,
            ..Default::default()
        },
    )
    .unwrap();

    Ok(String::from_utf8(output)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let html_content = fs::read_to_string("html/sum-duureg.html")?;
    let sanitized_html = sanitize_html_attributes(&html_content)?;
    fs::write("html/sum-duureg-out.html", sanitized_html)?;
    println!("Sanitized HTML written to output.html");
    Ok(())
}
