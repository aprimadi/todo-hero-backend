use rocksdb::{ColumnFamilyDescriptor, DB, DBWithThreadMode, Options, SingleThreaded};

/// Storage
///
/// users: email -> null
/// tokens: token -> email
/// states: email -> { head: <path>, prevs: [<path>] }
///
struct Herodb {
    path: String,
    users_cf: ColumnFamilyDescriptor,
    tokens_cf: ColumnFamilyDescriptor,
    db: Option<DBWithThreadMode<SingleThreaded>>,
}

impl Herodb {
    fn new(path: String) -> Self {
        let mut opts = Options::default();
        opts.set_max_write_buffer_number(16);
        let users_cf = ColumnFamilyDescriptor::new("users", opts);
        let tokens_cf = ColumnFamilyDescriptor::new("tokens", opts);
        Self {
            path,
            users_cf,
            tokens_cf,
            db: None,
        }
    }

    /// This will initialize all column families
    fn open(&mut self) {
        let mut opts = Options::default();
        opts.create_missing_column_families(true);
        opts.create_if_missing(true);
        let db = DB::open_cf_descriptors(
            &opts, self.path, vec![self.users_cf, self.tokens_cf]
        ).unwrap();
        self.db = Some(db);
    }

    fn destroy(&mut self) {
        DB::destroy(&self.opts, path);
        self.db = None;
    }
}

struct SigninForm {
    auth_code: String,
}

#[tokio::main]
fn main() {
    let db = Herodb::new(String::from("_db"));
    db.open();

    let signin = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    let signin = warp::post()
        .and(warp::path("signin"))
        .and(warp::body::json())
        .map(|mut form: SigninForm| {
            // TODO
        });

    let fetch_state = warp::get()
        .and(warp::path("state"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("token") {
            Some(key) => {
                // TODO
                Response::builder().body(format!("key = {}", key))
            }
            None => {
                // TODO
                Response::builder().body(String::from("No \"key\" param in query."))
            }
        });

    let store_state = warp::post()
        .and(warp::path("state"))
        .and(warp::body::json())
        .map(|mut form: SigninForm| {
            // TODO
        });

    // POST /signin params: { 'auth_code' } returns: { 'token' }
    // GET /state params: { 'token' }, returns: { 'state' }
    // POST /state params: { 'token', 'state' }, returns: {}
    let routes = signin
        .or(fetch_state)
        .or(store_state);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    db.destroy();
}
