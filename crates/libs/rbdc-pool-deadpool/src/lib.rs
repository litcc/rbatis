use std::fmt::Debug;
use std::fmt::Formatter;
use std::time::Duration;

use async_trait::async_trait;
use deadpool::managed::Metrics;
use deadpool::managed::Object;
use deadpool::managed::RecycleError;
use deadpool::managed::RecycleResult;
use deadpool::managed::Timeouts;
use deadpool::Runtime;
use deadpool::Status;
use rbdc::db::Connection;
use rbdc::db::ExecResult;
use rbdc::db::Row;
use rbdc::pool::conn_box::ConnectionBox;
use rbdc::pool::conn_manager::ConnManager;
use rbdc::pool::Pool;
use rbdc::Error;
use rbs::to_value;
use rbs::value::map::ValueMap;
use rbs::Value;

pub struct DeadPool {
    pub manager: ConnManagerProxy,
    pub inner: deadpool::managed::Pool<ConnManagerProxy>,
}

unsafe impl Send for DeadPool {}

unsafe impl Sync for DeadPool {}

impl Debug for DeadPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pool").finish()
    }
}

impl DeadPool {
    /// Retrieves Status of this Pool.
    pub fn status(&self) -> Status {
        self.inner.status()
    }
}

#[async_trait]
impl Pool for DeadPool {
    fn new(manager: ConnManager) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut pool = deadpool::managed::Pool::builder(ConnManagerProxy {
            inner: manager.clone(),
            conn: None,
        });

        #[cfg(all(feature = "rt_tokio", feature = "rt_async_std"))]
        {
            compile_error!(
                "rt_tokio and rt_async_std cannot be enabled at the same time"
            );
        }

        #[cfg(feature = "rt_tokio")]
        {
            pool = pool.runtime(Runtime::Tokio1);
        }

        #[cfg(feature = "rt_async_std")]
        {
            pool = pool.runtime(Runtime::AsyncStd1);
        }

        Ok(Self {
            manager: ConnManagerProxy { inner: manager, conn: None },
            inner: pool
                // .create_timeout(Some(Duration::from_secs(30)))
                .build()
                .map_err(|e| Error::from(e.to_string()))?,
        })
    }

    async fn get(&self) -> Result<Box<dyn Connection>, Error> {
        let v = self.inner.get().await.map_err(|e| Error::from(e.to_string()))?;
        let conn =
            ConnManagerProxy { inner: v.manager_proxy.clone(), conn: Some(v) };
        Ok(Box::new(conn))
    }

    async fn get_timeout(&self, d: Duration) -> Result<Box<dyn Connection>, Error> {
        let out = Timeouts { create: Some(d), wait: Some(d), ..Default::default() };
        let v = self
            .inner
            .timeout_get(&out)
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let conn =
            ConnManagerProxy { inner: v.manager_proxy.clone(), conn: Some(v) };
        Ok(Box::new(conn))
    }

    async fn set_conn_max_lifetime(&self, _max_lifetime: Option<Duration>) {
        //un impl
    }

    async fn set_max_idle_conns(&self, _n: u64) {
        //un impl
    }

    async fn set_max_open_conns(&self, n: u64) {
        self.inner.resize(n as usize)
    }

    async fn state(&self) -> Value {
        let mut m = ValueMap::with_capacity(10);
        let state = self.status();
        m.insert(to_value!("max_size"), to_value!(state.max_size));
        m.insert(to_value!("size"), to_value!(state.size));
        m.insert(to_value!("available"), to_value!(state.available));
        m.insert(to_value!("waiting"), to_value!(state.waiting));
        Value::Map(m)
    }

    fn driver_type(&self) -> &str {
        self.manager.inner.driver_type()
    }
}

pub struct ConnManagerProxy {
    pub inner: ConnManager,
    pub conn: Option<Object<ConnManagerProxy>>,
}

// #[async_trait]
impl deadpool::managed::Manager for ConnManagerProxy {
    type Type = ConnectionBox;

    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.inner.connect().await
    }

    async fn recycle(
        &self,
        obj: &mut Self::Type,
        _metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        if obj.conn.is_none() {
            return Err(RecycleError::Message("none".into()));
        }
        self.inner.check(obj).await?;
        Ok(())
    }
}

use std::future::Future;
use std::pin::Pin;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

impl Connection for ConnManagerProxy {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        self.conn.as_mut().unwrap().get_rows(sql, params)
    }

    fn exec(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<ExecResult, Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        self.conn.as_mut().unwrap().exec(sql, params)
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        Box::pin(async { self.conn.as_mut().unwrap().ping().await })
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        Box::pin(async { self.conn.as_mut().unwrap().close().await })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
