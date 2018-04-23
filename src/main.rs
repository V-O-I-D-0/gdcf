extern crate gdcf;
extern crate gdrs;

extern crate chrono;
extern crate futures;
extern crate gdcf_dbcache;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;
extern crate env_logger;

use gdrs::BoomlingsClient;

use chrono::Duration;
use futures::Async;
use futures::Future;
use gdcf::api::request::level::SearchFilters;
use gdcf::api::request::LevelsRequest;
use gdcf::Gdcf;
use gdcf_dbcache::cache::DatabaseCache;
use gdcf_dbcache::cache::DatabaseCacheConfig;
use tokio_core::reactor::Core;

fn main() {
    env_logger::init();

    let mut core = Core::new().unwrap();
    let client = BoomlingsClient::new(&core.handle());
    let config = DatabaseCacheConfig::new("postgres://gdcf:gdcf@localhost/gdcf", Duration::seconds(0));
    let cache = DatabaseCache::new(config);

    let gdcf = Gdcf::new(cache, client);

    let lev_req = LevelsRequest::default()
        .search("Under Lavaland".into())
        .filter(SearchFilters::default().featured().uncompleted());

    //gdcf.level(11774780.into());
    //gdcf.levels(lev_req);
    //gdcf.level(11849346.into());

    gdcf.level(44325129.into());

    core.run(Thing {});
}

struct Thing;

impl Future for Thing {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<()>, ()> {
        Ok(Async::NotReady)
    }
}
