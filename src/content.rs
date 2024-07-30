// https://en.wikipedia.org/wiki/Media_type

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ContentTypes {
    Application(Application),
    Audio(Audio),
    Image(Image),
    Text(Text),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Application {
    #[serde(rename = "application/json")]
    Json,
    #[serde(rename = "application/pdf")]
    Pdf,
    #[serde(rename = "application/sql")]
    Sql,
    #[serde(rename = "application/xml")]
    Xml,
    #[serde(rename = "application/zip")]
    Zip,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Audio {
    #[serde(rename = "audio/mpeg")]
    Mpeg,
    #[serde(rename = "audio/ogg")]
    Ogg,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Image {
    #[serde(rename = "avif")]
    Avif,
    #[serde(rename = "")]
    Jpeg,
    #[serde(rename = "")]
    Png,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Text {
    #[serde(rename = "text/plain")]
    Plain,
    #[serde(rename = "text/css")]
    Css,
    #[serde(rename = "text/csv")]
    Csv,
    #[serde(rename = "text/html")]
    Html,
    #[serde(rename = "text/javascript")]
    Javascript,
    #[serde(rename = "text/xml")]
    Xml,
}
