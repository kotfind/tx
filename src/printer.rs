use crate::Cli;

pub struct Printer {
    data: Vec<Vec<String>>,
    column_count: Option<usize>,
    style: PrinterStyle,
}

impl Printer {
    pub fn new(style: PrinterStyle) -> Self {
        Self {
            data: Vec::new(),
            column_count: None,
            style,
        }
    }

    pub fn push_line(&mut self, line: Vec<String>) {
        let mut is_header = false;

        match self.column_count {
            // First run
            None => {
                self.column_count = Some(line.len());
                is_header = self.style.has_header;
            }

            // Not first run
            Some(column_count) => {
                assert!(column_count == line.len());
            }
        }

        if is_header && !self.style.print_header {
            return;
        }

        match self.style.kind {
            PrinterStyleKind::Simple => {
                for (col_id, item) in line.iter().enumerate() {
                    print!("{item}");
                    if col_id + 1 != self.column_count.unwrap() {
                        print!(" ");
                    }
                }
                println!("")
            }
            PrinterStyleKind::Table => {
                self.data.push(line);
            }
        }
    }

    pub fn finish(&self) {
        if self.style.kind == PrinterStyleKind::Simple {
            // Already printed
            return;
        }
        assert!(self.style.kind == PrinterStyleKind::Table);

        if self.data.is_empty() {
            return;
        }

        let mut column_widths = vec![0usize; self.column_count.unwrap()];
        let mut lines = self.data.iter();
        if self.style.has_header && !self.style.print_header {
            lines.next();
        }
        for line in lines {
            for (col_id, item) in line.iter().enumerate() {
                column_widths[col_id] = column_widths[col_id].max(item.len())
            }
        }

        for line in self.data.iter() {
            for (col_id, item) in line.iter().enumerate() {
                print!("{item:<width$}", width = column_widths[col_id]);
                if col_id + 1 != self.column_count.unwrap() {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

#[derive(Debug)]
pub struct PrinterStyle {
    kind: PrinterStyleKind,
    has_header: bool,
    print_header: bool,
}

#[derive(PartialEq, Debug)]
pub enum PrinterStyleKind {
    Simple,
    Table,
}

impl PrinterStyle {
    pub fn from_cli_and_header_required(cli: &Cli, header_required: bool) -> Self {
        Self {
            kind: PrinterStyleKind::from_cli(cli),
            has_header: cli.has_header || cli.print_header || header_required,
            print_header: cli.print_header,
        }
    }
}

impl PrinterStyleKind {
    pub fn from_cli(cli: &Cli) -> Self {
        if cli.no_pretty {
            PrinterStyleKind::Simple
        } else {
            PrinterStyleKind::Table
        }
    }
}
