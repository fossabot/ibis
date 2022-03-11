fn main() {
    
    let _guard = sentry::init(("https://2fb0e3d203c2413b9b20305ef37ccd97@o1034490.ingest.sentry.io/6254733", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    println!("Hello, world!");
}
