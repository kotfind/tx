use crate::Cli;

pub struct Printer {
    first_line_is_header: bool,
    data: Vec<Vec<String>>,
    column_count: Option<usize>,
    style: PrinterStyle,
}

impl Printer {
    pub fn new(style: PrinterStyle) -> Self {
        Self {
            data: Vec::new(),
            column_count: None,
            first_line_is_header: false,
            style,
        }
    }

    pub fn push_line(&mut self, line: Vec<String>) {
        self.push_line_header(line, false);
    }

    pub fn push_header(&mut self, line: Vec<String>) {
        self.push_line_header(line, true);
    }

    fn push_line_header(&mut self, line: Vec<String>, is_header: bool) {
        match self.column_count {
            Some(column_count) => {
                assert!(column_count == line.len());
            }
            None => {
                self.column_count = Some(line.len());
            }
        }

        if is_header {
            assert!(self.data.is_empty());

            if !self.style.print_header {
                return;
            }

            self.first_line_is_header = true;
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
        for line in self.data.iter() {
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
    print_header: bool,
    kind: PrinterStyleKind,
}

#[derive(PartialEq, Debug)]
pub enum PrinterStyleKind {
    Simple,
    Table,
}

impl PrinterStyle {
    pub fn from_cli(cli: &Cli) -> Self {
        Self {
            print_header: cli.print_header,
            kind: PrinterStyleKind::from_cli(cli),
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
