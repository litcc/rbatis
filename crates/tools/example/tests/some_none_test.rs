#[cfg(test)]
mod test {
    use rbs::Value;
    use serde::Deserialize;
    use serde::Serialize;

    #[test]
    pub fn test_some_none() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct A {
            a: Option<Option<i32>>,
            b: Option<Option<i32>>,
            c: Option<Option<i32>>,
        }

        let dd = A { a: Some(Some(1)), b: Some(None), c: None };

        let dd = rbs::to_value(&dd).unwrap();

        let dff = dd.as_map().unwrap();

        let bb = dff.0.get(&Value::String("b".into())).unwrap();
        assert!(bb.is_some_null());
    }
}
