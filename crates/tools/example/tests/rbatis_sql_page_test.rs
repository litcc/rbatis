#[cfg(test)]
mod tests {
    use rbatis::plugin::IPageRequest;
    use rbatis::plugin::PageRequest;

    fn make_pages() -> Vec<Vec<i32>> {
        vec![vec![1, 10, 100, 1], vec![2, 20, 200, 0], vec![3, 30, 300, 1]]
    }
    #[test]
    fn test_page_request() {
        let data = make_pages();
        let page_req = PageRequest::default();
        for ref i in data {
            let (page_no, page_size, total, do_count) =
                (i[0] as u64, i[1] as u64, i[2] as u64, i[3] != 0);
            let mut pr = PageRequest::new_total(page_no, page_size, total);
            pr = pr.set_do_count(do_count);
            assert_eq!(pr.page_size(), page_size);
            assert_eq!(pr.page_no(), if page_no < 1 { 1 } else { page_no });
            assert_eq!(pr.total(), total);
            assert_eq!(pr.do_count(), do_count);

            let mut pr = page_req
                .clone()
                .set_page_no(page_no)
                .set_page_size(page_size)
                .set_total(total)
                .set_do_count(do_count);
            pr = pr.set_do_count(do_count);
            assert_eq!(pr.page_size(), page_size);
            assert_eq!(pr.page_no(), if page_no < 1 { 1 } else { page_no });
            assert_eq!(pr.total(), total);
            assert_eq!(pr.do_count(), do_count);
        }
    }
}
