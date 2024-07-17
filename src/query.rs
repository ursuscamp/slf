use std::{borrow::Cow, sync::OnceLock, time::Duration};

#[derive(Debug, Clone, clap::Args)]
pub struct Query {
    /// Starting date/datetime
    #[arg(short)]
    pub begin: Option<String>,

    /// Ending date/datetime (non-inclusive)
    #[arg(short)]
    pub end: Option<String>,

    /// Tags to query for (multiple tags may be provided, all must be included)
    #[arg(short)]
    pub tag: Option<Vec<String>>,

    /// Only query from <num> most recent days (alternative to using date ranges)
    #[arg(short)]
    pub recent: Option<u64>,

    /// Return a maximum number of results
    #[arg(short)]
    pub limit: Option<u64>,
}

impl Query {
    pub fn query_map<'a>(&'_ self, log: &'a str) -> Option<Cow<'a, str>> {
        if let Some(begin) = &self.begin {
            if log < begin.as_str() {
                return None;
            }
        }

        if let Some(end) = &self.end {
            if log >= end.as_str() {
                return None;
            }
        }

        // In order to avoid allocations over and over, we will use a String as a
        // pre-allocated storage.
        let mut tag_cap = String::with_capacity(20);
        if let Some(tag) = &self.tag {
            for tag in tag {
                tag_cap.clear();
                tag_cap.push('#');
                tag_cap.push_str(tag);
                if !log.contains(&tag_cap) {
                    return None;
                }
            }
        }

        if let Some(recent) = self.recent_date() {
            if log < recent {
                return None;
            }
        }

        Some(Cow::Borrowed(log))
    }

    fn recent_date(&self) -> Option<&str> {
        static FMT: OnceLock<String> = OnceLock::new();
        let recent = self.recent?;
        Some(FMT.get_or_init(move || {
            let f = time::format_description::parse("[year]-[month]-[day]").unwrap();
            let start =
                time::OffsetDateTime::now_utc() - Duration::from_secs(recent * 24 * 60 * 60);
            start.format(&f).expect("formatting recent time")
        }))
    }
}
