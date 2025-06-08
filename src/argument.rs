use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Argument {
    /// Debug level
    ///
    /// * `i` - info
    /// * `d` - detailed
    #[arg(short, long, default_value_t = String::from("i"))]
    pub debug: String,

    /// Log format for given `source`
    ///
    /// * `nginx`
    #[arg(short, long, default_value_t = String::from("nginx"))]
    pub format: String,

    /// Export results to JSON file (e.g. `/path/to/stats.json`)
    #[arg(long)]
    pub export_json: Option<String>,

    /// Export results to SVG file (e.g. `/path/to/badge.svg`)
    ///
    /// * use `{hits}` / `{hosts}` pattern to replace parsed values
    #[arg(long)]
    pub export_svg: Option<String>,

    /// Use custom SVG file template with `{hits}` / `{hosts}` placeholders
    #[arg(long, default_value_t = String::from("default/counter.svg"))]
    pub template_svg: String,

    /// Expected memory index capacity
    #[arg(short, long, default_value_t = 100)]
    pub capacity: usize,

    /// Exclude host(s) from index
    #[arg(short, long)]
    pub ignore_host: Vec<String>,

    /// Access log source (e.g. `/var/nginx/access.log`)
    #[arg(short, long)]
    pub source: String,

    /// Update delay in seconds
    #[arg(short, long, default_value_t = 300)]
    pub update: u64,
}
