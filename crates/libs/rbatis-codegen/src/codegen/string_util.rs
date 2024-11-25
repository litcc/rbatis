use std::collections::LinkedList;

//find like #{*,*},${*,*} value *
pub fn find_convert_string(arg: &str) -> LinkedList<(String, String)> {
    let mut list = LinkedList::new();
    let chars: Vec<u8> = arg.bytes().collect();
    let mut item = String::with_capacity(arg.len());
    let mut index: i32 = -1;
    for v in &chars {
        index += 1;
        if !item.is_empty() {
            item.push(*v as char);
            if *v == b'}' {
                let key = item[2..item.len() - 1].to_string();
                list.push_back((key, item.clone()));
                item.clear();
            }
            continue;
        }
        if (*v == b'#' || *v == b'$') &&
            chars.get(index as usize + 1).eq(&Some(&b'{'))
        {
            item.push(*v as char);
        }
    }
    list
}

pub fn count_string_num(s: &str, c: char) -> usize {
    let cs = s.chars();
    cs.filter(|&x| x == c).count()
}

///input 'strings' => strings
pub fn un_packing_string(column: &str) -> &str {
    if column.len() >= 2 {
        if column.starts_with("'") && column.ends_with("'") {
            return &column[1..column.len() - 1];
        }
        if column.starts_with("`") && column.ends_with("`") {
            return &column[1..column.len() - 1];
        }
        if column.starts_with("\"") && column.ends_with("\"") {
            return &column[1..column.len() - 1];
        }
    }
    column
}
