use crate::Cli;

pub struct Printer {
    data: Vec<Vec<String>>,
    column_count: Option<usize>,
    style: PrinterStyle,
    has_header: bool,
    print_header: bool,
}

impl Printer {
    pub fn from_cli_and_header_required(cli: &Cli, header_required: bool) -> Self {
        Self {
            data: Vec::new(),
            column_count: None,
            style: PrinterStyle::from_cli(cli),
            has_header: cli.has_header || cli.print_header || header_required,
            print_header: cli.print_header,
        }
    }

    pub fn push_line(&mut self, line: Vec<String>) {
        let mut is_header = false;

        match self.column_count {
            // First run
            None => {
                self.column_count = Some(line.len());
                is_header = self.has_header;
            }

            // Not first run
            Some(column_count) => {
                assert!(column_count == line.len());
            }
        }

        if is_header && !self.print_header {
            return;
        }

        match self.style {
            PrinterStyle::Simple => {
                for (col_id, item) in line.iter().enumerate() {
                    print!("{item}");
                    if col_id + 1 != self.column_count.unwrap() {
                        print!(" ");
                    }
                }
                println!("")
            }
            PrinterStyle::Table => {
                self.data.push(line);
            }
        }
    }

    pub fn finish(&self) {
        if self.style == PrinterStyle::Simple {
            // Already printed
            return;
        }
        assert!(self.style == PrinterStyle::Table);

        if self.data.is_empty() {
            return;
        }

        let mut column_widths = vec![0usize; self.column_count.unwrap()];
        let mut lines = self.data.iter();
        if self.has_header && !self.print_header {
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

#[derive(PartialEq, Debug)]
enum PrinterStyle {
    Simple,
    Table,
}

impl PrinterStyle {
    fn from_cli(cli: &Cli) -> Self {
        if cli.no_pretty {
            PrinterStyle::Simple
        } else {
            PrinterStyle::Table
        }
    }
}
