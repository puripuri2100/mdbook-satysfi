use scraper::node::Node;
use scraper::Html;

pub fn parse_html(html_code: &str) -> String {
  let html = Html::parse_fragment(html_code);
  let node_opt = html.tree.into_iter().next();
  let tag_name: String = match node_opt {
    None => "None".to_owned(),
    Some(node) => match node {
      Node::Element(element) => format!("{:?}", element.name()),
      _ => format!("{:?}", node),
    },
  };
  tag_name
}

#[test]
fn check_parse_html() {
  assert_eq!(
    "span".to_string(),
    parse_html(
      r#"<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />"#
    )
  )
}
