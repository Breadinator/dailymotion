use crate::{
    api::{Endpoint, EndpointParams, Fielder, HttpRequestMethod},
    endpoint,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};
use chrono::{
    Utc,
    DateTime,
};
use chrono::serde::ts_seconds_option::deserialize as from_tsopt;

// pub static GET: Endpoint<()> = endpoint!("/videos", HttpRequestMethod::Get, false, &[]);

pub static GET_FROM_ID: Endpoint<GetFromIdParams, GetFromIdResponse> =
    endpoint!("/video/{id}", HttpRequestMethod::Get, false, &[]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFromIdParams {
    pub id: String,
    pub fields: Option<Vec<VideoField>>,
}

impl EndpointParams for GetFromIdParams {
    fn build_url<T: EndpointParams, P: DeserializeOwned>(
        &self,
        endpoint: &Endpoint<T, P>,
    ) -> String {
        let mut url = endpoint.url.to_owned().replace("{id}", &self.id);

        if let Some(fields) = &self.fields {
            if !fields.is_empty() {
                url.push('?');
                url.push_str(&fields.generate_fields_string());
            }
        }

        url
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetFromIdResponse {
    pub advertising_custom_target: Option<String>,
    pub advertising_instream_blocked: Option<bool>,
    pub allow_embed: Option<bool>,
    pub allowed_in_playlists: Option<bool>,
    pub aspect_ratio: Option<i32>,
    pub audience: Option<i32>,
    pub audience_total: Option<i32>,
    pub audience_url: Option<String>,
    pub available_formats: Option<Vec<String>>,
    pub channel: Option<String>,
    pub checksum: Option<String>,
    pub claim_rule_blocked_countries: Option<Vec<String>>,
    pub claim_rule_monetized_countries: Option<Vec<String>>,
    pub claim_rule_tracked_countries: Option<Vec<String>>,
    pub content_provider: Option<String>,
    pub content_provider_id: Option<String>,
    pub country: Option<String>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub created_time: Option<DateTime<Utc>>,
    pub custom_classification: Option<Vec<String>>,
    pub description: Option<String>,
    pub duration: Option<i32>,
    pub embed_html: Option<String>,
    pub embed_url: Option<String>,
    pub encoding_progress: Option<i32>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub expiry_date: Option<DateTime<Utc>>,
    pub expiry_date_availability: Option<bool>,
    pub expiry_date_deletion: Option<bool>,
    pub explicit: Option<bool>,
    pub filmstrip_60_url: Option<String>,
    pub geoblocking: Option<Vec<String>>,
    pub geoloc: Option<Vec<String>>,
    pub height: Option<i32>,
    pub id: Option<String>,
    pub is_created_for_kids: Option<bool>,
    pub item_type: Option<String>,
    pub language: Option<String>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub liked_at: Option<DateTime<Utc>>,
    pub likes_total: Option<i32>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub live_ad_break_end_time: Option<DateTime<Utc>>,
    pub live_ad_break_launch: Option<i32>,
    pub live_ad_break_remaining: Option<i32>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub live_airing_time: Option<DateTime<Utc>>,
    pub live_audio_bitrate: Option<i32>,
    pub live_auto_record: Option<bool>,
    pub live_ingests: Option<HashMap<String, String>>,
    pub live_publish_url: Option<String>,
    pub log_external_view_urls: Option<HashMap<String, String>>,
    pub log_view_url: Option<String>,
    pub log_view_urls: Option<HashMap<String, String>>,
    pub media_type: Option<String>,
    pub mode: Option<String>,
    pub onair: Option<bool>,
    pub owner: Option<String>,
    pub partner: Option<bool>,
    pub password: Option<String>,
    pub password_protected: Option<bool>,
    pub player_next_video: Option<String>,
    pub player_next_videos: Option<Vec<String>>,
    pub preview_240p_url: Option<String>,
    pub preview_360p_url: Option<String>,
    pub preview_480p_url: Option<String>,
    pub private: Option<bool>,
    pub private_id: Option<String>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub publish_date: Option<DateTime<Utc>>,
    pub publish_date_keep_private: Option<bool>,
    pub published: Option<bool>,
    pub publishing_progress: Option<i32>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub record_end_time: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub record_start_time: Option<DateTime<Utc>>,
    pub record_status: Option<String>,
    pub recurrence: Option<String>,
    pub seeker_url: Option<String>,
    pub soundtrack_isrc: Option<String>,
    pub soundtrack_popularity: Option<i32>,
    pub sprite_320x_url: Option<String>,
    pub sprite_url: Option<String>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub start_time: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub stream_audio_url: Option<String>,
    pub stream_h264_hd1080_url: Option<String>,
    pub stream_h264_hd_url: Option<String>,
    pub stream_h264_hq_url: Option<String>,
    pub stream_h264_l1_url: Option<String>,
    pub stream_h264_l2_url: Option<String>,
    pub stream_h264_ld_url: Option<String>,
    pub stream_h264_qhd_url: Option<String>,
    pub stream_h264_uhd_url: Option<String>,
    pub stream_h264_url: Option<String>,
    pub stream_hls_url: Option<String>,
    pub stream_live_hls_url: Option<String>,
    pub stream_live_rtmp_url: Option<String>,
    pub stream_live_smooth_url: Option<String>,
    pub stream_source_url: Option<String>,
    pub studio: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_60_url: Option<String>,
    pub thumbnail_62_url: Option<String>,
    pub thumbnail_120_url: Option<String>,
    pub thumbnail_180_url: Option<String>,
    pub thumbnail_240_url: Option<String>,
    pub thumbnail_360_url: Option<String>,
    pub thumbnail_480_url: Option<String>,
    pub thumbnail_720_url: Option<String>,
    pub thumbnail_1080_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub tiny_url: Option<String>,
    pub title: Option<String>,
    #[serde(default, deserialize_with = "from_tsopt")]
    pub updated_time: Option<DateTime<Utc>>,
    pub url: Option<String>,
    pub verified: Option<bool>,
    pub views_last_day: Option<i32>,
    pub views_last_hour: Option<i32>,
    pub views_last_month: Option<i32>,
    pub views_last_week: Option<i32>,
    pub views_total: Option<i32>,
    pub width: Option<i32>,
}

#[allow(clippy::module_name_repetitions, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
pub enum VideoField {
    advertising_custom_target,
    advertising_instream_blocked,
    allow_embed,
    allowed_in_playlists,
    aspect_ratio,
    audience,
    audience_total,
    audience_url,
    available_formats,
    channel,
    checksum,
    claim_rule_blocked_countries,
    claim_rule_monetized_countries,
    claim_rule_tracked_countries,
    content_provider,
    content_provider_id,
    country,
    created_time,
    custom_classification,
    description,
    duration,
    embed_html,
    embed_url,
    encoding_progress,
    end_time,
    expiry_date,
    expiry_date_availability,
    expiry_date_deletion,
    explicit,
    filmstrip_60_url,
    geoblocking,
    geoloc,
    height,
    id,
    is_created_for_kids,
    item_type,
    language,
    liked_at,
    likes_total,
    live_ad_break_end_time,
    live_ad_break_launch,
    live_ad_break_remaining,
    live_airing_time,
    live_audio_bitrate,
    live_auto_record,
    live_ingests,
    live_publish_url,
    log_external_view_urls,
    log_view_url,
    log_view_urls,
    media_type,
    mode,
    onair,
    owner,
    partner,
    password,
    password_protected,
    player_next_video,
    player_next_videos,
    preview_240p_url,
    preview_360p_url,
    preview_480p_url,
    private,
    private_id,
    publish_date,
    publish_date_keep_private,
    published,
    publishing_progress,
    record_end_time,
    record_start_time,
    record_status,
    recurrence,
    seeker_url,
    soundtrack_isrc,
    soundtrack_popularity,
    sprite_320x_url,
    sprite_url,
    start_time,
    status,
    stream_audio_url,
    stream_h264_hd1080_url,
    stream_h264_hd_url,
    stream_h264_hq_url,
    stream_h264_l1_url,
    stream_h264_l2_url,
    stream_h264_ld_url,
    stream_h264_qhd_url,
    stream_h264_uhd_url,
    stream_h264_url,
    stream_hls_url,
    stream_live_hls_url,
    stream_live_rtmp_url,
    stream_live_smooth_url,
    stream_source_url,
    studio,
    tags,
    thumbnail_60_url,
    thumbnail_62_url,
    thumbnail_120_url,
    thumbnail_180_url,
    thumbnail_240_url,
    thumbnail_360_url,
    thumbnail_480_url,
    thumbnail_720_url,
    thumbnail_1080_url,
    thumbnail_url,
    tiny_url,
    title,
    updated_time,
    url,
    verified,
    views_last_day,
    views_last_hour,
    views_last_month,
    views_last_week,
    views_total,
    width,
}

impl Fielder for Vec<VideoField> {
    fn generate_fields_string(&self) -> String {
        let mut fields = String::new();

        if self.is_empty() {
            return fields;
        }

        fields.push_str("fields=");
        fields.push_str(
            &self
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("%2C"),
        );

        fields
    }
}

