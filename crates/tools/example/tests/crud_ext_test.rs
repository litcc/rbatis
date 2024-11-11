
#[cfg(test)]
mod test{
    use std::path::Path;
    use rbatis::{crud_ext, impl_delete_ext, impl_insert_ext, impl_select_ext, impl_update_ext};
    use rbdc::DateTime;

    /// table
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Activity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<DateTime>,
        pub version: Option<i64>,
        pub delete_flag: Option<i32>,
    }


    // impl_insert_ext2!(Activity{},"a");
    // impl_select_ext2!(Activity{},"a");
    // impl_update_ext2!(Activity{},"a");
    impl_delete_ext!(Activity{},"a");

    #[tokio::test]
    pub async fn test1(){



    }



}