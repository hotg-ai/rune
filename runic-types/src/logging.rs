use alloc::borrow::Cow;
use log::{Level, Record};

/// A serializable version of [`log::Record`].
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerializableRecord<'a> {
    pub level: Level,
    pub message: Cow<'a, str>,
    pub target: Cow<'a, str>,
    pub module_path: Option<Cow<'a, str>>,
    pub file: Option<Cow<'a, str>>,
    pub line: Option<u32>,
}

impl<'a> SerializableRecord<'a> {
    pub fn into_owned(self) -> SerializableRecord<'static> {
        let SerializableRecord {
            level,
            message,
            target,
            module_path,
            file,
            line,
        } = self;

        SerializableRecord {
            level,
            message: message.into_owned().into(),
            target: target.into_owned().into(),
            module_path: module_path.map(|m| m.into_owned().into()),
            file: file.map(|f| f.into_owned().into()),
            line,
        }
    }

    pub fn with_record<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Record<'_>) -> R,
    {
        f(&Record::builder()
            .level(self.level)
            .args(format_args!("{}", self.message.as_ref()))
            .target(self.target.as_ref())
            .module_path(self.module_path.as_deref())
            .file(self.file.as_deref())
            .line(self.line)
            .build())
    }
}

impl<'a> From<&'a Record<'a>> for SerializableRecord<'a> {
    fn from(r: &'a Record<'a>) -> Self {
        SerializableRecord {
            level: r.level(),
            message: Cow::Owned(alloc::format!("{}", r.args())),
            target: r.target().into(),
            module_path: r.module_path().map(Cow::Borrowed),
            file: r.file().map(Cow::Borrowed),
            line: r.line(),
        }
    }
}

impl<'a> Default for SerializableRecord<'a> {
    fn default() -> Self {
        SerializableRecord {
            level: Level::Info,
            message: Default::default(),
            target: Default::default(),
            module_path: Default::default(),
            file: Default::default(),
            line: Default::default(),
        }
    }
}
