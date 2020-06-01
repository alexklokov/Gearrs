pub mod gs_element;
use gs_element::GsElement;

pub fn create_header() -> GsElement {
    let meta_charset = GsElement::new("meta", false).add_attr("charset", "utf-8");
    let meta_viewport = GsElement::new("meta", false)
        .add_attr("name", "viewport")
        .add_attr("content", "width=device-width, initial-scale=1");

    GsElement::new("head", true)
        .add(&meta_viewport)
        .add(&meta_charset)
}

pub fn create_html(head: GsElement, body: GsElement) -> String {
    let head_html = head.convert();
    let body_html = body.convert();
    format!("<!DOCTYPE html>\n{}\n{}", head_html, body_html)
}

pub fn wrap(child: &GsElement) -> GsElement {
    GsElement::new("div", true).add(child)
}

#[cfg(test)]
mod tests {
    use super::gs_element::GsElement;
    use super::*;

    #[test]
    fn test_creating() {
        let body = GsElement::new("body", true);
        assert_eq!(body.tag, "body".to_string());
        assert_eq!(body.get_value(), "");
    }
    #[test]
    fn test_converting_to_html() {
        let mut body = GsElement::new("div", true);
        assert_eq!(body.convert(), "<div></div>".to_string());

        body.set_value("Hello world");
        assert_eq!(body.convert(), "<div>Hello world</div>".to_string());
    }

    #[test]
    fn test_add_children() {
        let p1 = GsElement::new("p", true);
        let p2 = GsElement::new("p", true);
        let div = GsElement::new("div", true).add(&p1).add(&p2);
        
        assert_eq!(2, div.get_children().len());
    }

    #[test]
    fn test_push_children() {
        let p1 = GsElement::new("p", true);
        let p2 = GsElement::new("p", true);

        let mut div = GsElement::new("div", true);
        div.push(&p1);
        div.push(&p2);

        assert_eq!(2, div.get_children().len());
    }

    #[test]
    fn test_deep_convert() {
        // Test 1
        let p1 = GsElement::new("p", true);
        let p2 = GsElement::new("p", true);
        let mut div = GsElement::new("div", true).add(&p1).add(&p2);

        assert_eq!("<div><p></p><p></p></div>".to_string(), div.convert());
        div.set_value("Hello world");
        assert_eq!(
            "<div>Hello world<p></p><p></p></div>".to_string(),
            div.convert()
        );

        // Test 2
        let mut span = GsElement::new("span", true);
        span.set_value("Hello span");
        let mut p = GsElement::new("p", true).add(&span);
        p.set_value("Hello paragraph");
        let mut div2 = GsElement::new("div", true).add(&p);
        div2.set_value("Hello world");

        assert_eq!(
            "<div>Hello world<p>Hello paragraph<span>Hello span</span></p></div>".to_string(),
            div2.convert()
        );

        // Test 3
        let input = GsElement::new("input", false)
            .add_attr("type", "text")
            .add_attr("name", "login")
            .add_attr("id", "login");
        let p = GsElement::new("p", true).add(&input);
        let mut p2 = GsElement::new("p", true);
        p2.set_value("Hello world");
        let div = GsElement::new("div", true).add(&p).add(&p2);

        assert_eq!(
            "<div><p><input type = \"text\" name = \"login\" id = \"login\"></p><p>Hello world</p></div>".to_string(),
            div.convert()
            );

        let div = GsElement::new("div", true).add_value("Hello world");
        
        let div2 = GsElement::new("div", true)
            .add_value("Привет мир")
            .add_value("Мир привет");


        assert_eq!("<div>Hello world</div>".to_string(), div.convert());
        assert_eq!("<div>World hello</div>".to_string(), div2.convert());
    }

    #[test]
    fn test_memory() {
        let p1 = GsElement::new("p", true);
        let p2 = GsElement::new("p", true);
        let span = GsElement::new("span", true);
        let div = GsElement::new("div", true).add(&p1).add(&p2).add(&span);

        let mut div2 = GsElement::new("div", true);
        div2.push(&p1);
        div2.push(&p2);

        assert_eq!(std::mem::size_of_val(&div), std::mem::size_of_val(&div2));
    }

    #[test]
    fn test_delete() {
        let p1 = GsElement::new("p", true);
        let span = GsElement::new("span", true);
        let mut div = GsElement::new("div", true).add(&p1).add(&span);

        div.remove(0).unwrap();
        assert_eq!(1, div.get_children().len());
        assert_eq!("<div><span></span></div>".to_string(), div.convert());
    }

    #[test]
    fn test_create_from() {
        //Create from tuple (&str, bool)

        let div = GsElement::from(("div", true));
        assert_eq!("<div></div>".to_string(), div.convert());

        //Create from tuple (String, bool)

        let div = GsElement::from((String::from("div"), true));
        assert_eq!("<div></div>".to_string(), div.convert());
    }

    #[test]
    fn test_error_handle() {
        let mut div = GsElement::new("div", true);
        let result = div.remove(0);
        assert_eq!(Err("Index out of bounds"), result);

        let mut text_input = GsElement::new("input", false);
        let result = text_input.push(&GsElement::new("div", true));
        assert_eq!(Err("This is unclosed tag"), result);

        let result = div.push(&text_input);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_attributes() {
        let div = GsElement::new("div", true)
            .add_attr("class", "my_div")
            .add_attr("id", "div_id");

        assert_eq!(2, div.get_attr().len());
        assert_eq!(
            "<div class = \"my_div\" id = \"div_id\"></div>".to_string(),
            div.convert()
        );

        let mut div = GsElement::new("div", true);
        div.push_attr("class", "my_div");
        div.push_attr("id", "div_id");
        assert_eq!(
            "<div class = \"my_div\" id = \"div_id\"></div>".to_string(),
            div.convert()
        );
    }
}
