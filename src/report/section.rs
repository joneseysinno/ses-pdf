use crate::chart::Chart;
use crate::element::{
    CalcBlock, Equation, Figure, HeadingLevel, ReportElement, Table,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub level: HeadingLevel,
    pub elements: Vec<ReportElement>,
}

impl Section {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            level: HeadingLevel::H1,
            elements: Vec::new(),
        }
    }

    pub fn level(mut self, level: HeadingLevel) -> Self {
        self.level = level;
        self
    }

    pub fn add(mut self, element: impl Into<ReportElement>) -> Self {
        self.elements.push(element.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.elements.push(ReportElement::Paragraph(text.into()));
        self
    }

    pub fn equation(mut self, eq: Equation) -> Self {
        self.elements.push(ReportElement::Equation(eq));
        self
    }

    pub fn table(mut self, table: Table) -> Self {
        self.elements.push(ReportElement::Table(table));
        self
    }

    pub fn calc(mut self, block: CalcBlock) -> Self {
        self.elements.push(ReportElement::CalcBlock(block));
        self
    }

    pub fn chart(mut self, chart: Chart, caption: impl Into<String>) -> Self {
        self.elements.push(ReportElement::Chart {
            chart,
            caption: caption.into(),
        });
        self
    }

    pub fn figure(mut self, figure: Figure) -> Self {
        self.elements.push(ReportElement::Figure(figure));
        self
    }

    pub fn page_break(mut self) -> Self {
        self.elements.push(ReportElement::PageBreak);
        self
    }

    pub fn rule(mut self) -> Self {
        self.elements.push(ReportElement::HorizontalRule);
        self
    }
}
