use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

pub struct Footer<'a> {
    pub bindings: &'a [(&'a str, &'a str)],
}

impl<'a> Widget for Footer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let spans: Vec<Span> = self
            .bindings
            .iter()
            .flat_map(|(key, desc)| {
                [
                    Span::styled(
                        format!("[{key}]"),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(desc.to_string(), Style::default().fg(Color::DarkGray)),
                ]
            })
            .collect();

        Paragraph::new(Line::from(spans).centered()).render(inner, buf);
    }
}
