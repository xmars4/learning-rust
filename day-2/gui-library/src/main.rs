pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
}

impl Button {
    fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", &self.label).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.label.chars().count() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        writeln!(buffer, "+{:-^width$}+", "").unwrap();
        writeln!(buffer, "|{: ^width$}|", &self.label.label).unwrap();
        writeln!(buffer, "+{:-^width$}+", "").unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.inner_width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let inner_width = self.inner_width() + 2;
        let inner_line_width = inner_width - 2;
        let mut inner_buffer = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner_buffer);
        }

        writeln!(buffer, "+{:=^inner_width$}+", "").unwrap_or_default();
        writeln!(buffer, "|{: ^inner_width$}|", &self.title).unwrap();
        writeln!(buffer, "+{:=^inner_width$}+", "").unwrap_or_default();

        for line in inner_buffer.lines() {
            writeln!(buffer, "| {: <inner_line_width$} |", line).unwrap();
        }
        buffer
            .write_str(format!("+{:-^inner_width$}+\n", "").as_str())
            .unwrap_or_default();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
