#![cfg(test)]

use dailymotion::api::{EndpointParams, video};

pub mod get_from_id {
    use super::*;

    #[test]
    fn build_url() {
        let params = video::GetFromIdParams {
            id: String::from("x8gkpx9"),
            fields: None,
        };
        let ep = params.build_url(&video::GET_FROM_ID);
        assert_eq!(ep, "/video/x8gkpx9");
    }

    #[test]
    fn call() {
        let params = video::GetFromIdParams {
            id: String::from("x8gkpx9"),
            fields: None,
        };
        let resp = video::GET_FROM_ID
            .call(&params).unwrap();

        assert_eq!(resp.id, "x8gkpx9");
        assert_eq!(resp.owner, "x1mjo36");
        assert_eq!(resp.channel, "news");
    }
}

