#[cfg(test)]
#[cfg(feature = "option")]
mod test {
    use std::sync::Arc;

    use async_trait::async_trait;
    use dark_std::sync::SyncVec;
    use futures_core::future::BoxFuture;
    use rbatis::{
        executor::Executor,
        impl_delete_ext, impl_insert_ext, impl_select_ext, impl_update_ext,
        intercept::{Intercept, ResultType},
        table_own_ref, table_ref, Error, RBatis,
    };
    use rbatis_macro_driver::RefModel;
    use rbdc::{
        db::{ConnectOptions, Connection, Driver, ExecResult, MetaData, Row},
        rt::block_on,
        DateTime,
    };
    use rbs::Value;

    #[derive(Debug)]
    pub struct MockIntercept {
        pub sql_args: Arc<SyncVec<(String, Vec<Value>)>>,
    }

    impl MockIntercept {
        fn new(inner: Arc<SyncVec<(String, Vec<Value>)>>) -> Self {
            Self { sql_args: inner }
        }
    }

    #[async_trait]
    impl Intercept for MockIntercept {
        async fn before(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            sql: &mut String,
            args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
        ) -> Result<Option<bool>, Error> {
            self.sql_args.push((sql.to_string(), args.clone()));
            Ok(Some(true))
        }
        async fn after(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
        ) -> Result<Option<bool>, Error> {
            println!("_result:{:?}", _result);
            Ok(Some(true))
        }
    }

    #[derive(Debug, Clone)]
    struct MockDriver {}

    impl Driver for MockDriver {
        fn name(&self) -> &str {
            "test"
        }

        fn connect(
            &self,
            _url: &str,
        ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async {
                Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
            })
        }

        fn connect_opt<'a>(
            &'a self,
            _opt: &'a dyn ConnectOptions,
        ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async {
                Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
            })
        }

        fn default_option(&self) -> Box<dyn ConnectOptions> {
            Box::new(MockConnectOptions {})
        }
    }

    #[derive(Clone, Debug)]
    struct MockRowMetaData {
        sql: String,
    }

    impl MetaData for MockRowMetaData {
        fn column_len(&self) -> usize {
            if self.sql.contains("select count") {
                1
            } else {
                2
            }
        }

        fn column_name(&self, i: usize) -> String {
            if self.sql.contains("select count") {
                "count".to_string()
            } else {
                if i == 0 {
                    "sql".to_string()
                } else {
                    "count".to_string()
                }
            }
        }

        fn column_type(&self, _i: usize) -> String {
            "String".to_string()
        }
    }

    #[derive(Clone, Debug)]
    struct MockRow {
        pub sql: String,
        pub count: u64,
    }

    impl Row for MockRow {
        fn meta_data(&self) -> Box<dyn MetaData> {
            Box::new(MockRowMetaData {
                sql: self.sql.clone(),
            }) as Box<dyn MetaData>
        }

        fn get(&mut self, i: usize) -> Result<Value, Error> {
            if self.sql.contains("select count") {
                Ok(Value::U64(self.count))
            } else {
                if i == 0 {
                    Ok(Value::String(self.sql.clone()))
                } else {
                    Ok(Value::U64(self.count.clone()))
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    struct MockConnection {}

    impl Connection for MockConnection {
        fn get_rows(
            &mut self,
            sql: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
            let sql = sql.to_string();
            Box::pin(async move {
                let data = Box::new(MockRow { sql: sql, count: 1 }) as Box<dyn Row>;
                Ok(vec![data])
            })
        }

        fn exec(
            &mut self,
            _sql: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<Result<ExecResult, Error>> {
            Box::pin(async move {
                Ok(ExecResult {
                    rows_affected: 0,
                    last_insert_id: Value::Null,
                })
            })
        }

        fn close(&mut self) -> BoxFuture<Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }

        fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[derive(Clone, Debug)]
    struct MockConnectOptions {}

    impl ConnectOptions for MockConnectOptions {
        fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async {
                Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
            })
        }

        fn set_uri(&mut self, _uri: &str) -> Result<(), Error> {
            Ok(())
        }
    }

    /*
        #[test]
        fn test_ref_model() {
            /// This structure is consistent with the database table structure, add, delete, and check using the generated Ref structure
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, RefModel)]
            pub struct MockTable {
                /// Primary key id, can't not exist
                pub id: String,
                /// Remarks, can be empty
                pub name: Option<String>,
            }

            impl_insert_ext!(MockTable{});
            impl_update_ext!(MockTable{});


            // Generate shortcut macros for refs, reducing the need to write multiple None
            let new = table_ref!(MockTable {
                name: Cow::Owned(None)
            });
            let new2 = table_own_ref!(MockTable {
                name: None
            });

            // With this data insertion, you can specify that a lot inserts Null
            // For example, name
            let v2 = MockTableRef {
                id: None,
                name: Some(Cow::Owned(None)),
            };

            assert_eq!(new, v2);
            assert_eq!(new2, v2);
            println!("{:?}",v2);



        }
    */

    #[derive(
        Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, RefModel,
    )]
    struct MockTable2 {
        pub id: String,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: String,
        pub h5_banner_img: String,
        pub sort: Option<String>,
        pub status: i32,
        pub remark: Option<String>,
        pub create_time: DateTime,
        pub version: i64,
        pub delete_flag: Option<i32>,
        //exec sql
        pub count: u64, //page count num
    }

    #[test]
    fn test_crud_ref_insert() {
        use std::borrow::Cow;
        let f = async move {
            impl_insert_ext!(MockTable2 {});

            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let _r = MockTable2::insert_batch_ref(
                &mut rb,
                &[
                    table_own_ref!(MockTable2 {
                        id: "1".into(),
                        pc_banner_img: "2".into(),
                        h5_banner_img: "3".into(),
                        status: 1,
                        create_time: DateTime::now(),
                        version: 2,
                        count: 1,
                    }),
                    table_ref!(MockTable2 {
                        id: Cow::Owned("2".into()),
                        pc_link: Cow::Owned(Some("2".into())),
                        h5_link: Cow::Owned(None),
                        pc_banner_img: Cow::Owned("2".into()),
                        h5_banner_img: Cow::Owned("2".into()),
                        status: Cow::Owned(1),
                        create_time: Cow::Owned(DateTime::now()),
                        version: Cow::Owned(1),
                        count: Cow::Owned(1),
                    }),
                ],
                2,
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            println!("{:?}", args);
            assert_eq!(
                sql,
                "insert into mock_table2 (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag,count) VALUES ( ?, DEFAULT, DEFAULT, DEFAULT,?,?, DEFAULT,?, DEFAULT,?,?, DEFAULT,?)?, DEFAULT,?,?,?,?, DEFAULT,?, DEFAULT,?,?, DEFAULT,?)"
            );
            // let (sql, args) = queue.pop().unwrap();
            // println!("{}", sql);
            // assert_eq!(
            //     sql,
            //     "select  count(1) as count from activity where delete_flag = 0 and var1 = ? and name=?"
            // );
        };
        block_on(f);
    }

    #[test]
    fn test_crud_ref_update() {
        use std::borrow::Cow;
        let f = async move {
            impl_update_ext!(MockTable2 {});

            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let _r = MockTable2::update_by_ref(
                &mut rb,
                &table_own_ref!(MockTable2 {
                    id: "1".into(),
                    pc_banner_img: "2".into(),
                    h5_banner_img: "3".into(),
                    status: 1,
                    create_time: DateTime::now(),
                    version: 2,
                    count: 1,
                }),
                &table_ref!(MockTable2 {
                    id: Cow::Owned("2".into()),
                    pc_link: Cow::Owned(Some("2".into())),
                    h5_link: Cow::Owned(None),
                    pc_banner_img: Cow::Owned("2".into()),
                    h5_banner_img: Cow::Owned("2".into()),
                    status: Cow::Owned(1),
                    create_time: Cow::Owned(DateTime::now()),
                    version: Cow::Owned(1),
                    count: Cow::Owned(1),
                }),
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            println!("{:?}", args);
            assert_eq!(
                sql,
                "update mock_table2 set id=?,pc_banner_img=?,h5_banner_img=?,status=?,create_time=?,version=?,count=?  where  id = ? and pc_link = ? and h5_link = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? "
            );

            let _r = MockTable2::update_by_refs(
                &mut rb,
                &table_own_ref!(MockTable2 {
                    id: "1".into(),
                    pc_banner_img: "2".into(),
                    h5_banner_img: "3".into(),
                    status: 1,
                    create_time: DateTime::now(),
                    version: 2,
                    count: 1,
                }),
                &[
                    table_own_ref!(MockTable2 {
                        id: "1".into(),
                        pc_banner_img: "2".into(),
                        h5_banner_img: "3".into(),
                        status: 1,
                        create_time: DateTime::now(),
                        version: 2,
                        count: 1,
                    }),
                    table_ref!(MockTable2 {
                        id: Cow::Owned("2".into()),
                        pc_link: Cow::Owned(Some("2".into())),
                        h5_link: Cow::Owned(None),
                        pc_banner_img: Cow::Owned("2".into()),
                        h5_banner_img: Cow::Owned("2".into()),
                        status: Cow::Owned(1),
                        create_time: Cow::Owned(DateTime::now()),
                        version: Cow::Owned(1),
                        count: Cow::Owned(1),
                    }),
                ],
            )
            .await
            .unwrap();

            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            println!("{:?}", args);
            assert_eq!(
                sql,
                "update mock_table2 set id=?,pc_banner_img=?,h5_banner_img=?,status=?,create_time=?,version=?,count=?  where  (  id = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? ) or (  id = ? and pc_link = ? and h5_link = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? ) "
            );
        };
        block_on(f);
    }

    #[test]
    fn test_crud_ref_delete() {
        use std::borrow::Cow;
        let f = async move {
            impl_delete_ext!(MockTable2 {});

            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let _r = MockTable2::delete_by_ref(
                &mut rb,
                &table_own_ref!(MockTable2 {
                    id: "1".into(),
                    pc_banner_img: "2".into(),
                    h5_banner_img: "3".into(),
                    status: 1,
                    create_time: DateTime::now(),
                    version: 2,
                    count: 1,
                }),
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            println!("{:?}", args);
            assert_eq!(
                sql,
                "delete from mock_table2  where  id = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? "
            );

            let _r = MockTable2::delete_by_refs(
                &mut rb,
                &[
                    table_own_ref!(MockTable2 {
                        id: "1".into(),
                        pc_banner_img: "2".into(),
                        h5_banner_img: "3".into(),
                        status: 1,
                        create_time: DateTime::now(),
                        version: 2,
                        count: 1,
                    }),
                    table_ref!(MockTable2 {
                        id: Cow::Owned("2".into()),
                        pc_link: Cow::Owned(Some("2".into())),
                        h5_link: Cow::Owned(None),
                        pc_banner_img: Cow::Owned("2".into()),
                        h5_banner_img: Cow::Owned("2".into()),
                        status: Cow::Owned(1),
                        create_time: Cow::Owned(DateTime::now()),
                        version: Cow::Owned(1),
                        count: Cow::Owned(1),
                    }),
                    MockTable2Ref {
                        id: Some(Cow::Owned("2".into())),
                        name: Some(Cow::Owned(Some("2".into()))),
                        pc_link: Some(Cow::Owned(Some("2".into()))),
                        h5_link: Some(Cow::Owned(Some("2".into()))),
                        pc_banner_img: Some(Cow::Owned("2".into())),
                        h5_banner_img: Some(Cow::Owned("2".into())),
                        sort: Some(Cow::Owned(Some("2".into()))),
                        status: Some(Cow::Owned(43)),
                        remark: None,
                        create_time: Some(Cow::Owned(DateTime::now())),
                        version: Some(Cow::Owned(43)),
                        delete_flag: None,
                        count: Some(Cow::Owned(43)), //page count num
                    },
                ],
            )
            .await
            .unwrap();

            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            println!("{:?}", args);
            assert_eq!(
                sql,
                "delete from mock_table2  where  (  id = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? ) or (  id = ? and pc_link = ? and h5_link = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? ) or (  id = ? and name = ? and pc_link = ? and h5_link = ? and pc_banner_img = ? and h5_banner_img = ? and sort = ? and status = ? and create_time = ? and version = ? and count = ? ) "
            );
        };
        block_on(f);
    }

    #[test]
    fn test_crud_ref_select() {
        use std::borrow::Cow;
        let f = async move {
            impl_select_ext!(MockTable2 {});

            // impl_select_ext!(MockTable2 {select_list_by_refs<'__ref>(where_datas: &[[<$table Ref>]<'__ref>]) -> Vec =>
            //  "where :
            //    trim 'or': for _,item in where_datas:
            //      `or ( `
            //      trim 'and': for k,v in item:
            //        if v.is_null():
            //          continue:
            //        `and ${k} = #{v} `
            //      `) `"});

            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable2::select_list_by_ref(
                &mut rb,
                &table_own_ref!(MockTable2 {
                    id: "1".into(),
                    pc_banner_img: "2".into(),
                    h5_banner_img: "3".into(),
                    status: 1,
                    create_time: DateTime::now(),
                    version: 2,
                    count: 1,
                }),
            )
            .await;

            // Expected error, unlikely if the structure is configured consistently with the nullable configuration of the table structure
            assert_eq!(r.is_err(), true);
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            println!("{:?}", args);
            assert_eq!(
                sql,
                "select * from mock_table2  where  id = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? "
            );

            let r = MockTable2::select_list_by_refs(
                &mut rb,
                &[
                    table_own_ref!(MockTable2 {
                        id: "1".into(),
                        pc_banner_img: "2".into(),
                        h5_banner_img: "3".into(),
                        status: 1,
                        create_time: DateTime::now(),
                        version: 2,
                        count: 1,
                    }),
                    table_ref!(MockTable2 {
                        id: Cow::Owned("2".into()),
                        pc_link: Cow::Owned(Some("2".into())),
                        h5_link: Cow::Owned(None),
                        pc_banner_img: Cow::Owned("2".into()),
                        h5_banner_img: Cow::Owned("2".into()),
                        status: Cow::Owned(1),
                        create_time: Cow::Owned(DateTime::now()),
                        version: Cow::Owned(1),
                        count: Cow::Owned(1),
                    }),
                    MockTable2Ref {
                        id: Some(Cow::Owned("2".into())),
                        name: Some(Cow::Owned(Some("2".into()))),
                        pc_link: Some(Cow::Owned(Some("2".into()))),
                        h5_link: Some(Cow::Owned(Some("2".into()))),
                        pc_banner_img: Some(Cow::Owned("2".into())),
                        h5_banner_img: Some(Cow::Owned("2".into())),
                        sort: Some(Cow::Owned(Some("2".into()))),
                        status: Some(Cow::Owned(43)),
                        remark: None,
                        create_time: Some(Cow::Owned(DateTime::now())),
                        version: Some(Cow::Owned(43)),
                        delete_flag: None,
                        count: Some(Cow::Owned(43)), //page count num
                    },
                ],
            )
            .await;

            // Expected error, unlikely if the structure is configured consistently with the nullable configuration of the table structure
            assert_eq!(r.is_err(), true);

            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            println!("{:?}", args);
            assert_eq!(
                sql,
                "select * from mock_table2  where  (  id = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? ) or (  id = ? and pc_link = ? and h5_link = ? and pc_banner_img = ? and h5_banner_img = ? and status = ? and create_time = ? and version = ? and count = ? ) or (  id = ? and name = ? and pc_link = ? and h5_link = ? and pc_banner_img = ? and h5_banner_img = ? and sort = ? and status = ? and create_time = ? and version = ? and count = ? ) "
            );
        };
        block_on(f);
    }
}
