#![cfg(test)]

use dailymotion::*;

#[test]
#[ignore = "includes auth"]
fn basic_auth() -> Result<()> {
    auth::get_using_env()
        .map(|_| ())
}

