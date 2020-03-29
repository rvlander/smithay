extern crate vidoc;
use vidoc::launcher::application_store::ApplicationStore;


fn main () {
  let mut application_store = ApplicationStore::new(); 
  application_store.populate();
  application_store.pretty_print(); 
}
