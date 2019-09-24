use serde::de;
use serde::de::Deserializer;
use std::fmt;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(deserialize_with = "path_or_seq_path")]
    pub search_dir: Vec<PathBuf>,
}

// courtesy of https://stackoverflow.com/a/43627388
fn path_or_seq_path<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PathBufOrVec(PhantomData<Vec<PathBuf>>);

    impl<'de> de::Visitor<'de> for PathBufOrVec {
        type Value = Vec<PathBuf>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Path or array of paths")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![PathBuf::from(value)])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            de::Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(PathBufOrVec(PhantomData))
}
