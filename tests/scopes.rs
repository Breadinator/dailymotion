#![cfg(test)]

use dailymotion::{Scope, Scopes, scopes};

#[test]
fn basic_conversion() {
    let scopes = vec![Scope::Feed, Scope::ManagePlayer];
    let parsed: Scopes = scopes.into();
    assert_eq!(parsed.scopes, vec![Scope::Feed, Scope::ManagePlayer]);
}

#[test]
fn basic_conversion_str() {
    let scopes = vec!["userinfo", "manage_subscriptions", "manage_likes"];
    let parsed: Scopes = scopes.into();
    assert_eq!(parsed.scopes, vec![Scope::Userinfo, Scope::ManageSubscriptions, Scope::ManageLikes]);
}

#[test]
fn basic_conversion_other() {
    let scope = "doesnt_exist_lol";
    let parsed: Scope = scope.into();
    assert_eq!(parsed, Scope::Other(String::from(scope)));
}

#[test]
fn convert_into_vec_str() {
    let scopes = scopes![Scope::Userinfo, Scope::ManageApplications];
    let scopes_as_str: Vec<&str> = (&scopes).into();
    assert_eq!(scopes_as_str, vec!["userinfo", "manage_applications"]);
}

