#![cfg(test)]

use dailymotion::api::{
    video::{self, VideoField},
    EndpointParams,
};
use std::str::FromStr;

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
        let resp = video::GET_FROM_ID.call(&params).unwrap();

        assert_eq!(resp.id.unwrap(), "x8gkpx9");
        assert_eq!(resp.owner.unwrap(), "x1mjo36");
    }

    #[test]
    fn call_with_specified_fields() {
        let fields = vec!["id", "explicit"];
        let fields: Vec<VideoField> = fields
            .into_iter()
            .map(|f| VideoField::from_str(f).unwrap())
            .collect();
        let params = video::GetFromIdParams {
            id: String::from("x8gkpx9"),
            fields: Some(fields),
        };
        let resp = video::GET_FROM_ID.call(&params).unwrap();

        assert_eq!(resp.id.unwrap(), "x8gkpx9");
        assert_eq!(resp.explicit, Some(false));
    }

    #[test]
    fn call_with_complex_fields() {
        let fields = vec![
            VideoField::live_ingests,
            VideoField::tags,
            VideoField::views_last_day,
            VideoField::width,
        ];
        let params = video::GetFromIdParams {
            id: String::from("x8gkpx9"),
            fields: Some(fields),
        };
        let resp = video::GET_FROM_ID.call(&params).unwrap();

        assert!(resp.live_ingests.is_some());
        assert_eq!(resp.tags.unwrap(), Vec::<String>::new());
        assert!(resp.views_last_day.is_some());
        assert_eq!(resp.width.unwrap(), 720);
    }

    #[test]
    fn datetime_parse() {
        let params = video::GetFromIdParams {
            id: String::from("x8gkpx9"),
            fields: Some(vec![VideoField::created_time, VideoField::end_time])
        };
        let resp = video::GET_FROM_ID.call(&params).unwrap();

        assert_eq!(resp.created_time.unwrap().timestamp(), 1671808208);
        assert!(resp.end_time.is_none()); // wasn't a livestream
    }
}

