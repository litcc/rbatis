/// impl exchange
pub fn impl_exchange(start_str: &str, start_num: usize, sql: &str) -> String {
    let mut last = b' ';
    let mut sql = sql.to_string();
    let mut sql_bytes = sql.as_bytes();
    let mut placeholder_idx = start_num;
    let mut index = 0;
    loop {
        if index == sql_bytes.len() {
            break;
        }
        let x = sql_bytes[index];
        if x == b'?' && last != b'\\' {
            sql.remove(index);
            for (i, x) in start_str.chars().enumerate() {
                sql.insert(index + i, x);
            }
            sql.insert_str(
                index + start_str.len(),
                itoa::Buffer::new().format(placeholder_idx),
            );
            placeholder_idx += 1;
            sql_bytes = sql.as_bytes();
        } else {
            last = x;
        }
        index += 1;
    }
    sql
}
