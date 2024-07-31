// https://en.wikipedia.org/wiki/Media_type

#[derive(Debug)]
pub enum AcceptEncoding {
    Gzip,
    Deflate,
    Br,
    Zstd,
    Identity,
    Any,
    Qvalues,
}

#[derive(Debug)]
pub enum ContentTypes {
    Plain,
    Css,
    Csv,
    Html,
    Javascript,
    Xml,
    Json,
    Pdf,
    Sql,
    Zip,
    Mpeg,
    Avif,
    Jpeg,
    Png,
    Ogg,
    Webp,
    Svg,
    Any,
}
