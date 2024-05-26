#[macro_export]
macro_rules! crud_ext {
    ($table:ty{}) => {
        $crate::impl_insert_ext!($table {});
        $crate::impl_select_ext!($table {});
        $crate::impl_update_ext!($table {});
        $crate::impl_delete_ext!($table {});
    };
    ($table:ty{},$table_name:expr) => {
        $crate::impl_insert_ext!($table {}, $table_name);
        $crate::impl_select_ext!($table {}, $table_name);
        $crate::impl_update_ext!($table {}, $table_name);
        $crate::impl_delete_ext!($table {}, $table_name);
    };
}

#[macro_export]
macro_rules! impl_insert_ext {
    ($table:ty{}) => {
        $crate::impl_insert_ext!(
            $table {},
            ""
        );
    };
    ($table:ty{},$table_name:expr) => {
        $crate::paste::paste!{
            impl $table {
                pub async fn insert_batch_ref<'__ref>(
                    executor: &dyn $crate::executor::Executor,
                    tables: &[[<$table Ref>]<'__ref>],
                    batch_size: u64,
                ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    #[$crate::py_sql(
                        "`insert into ${table_name} `
                        trim ',':
                          for idx,table in tables:
                            if idx == 0:
                              `(`
                              trim ',':
                                for k,v in table:
                                  ${k},
                              `) VALUES `
                            (
                            trim ',':
                            for k,v in table:
                              if v.is_null():
                                ` DEFAULT,`
                                continue:
                              #{v},
                            ),
                        "
                    )]
                    async fn inner_insert_batch_ref<'__ref>(
                        executor: &dyn $crate::executor::Executor,
                        tables: &[[<$table Ref>]<'__ref>],
                        table_name: &str,
                    ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error>
                    {
                        impled!()
                    }
                    #[$crate::snake_name($table)]
                    fn snake_name(){}
                    let mut table_name = $table_name.to_string();
                    if table_name.is_empty(){
                        table_name = snake_name();
                    }
                    let mut result = $crate::rbdc::db::ExecResult {
                        rows_affected: 0,
                        last_insert_id: rbs::Value::Null,
                    };
                    let ranges = $crate::plugin::Page::<()>::make_ranges(tables.len() as u64, batch_size);
                    for (offset, limit) in ranges {
                        let exec_result = inner_insert_batch_ref(
                            executor,
                            &tables[offset as usize..limit as usize],
                            table_name.as_str(),
                        )
                        .await?;
                        result.rows_affected += exec_result.rows_affected;
                        result.last_insert_id = exec_result.last_insert_id;
                    }
                    Ok(result)
                }

                pub async fn insert_ref<'__ref>(
                    executor: &dyn $crate::executor::Executor,
                    table: &[<$table Ref>]<'__ref>,
                ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    <$table>::insert_batch_ref(executor, &[table.clone()], 1).await
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_select_ext {
    ($table:ty{}) => {
        $crate::impl_select_ext!($table{},"");
    };
    ($table:ty{} $(,$table_name:expr)?) => {
        $crate::paste::paste!{
            $crate::impl_select_ext!($table{select_list_by_ref<'__ref>(where_data: &[<$table Ref>]<'__ref>) -> Vec => "` where `
              trim 'and': for k,v in where_data:
                if v.is_null():
                    continue:
                `and ${k} = #{v} `"}$(,$table_name)?);
            $crate::impl_select_ext!($table{select_list_by_refs<'__ref>(where_datas: &[[<$table Ref>]<'__ref>]) -> Vec =>
             "` where `
               trim 'or': for _,item in where_datas:
                 `or ( `
                 trim 'and': for k,v in item:
                   if v.is_null():
                     continue:
                   `and ${k} = #{v} `
                 `) `"}$(,$table_name)?);
            $crate::impl_select_ext!($table{select_opt_by_ref<'__ref>(where_data: &[<$table Ref>]<'__ref>) -> Option =>
             "` where `
               trim 'and': for k,v in where_data:
                 if v.is_null():
                     continue:
                 `and ${k} = #{v} `"}$(,$table_name)?);
        }
    };

    ($table:ty{$fn_name:ident $(<$($life_cycle:lifetime $(,)?)*$($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty $(,)?)*) -> $container:tt => $sql:expr}$(,$table_name:expr)?) => {
        impl $table{
            pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor: &dyn  $crate::executor::Executor,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::rbdc::Error>
            {
                     #[$crate::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                     async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor: &dyn $crate::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::rbdc::Error> {impled!()}
                     let mut table_column = "*".to_string();
                     let mut table_name = String::new();
                     $(table_name = $table_name.to_string();)?
                     #[$crate::snake_name($table)]
                     fn snake_name(){}
                     if table_name.is_empty(){
                         table_name = snake_name();
                     }
                     $fn_name(executor,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };

}

#[macro_export]
macro_rules! impl_update_ext {
    ($table:ty{}) => {
        $crate::impl_update_ext!(
            $table{},
            ""
        );
    };

    ($table:ty{}$(,$table_name:expr)?) => {
        $crate::paste::paste!{
            $crate::impl_update_ext!($table{update_by_ref<'__ref>(where_data: &[<$table Ref>]<'__ref>) => "` where `
                trim 'and': for k,v in where_data:
                  if v.is_null():
                    continue:
                  `and ${k} = #{v} `"}$(,$table_name)?);
            $crate::impl_update_ext!($table{update_by_refs<'__ref>(where_datas: &[[<$table Ref>]<'__ref>]) => "` where `
                trim 'or': for _,item in where_datas:
                  `or ( `
                  trim 'and': for k,v in item:
                    if v.is_null():
                      continue:
                    `and ${k} = #{v} `
                  `) `"}$(,$table_name)?);
        }
    };

    ($table:ty{$fn_name:ident $(< $($life_cycle:lifetime $(,)?)* $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        $crate::paste::paste!{
            impl $table {
                pub async fn $fn_name $(<'__ref1, $($life_cycle,)* $($gkey:$gtype,)*>)? (
                    executor: &dyn $crate::executor::Executor,
                    update_data: &[<$table Ref>]<'__ref1>,
                    $($param_key:$param_type,)*
                ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    if $sql_where.is_empty(){
                        return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                    }
                    #[$crate::py_sql("`update ${table_name} set `
                                     trim ',':
                                       for k,v in table:
                                         if v.is_null():
                                            continue:
                                         `${k}=#{v},`
                                     ` `",$sql_where)]
                      async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)?(
                          executor: &dyn $crate::executor::Executor,
                          table_name: String,
                          table: &rbs::Value,
                          $($param_key:$param_type,)*
                      ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                          impled!()
                      }
                     let mut table_name = String::new();
                     $(table_name = $table_name.to_string();)?
                     #[$crate::snake_name($table)]
                     fn snake_name(){}
                     if table_name.is_empty(){
                         table_name = snake_name();
                     }
                      let table = rbs::to_value!(update_data);
                      $fn_name(executor, table_name, &table, $($param_key,)*).await
                }
            }
        }
    };

}

#[macro_export]
macro_rules! impl_delete_ext {
    ($table:ty{}) => {
        $crate::impl_delete_ext!(
            $table{},
            ""
        );
    };

    ($table:ty{}$(,$table_name:expr)?) => {
        $crate::paste::paste!{
            $crate::impl_delete_ext!($table {delete_by_ref<'__ref>(where_data: &[<$table Ref>]<'__ref>) =>
                "` where `
                    trim 'and': for k,v in where_data:
                        if v.is_null():
                            continue:
                        `and ${k} = #{v} `"}$(,$table_name)?);
            $crate::impl_delete_ext!($table {delete_by_refs<'__ref>(where_datas: &[[<$table Ref>]<'__ref>]) =>
                "` where `
                    trim 'or': for _,item in where_datas:
                        `or ( `
                        trim 'and': for k,v in item:
                            if v.is_null():
                                continue:
                            `and ${k} = #{v} `
                        `) `"}$(,$table_name)?);
        }
    };


    ($table:ty{$fn_name:ident $(< $($life_cycle:lifetime $(,)?)* $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)?(
                executor: &dyn $crate::executor::Executor,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql("`delete from ${table_name} `",$sql_where)]
                async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)?(
                    executor: &dyn $crate::executor::Executor,
                    table_name: String,
                    $($param_key:$param_type,)*
                ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                let mut table_name = String::new();
                $(table_name = $table_name.to_string();)?
                #[$crate::snake_name($table)]
                fn snake_name(){}
                if table_name.is_empty(){
                         table_name = snake_name();
                }
                $fn_name(executor, table_name, $($param_key,)*).await
            }
        }
    };

}

#[macro_export]
macro_rules! impl_select_page_ext {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}) => {
        $crate::impl_select_page_ext!(
            $table{$fn_name($($param_key:$param_type,)*)=> $where_sql},
            ""
        );
    };
    ($table:ty{$fn_name:ident $(<$($life_cycle:lifetime $(,)?)*$($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (
                executor: &dyn $crate::executor::Executor,
                page_request: &dyn $crate::plugin::IPageRequest,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::plugin::Page::<$table>, $crate::rbdc::Error> {
                let mut table_column = "*".to_string();
                let mut table_name = String::new();
                $(table_name = $table_name.to_string();)?
                #[$crate::snake_name($table)]
                fn snake_name(){}
                if table_name.is_empty(){
                    table_name = snake_name();
                }
                //pg,mssql can override this parameter to implement its own limit statement
                let mut limit_sql = " limit ${page_no},${page_size}".to_string();
                limit_sql=limit_sql.replace("${page_no}", &page_request.offset().to_string());
                limit_sql=limit_sql.replace("${page_size}", &page_request.page_size().to_string());
                let records:Vec<$table>;
                struct Inner{}
                impl Inner{
                 #[$crate::py_sql(
                    "`select `
                    if do_count == false:
                      `${table_column}`
                    if do_count == true:
                       `count(1) as count`
                    ` from ${table_name} `\n",$where_sql,"\n
                    if do_count == false:
                        `${limit_sql}`")]
                   async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor: &dyn $crate::executor::Executor,
                                     do_count:bool,
                                     table_column:&str,
                                     table_name: &str,
                                     page_no:u64,
                                     page_size:u64,
                                     page_offset:u64,
                                     limit_sql:&str,
                 $($param_key:&$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error> {impled!()}
                }
                let mut total = 0;
                if page_request.do_count() {
                    let total_value = Inner::$fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor,
                                                      true,
                                                      &table_column,
                                                      &table_name,
                                                      page_request.page_no(),
                                                      page_request.page_size(),
                                                      page_request.offset(),
                                                      "",
                                                      $(&$param_key,)*).await?;
                    total = $crate::decode(total_value).unwrap_or(0);
                }
                let mut page = $crate::plugin::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
                let records_value = Inner::$fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)?(executor,
                                                    false,
                                                    &table_column,
                                                    &table_name,
                                                    page_request.page_no(),
                                                    page_request.page_size(),
                                                    page_request.offset(),
                                                    &limit_sql,
                                                    $(&$param_key,)*).await?;
                page.records = rbs::from_value(records_value)?;
                Ok(page)
            }
        }
    };
}

#[macro_export]
macro_rules! htmlsql_select_page_ext {
    ($fn_name:ident $(<$($life_cycle:lifetime $(,)?)*$($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $html_file:expr) => {
            pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor: &dyn $crate::executor::Executor, page_request: &dyn $crate::plugin::IPageRequest, $($param_key:$param_type,)*) -> std::result::Result<$crate::plugin::Page<$table>, $crate::rbdc::Error> {
            struct Inner{}
            impl Inner{
              #[$crate::html_sql($html_file)]
              pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor: &dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key: &$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error>{
                 $crate::impled!()
              }
            }
            let mut total = 0;
            if page_request.do_count() {
               let total_value = Inner::$fn_name(executor, true, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
               total = $crate::decode(total_value).unwrap_or(0);
            }
            let mut page = $crate::plugin::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
            let records_value = Inner::$fn_name(executor, false, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
            page.records = rbs::from_value(records_value)?;
            Ok(page)
         }
    }
}

#[macro_export]
macro_rules! pysql_select_page_ext {
    ($fn_name:ident $(<$($life_cycle:lifetime $(,)?)*$($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $py_file:expr) => {
            pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor: &dyn $crate::executor::Executor, page_request: &dyn $crate::plugin::IPageRequest, $($param_key:$param_type,)*) -> std::result::Result<$crate::plugin::Page<$table>, $crate::rbdc::Error> {
            struct Inner{}
            impl Inner{
              #[$crate::py_sql($py_file)]
              pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (executor: &dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key: &$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error>{
                 $crate::impled!()
              }
            }
            let mut total = 0;
            if page_request.do_count() {
               let total_value = Inner::$fn_name(executor, true, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
               total = $crate::decode(total_value).unwrap_or(0);
            }
            let mut page = $crate::plugin::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
            let records_value = Inner::$fn_name(executor, false, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
            page.records = rbs::from_value(records_value)?;
            Ok(page)
         }
    }
}

#[macro_export]
macro_rules! raw_sql_ext {
    ($fn_name:ident $(<$($life_cycle:lifetime $(,)?)*$($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $sql_file:expr) => {
       pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (rb: &dyn $crate::executor::Executor,$($param_key: $param_type,)*) -> $return_type{
           pub struct Inner{};
           impl Inner{
               #[$crate::sql($sql_file)]
               pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (rb: &dyn $crate::executor::Executor,$($param_key: $param_type,)*) -> $return_type{
                 impled!()
               }
           }
           Inner::$fn_name(rb,$($param_key,)*).await
       }
    }
}

#[macro_export]
macro_rules! pysql_ext {
    ($fn_name:ident $(<$($life_cycle:lifetime $(,)?)*$($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $py_file:expr) => {
       pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (rb: &dyn $crate::executor::Executor,$($param_key: $param_type,)*) -> $return_type{
           pub struct Inner{};
           impl Inner{
               #[$crate::py_sql($py_file)]
               pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (rb: &dyn $crate::executor::Executor,$($param_key: $param_type,)*) -> $return_type{
                 impled!()
               }
           }
           Inner::$fn_name(rb,$($param_key,)*).await
       }
    }
}

#[macro_export]
macro_rules! htmlsql_ext {
    ($fn_name:ident $(<$($life_cycle:lifetime $(,)?)*$($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $html_file:expr) => {
        pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (rb: &dyn $crate::executor::Executor,$($param_key: $param_type,)*) -> $return_type{
            pub struct Inner{};
            impl Inner{
            #[$crate::html_sql($html_file)]
            pub async fn $fn_name $(< $($life_cycle,)* $($gkey:$gtype,)*>)? (rb: &dyn $crate::executor::Executor,$($param_key: $param_type,)*) -> $return_type{
              impled!()
             }
           }
           Inner::$fn_name(rb,$($param_key,)*).await
        }
    }
}
