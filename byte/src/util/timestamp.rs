use twilight_model::util::Timestamp;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TimestampStyle {
    ShortTime,
    LongTime,
    #[default]
    ShortDate,
    LongDate,
    FullShort,
    FullLong,
    Relative,
}

pub trait TimestampMention {
    fn mention(&self, style: TimestampStyle) -> String;
}

impl TimestampMention for Timestamp {
    fn mention(&self, style: TimestampStyle) -> String {
        let secs = self.as_secs();

        match style {
            TimestampStyle::ShortTime => format!("<t:{}:t>", secs),
            TimestampStyle::LongTime => format!("<t:{}:T>", secs),
            TimestampStyle::ShortDate => format!("<t:{}:d>", secs),
            TimestampStyle::LongDate => format!("<t:{}:D>", secs),
            TimestampStyle::FullShort => format!("<t:{}:f>", secs),
            TimestampStyle::FullLong => format!("<t:{}:F>", secs),
            TimestampStyle::Relative => format!("<t:{}:R>", secs),
        }
    }
}

#[test]
fn test_timestamp() -> anyhow::Result<()> {
    let timestamp = Timestamp::from_secs(1_000_000_000)?;

    assert_eq!(timestamp.mention(TimestampStyle::ShortTime), "<t:1000000000:t>");
    assert_eq!(timestamp.mention(TimestampStyle::LongTime), "<t:1000000000:T>");
    assert_eq!(timestamp.mention(TimestampStyle::ShortDate), "<t:1000000000:d>");
    assert_eq!(timestamp.mention(TimestampStyle::LongDate), "<t:1000000000:D>");
    assert_eq!(timestamp.mention(TimestampStyle::FullShort), "<t:1000000000:f>");
    assert_eq!(timestamp.mention(TimestampStyle::FullLong), "<t:1000000000:F>");
    assert_eq!(timestamp.mention(TimestampStyle::Relative), "<t:1000000000:R>");

    Ok(())
}