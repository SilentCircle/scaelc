extern crate docopt;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use docopt::Docopt;
use serde_json::value::Value;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;


#[derive(Debug, Deserialize)]
struct Args {
    arg_apikey: String,
    flag_output: String,
    flag_envelope: bool,
    flag_server: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    count: i32,
    total: i32,
    version: i32,
    result: String,
    tid: String,
    since: String,
    until: String,
    logs: Value,
}


const USAGE: &'static str = "
SC Audit Event Log Collector.

Usage:
  scaelc [options] <apikey>

Options:
  -o FILE, --output=FILE    Where to write the output (default is stdout).
  -e --envelope             Add a JSON list envelope to the output.

  --server=SERVER           The domain of the server to contact
                            [default: https://accounts.silentcircle.com].

  -h --help                 Show this screen.
  --version                 Show version.
";


fn write_logs(logs: Vec<Value>, filename: String, envelope: bool) {
    // Select the proper writer (file or stdout).
    let mut output: Box<Write> = if filename.is_empty() {
        Box::new(io::stdout())
    } else {
        let path = Path::new(&filename);
        Box::new(File::create(&path).unwrap())
    };

    if envelope {
        // Wrap the lines in a list [].
        serde_json::to_writer(output, &logs).expect("Could not write output.");
    } else {
        // No envelope.
        for line in logs.iter() {
            write!(&mut output, "{}\n", line.to_string()).expect("Could not write output.");
        }
    }
    println!("Done writing.");
}


fn fetch_logs(url: String, api_key: String) -> Result<Vec<Value>, Box<Error>> {
    let mut logs: Vec<Value> = Vec::new();
    let mut after = String::new();

    loop {
        let url = format!("{}/scmc/api/logs/?api_key={}&after={}", url, api_key, after);

        let mut resp = reqwest::get(url.as_str())?;

        if !resp.status().is_success() {
            Err(format!(
                "There was an error contacting the server: {:?}",
                resp.status()
            ))?;
        }
        let json: Data = resp.json()?;
        logs.extend(json.logs.as_array().unwrap().clone());

        if json.count == json.total {
            break;
        };
        after = json.until;
    }

    Ok(logs)
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Fetching logs...");
    let logs = fetch_logs(args.flag_server, args.arg_apikey);

    match logs {
        Ok(data) => write_logs(data, args.flag_output, args.flag_envelope),
        Err(e) => {
            println!("There was an error contacting the server: {}", e);
            process::exit(1);
        }
    };
}
