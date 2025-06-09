mod argument;
mod debug;

fn main() -> anyhow::Result<()> {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader, Write},
        thread,
        time::Duration,
    };

    let argument = {
        use clap::Parser;
        argument::Argument::parse()
    };

    // parse some arguments once
    let is_debug_i = argument.debug.contains("i");
    let is_debug_d = argument.debug.contains("d");

    if !matches!(argument.format.to_lowercase().as_str(), "nginx") {
        todo!("Format `{}` yet not supported!", argument.format)
    }

    if is_debug_i {
        debug::info("Crawler started");
    }

    loop {
        if is_debug_i {
            debug::info("Index queue begin...");
        }

        let now = argument
            .match_time
            .as_ref()
            .map(|t| chrono::Local::now().format(t).to_string());

        let mut index: HashMap<String, usize> = HashMap::with_capacity(argument.index_capacity);

        'l: for l in BufReader::new(File::open(&argument.source)?).lines() {
            let line = l?;

            if let Some(ref t) = now {
                if !line.contains(t) {
                    if is_debug_d {
                        debug::info(&format!("Record time mismatch time filter {t}"))
                    }
                    continue;
                }
            }

            let host = line
                .split_whitespace()
                .next()
                .map(|s| s.into())
                .unwrap_or_default();

            for h in &argument.ignore_host {
                if h == &host {
                    if is_debug_d {
                        debug::info(&format!("Host `{h}` ignored by settings"))
                    }
                    continue 'l;
                }
            }

            index.entry(host).and_modify(|c| *c += 1).or_insert(1);
        }

        let hosts = index.len();
        let hits: usize = index.values().sum();

        if is_debug_i {
            debug::info(&format!(
                "Index queue completed:\n{}\n\thosts: {} / hits: {}, await {} seconds to continue...",
                if is_debug_d {
                    let mut b = Vec::with_capacity(hosts);
                    for (host, count) in &index {
                        b.push(format!("\t{} ({})", host, count))
                    }
                    b.join("\n")
                } else {
                    "".into()
                },
                hosts,
                hits,
                argument.update,
            ));
        }

        if let Some(ref p) = argument.target_json {
            let mut f = File::create(p)?;
            f.write_all(format!("{{\"hosts\":{hosts},\"hits\":{hits}}}").as_bytes())?;
        }

        if let Some(ref p) = argument.target_svg {
            let t = std::fs::read_to_string(&argument.template_svg)?;
            let mut f = File::create(p)?;
            f.write_all(
                t.replace("{hosts}", &hosts.to_string())
                    .replace("{hits}", &hits.to_string())
                    .as_bytes(),
            )?;
        }

        thread::sleep(Duration::from_secs(argument.update));
    }
}
