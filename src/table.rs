use crate::Cli;

pub struct Table(Vec<Vec<String>>);

impl Table {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_line(&mut self, line: Vec<String>) {
        assert!(self.0.is_empty() || self.0[0].len() == line.len());
        self.0.push(line);
    }

    pub fn column_count(&self) -> usize {
        if self.0.len() == 0 {
            0usize
        } else {
            self.0[0].len()
        }
    }

    pub fn print(&self, fmt: &TableStyle) {
        match fmt {
            TableStyle::Simple => self.print_simple(),
            TableStyle::Rich => self.print_rich(),
        }
    }

    // TODO: print line by line on push_line
    fn print_simple(&self) {
        for line in self.0.iter() {
            for (col_id, item) in line.iter().enumerate() {
                print!("{item}");
                if col_id + 1 != self.column_count() {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    fn print_rich(&self) {
        let mut column_widths = vec![0usize; self.column_count()];
        for line in self.0.iter() {
            for (col_id, item) in line.iter().enumerate() {
                column_widths[col_id] = column_widths[col_id].max(item.len())
            }
        }

        for line in self.0.iter() {
            for (col_id, item) in line.iter().enumerate() {
                print!("{item:<width$}", width = column_widths[col_id]);
                if col_id + 1 != self.column_count() {
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
        if cli.pretty {
            TableStyle::Rich
        } else {
            TableStyle::Simple
        }
    }
}
