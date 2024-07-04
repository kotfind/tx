use crate::Cli;

pub struct Table {
    data: Vec<Vec<String>>,
    column_count: Option<usize>,
    style: TableStyle,
}

impl Table {
    pub fn new(style: TableStyle) -> Self {
        Self {
            data: Vec::new(),
            column_count: None,
            style,
        }
    }

    pub fn push_line(&mut self, line: Vec<String>) {
        match self.column_count {
            Some(column_count) => {
                assert!(column_count == line.len());
            }
            None => {
                self.column_count = Some(line.len());
            }
        }

        match self.style {
            TableStyle::Simple => {
                for (col_id, item) in line.iter().enumerate() {
                    print!("{item}");
                    if col_id + 1 != self.column_count.unwrap() {
                        print!(" ");
                    }
                }
                println!("")
            }
            TableStyle::Rich => {
                self.data.push(line);
            }
        }
    }

    pub fn finish(&self) {
        if self.style == TableStyle::Simple {
            // Already printed
            return;
        }
        assert!(self.style == TableStyle::Rich);

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

#[derive(PartialEq)]
pub enum TableStyle {
    Simple,
    Rich,
}

impl TableStyle {
    pub fn from_cli(cli: &Cli) -> Self {
        if cli.no_pretty {
            TableStyle::Simple
        } else {
            TableStyle::Rich
        }
    }
}
