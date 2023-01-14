#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Scopes {
    pub scopes: Vec<Scope>,
}

impl Scopes {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S> From<Vec<S>> for Scopes where S: Into<Scope> {
    fn from(scopes: Vec<S>) -> Self {

        Self {
            scopes: scopes.into_iter().map(std::convert::Into::into).collect()
        }
    }
}

impl<'a, S: From<&'a Scope>> From<&'a Scopes> for Vec<S> {
    fn from(scopes: &'a Scopes) -> Vec<S> {
        scopes.scopes
            .iter()
            .map(std::convert::Into::into)
            .collect()
    }
}

#[macro_export]
macro_rules! scopes {
    () => { dailymotion::Scopes::new() };
    ( $( $x:expr ),* ) => { dailymotion::Scopes::from(vec![$($x,)*]) };
}

/// Represents a scope. Scopes are in snake case, e.g. [`Scope::Manage_Videos`] is for the scope `manage_videos`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    Email,
    Userinfo,
    Feed,
    ManageVideos,
    ManagePlaylists,
    ManageSubscriptions,
    ManageLikes,
    ManageRecords,
    ManageSubtitles,
    ManageFeatures,
    ManageHistory,
    ReadInsights,
    ManageClaimRules,
    ManageAnalytics,
    ManagePlayer,
    ManagePlayers,
    ManageUserSettings,
    ManageAppConnections,
    ManageApplications,
    ManageDomains,
    ManagePodcasts,

    /// Any other scope
    Other(String),
}

impl From<&str> for Scope {
    fn from(scope: &str) -> Self {
        match scope {
            "email" => Self::Email,
            "userinfo" => Self::Userinfo,
            "feed" => Self::Feed,
            "manage_videos" => Self::ManageVideos,
            "manage_playlists" => Self::ManagePlaylists,
            "manage_subscriptions" => Self::ManageSubscriptions,
            "manage_likes" => Self::ManageLikes,
            "manage_records" => Self::ManageRecords,
            "manage_subtitles" => Self::ManageSubtitles,
            "manage_features" => Self::ManageFeatures,
            "manage_history" => Self::ManageHistory,
            "read_insights" => Self::ReadInsights,
            "manage_claim_rules" => Self::ManageClaimRules,
            "manage_analytics" => Self::ManageAnalytics,
            "manage_player" => Self::ManagePlayer,
            "manage_players" => Self::ManagePlayers,
            "manage_user_settings" => Self::ManageUserSettings,
            "manage_app_connections" => Self::ManageAppConnections,
            "manage_applications" => Self::ManageApplications,
            "manage_domains" => Self::ManageDomains,
            "manage_podcasts" => Self::ManagePodcasts,
            s => Self::Other(String::from(s))
        }
    }
}

impl<'a> From<&'a Scope> for &'a str {
    fn from(scope: &'a Scope) -> &'a str {
        match scope {
            Scope::Email => "email",
            Scope::Userinfo => "userinfo",
            Scope::Feed => "feed",
            Scope::ManageVideos => "manage_videos",
            Scope::ManagePlaylists => "manage_playlists",
            Scope::ManageSubscriptions => "manage_subscriptions",
            Scope::ManageLikes => "manage_likes",
            Scope::ManageRecords => "manage_records",
            Scope::ManageSubtitles => "manage_subtitles",
            Scope::ManageFeatures => "manage_features",
            Scope::ManageHistory => "manage_history",
            Scope::ReadInsights => "read_insights",
            Scope::ManageClaimRules => "manage_claim_rules",
            Scope::ManageAnalytics => "manage_analytics",
            Scope::ManagePlayer => "manage_player",
            Scope::ManagePlayers => "manage_players",
            Scope::ManageUserSettings => "manage_user_settings",
            Scope::ManageAppConnections => "manage_app_connections",
            Scope::ManageApplications => "manage_applications",
            Scope::ManageDomains => "manage_domains",
            Scope::ManagePodcasts => "manage_podcasts",
            Scope::Other(s) => s,
        }
    }
}

