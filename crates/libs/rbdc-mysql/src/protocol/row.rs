use std::ops::Range;

#[derive(Debug)]
pub struct Row {
    pub storage: Vec<Option<Vec<u8>>>,
    pub values: Vec<Option<Range<usize>>>,
}

impl From<(Vec<Option<Range<usize>>>, Vec<u8>)> for Row {
    fn from((ranges, data): (Vec<Option<Range<usize>>>, Vec<u8>)) -> Self {
        let mut row = Row {
            storage: Vec::with_capacity(ranges.len()),
            values: Vec::with_capacity(ranges.len()),
        };
        for x in ranges {
            if let Some(col) = x {
                row.storage.push(Some(data[col.start..col.end].to_vec()));
                row.values.push(Some(col));
            } else {
                row.storage.push(None);
                row.values.push(None);
            }
        }
        row
    }
}

impl Row {
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        self.values.iter().enumerate().find_map(|(idx, x)| {
            if index == idx {
                match x {
                    None => None,
                    Some(_) => self.storage[idx].as_deref(),
                }
            } else {
                None
            }
        })
    }

    pub fn take(&mut self, index: usize) -> Option<Vec<u8>> {
        self.values.iter().enumerate().find_map(|(idx, x)| {
            if index == idx {
                match x {
                    None => None,
                    Some(_) => self.storage[idx].take(),
                }
            } else {
                None
            }
        })
    }
}
