extern crate failure;
extern crate hypercore;
extern crate random_access_memory as ram;

use failure::Error;
use hypercore::{Feed, Storage, Store};

fn create_feed(page_size: usize) -> Result<Feed<ram::SyncMethods>, Error> {
  let create = |_store: Store| ram::Sync::new(page_size);
  let storage = Storage::new(create)?;
  Ok(Feed::with_storage(storage)?)
}

#[test]
fn set_get() {
  let mut feed = create_feed(50).unwrap();
  feed.append(b"hello").unwrap();
  feed.append(b"world").unwrap();

  assert_eq!(feed.get(0).unwrap(), Some(b"hello".to_vec()));
  assert_eq!(feed.get(1).unwrap(), Some(b"world".to_vec()));
}
