use std::collections::HashMap;
use std::convert::From;

pub struct GsHTML {}

#[derive(Debug, Clone, PartialEq)]
pub struct GsElement {
    pub tag: String,
    inner_html: String,
    pub is_closed: bool,
    children: Vec<GsElement>,
    attribs: HashMap<&'static str, String>,
}

impl GsElement {
    pub fn new(tag: &str, is_closed: bool) -> Self {
        GsElement {
            tag: String::from(tag),
            inner_html: String::new(),
            is_closed: is_closed,
            children: vec![],
            attribs: HashMap::new(),
        }
    }

    // Converting current Gearrs Element into HTML
    pub fn convert(&self) -> String {
        let end_tag: String = if self.is_closed {
            format!("</{}>", self.tag)
        } else {
            String::new()
        };

        let content = if self.children.len() == 0 {
            self.inner_html.clone()
        } else {
            let children_content = self.children.iter().fold(String::new(), |sum, next| {
                format!("{}{}", sum, next.convert())
            });
            format!("{}{}", self.inner_html.clone(), children_content)
        };

        let attributes = self
            .attribs
            .iter()
            .fold(String::new(), |sum, (key, value)| {
                format!("{} {} = \"{}\"", sum, key, value)
            });

        format!("<{}{}>{}{}", self.tag, attributes, content, end_tag)
    }

    // Setting value in element that was created already

    pub fn set_value(&mut self, value: &str) -> Result<(), &str> {
        if self.is_closed {
            self.inner_html = value.to_string();
            Ok(())
        } else {
            Err("Этот тег не является двойным")
        }
    }

    // Установка значения тега во время создания структуры
    pub fn add_value(mut self, value: &str) -> Self {
        if self.is_closed {
            self.inner_html += value;
        }
        self
    }

    // Получение значение элемента
    pub fn get_value(&self) -> &str {
        &self.inner_html
    }

    // Получить ссылку на дочерние элементы
    pub fn get_children(&self) -> &Vec<Self> {
        &self.children
    }

    // Добавить дочерний элемент во время создания объекта
    pub fn add(mut self, element: &GsElement) -> Self {
        if self.is_closed {
            self.children.push(element.clone());
        }
        self
    }

    // Добавить дочерний элемент после создания объекта
    pub fn push(&mut self, element: &GsElement) -> Result<(), &str> {
        if self.is_closed {
            self.children.push(element.clone());
            Ok(())
        } else {
            Err("This is unclosed tag")
        }
    }

    // Удалить элемент по индексу
    pub fn remove(&mut self, index: usize) -> Result<Self, &str> {
        if index >= self.children.len() {
            Err("Индекс выходит за рамки максимального количества элементов")
        } else {
            Ok(self.children.remove(index))
        }
    }

    // Добавить атрибут во время создания элемента
    pub fn add_attr(mut self, attribute: &'static str, value: &'static str) -> Self {
        self.attribs
            .entry(attribute)
            .and_modify(|val| *val = format!("{} {}", val, value))
            .or_insert(value.to_string());
        self
    }

    // Добавить атрибут после создания элемента
    pub fn push_attr(&mut self, attribute: &'static str, value: &'static str) {
        self.attribs
            .entry(attribute)
            .and_modify(|val| *val = format!("{} {}", val, value))
            .or_insert(value.to_string());
    }

    // Получить все атрибуты
    pub fn get_attr(&self) -> &HashMap<&'static str, String> {
        &self.attribs
    }
}

impl From<(&str, bool)> for GsElement {
    fn from((tag, is_closed): (&str, bool)) -> Self {
        GsElement::new(tag.into(), is_closed)
    }
}

impl From<(String, bool)> for GsElement {
    fn from((tag, is_closed): (String, bool)) -> Self {
        GsElement::new(&tag, is_closed)
    }
}
