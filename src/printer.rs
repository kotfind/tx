use crate::Cli;

pub struct Printer {
    data: Vec<Vec<String>>,
    column_count: Option<usize>,
    style: PrinterStyle,
    has_header: bool,
    print_header: bool,
}

impl Printer {
    pub fn new(cli: &Cli, has_header: bool, print_header: bool) -> Self {
        Self {
            data: Vec::new(),
            column_count: None,
            style: PrinterStyle::from_cli(cli),
            has_header,
            print_header,
        }
    }

    pub fn push_header(&mut self, row: Vec<String>) {
        self.push_row_header(row, true);
    }

    pub fn push_row(&mut self, row: Vec<String>) {
        self.push_row_header(row, false);
    }

    fn push_row_header(&mut self, row: Vec<String>, is_header: bool) {
        match self.column_count {
            // First run
            None => {
                self.column_count = Some(row.len());
                assert!(is_header == self.has_header);
            }

            // Not first run
            Some(column_count) => {
                assert!(column_count == row.len());
            }
        }

        if is_header && !self.print_header {
            return;
        }

        match self.style {
            PrinterStyle::Simple => {
                for (col_id, item) in row.iter().enumerate() {
                    print!("{item}");
                    if col_id + 1 != self.column_count.unwrap() {
                        print!(" ");
                    }
                }
                println!("")
            }
            PrinterStyle::Table => {
                self.data.push(row);
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
        let mut rows = self.data.iter();
        if self.has_header && !self.print_header {
            rows.next();
        }
        for row in rows {
            for (col_id, item) in row.iter().enumerate() {
                column_widths[col_id] = column_widths[col_id].max(item.len())
            }
        }

        for row in self.data.iter() {
            for (col_id, item) in row.iter().enumerate() {
                if col_id + 1 != self.column_count.unwrap() {
                    print!("{item:<width$}", width = column_widths[col_id]);
                    print!(" ");
                } else {
                    print!("{item}");
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
