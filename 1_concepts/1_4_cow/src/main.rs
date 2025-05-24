use std::borrow::Cow;
use std::env;

fn main() {
    let mut path = Cow::Borrowed("/etc/app/app.conf");

    let mut args = env::args();
    if args.any(|arg| &arg == "--conf") {
        path = args
            .next()
            .expect("the --conf argument should specify a path")
            .into();
    } else if let Ok(env_path) = env::var("APP_CONF") {
        if !env_path.is_empty() {
            path = env_path.into();
        }
    }

    println!("{path}");
}
