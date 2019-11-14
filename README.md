# file_scanner

A simple scanner that iterates through the entire drive (`C:\` in Windows and `/` in Linux) looking for files that match a set of hashes that it has been compiled with, like some ersatz antivirus replacement. Why? It's a really stupid story, involving users I'm supposed to support who apparently lack the neccessary werewithal and competence in life to oh so gently place two different files in one directory and run them. 

Comes with a CGI script to take your hashes compile the neccesary executable, all from the browser, to reduce the possibility of your colleagues stuffing everything up in CLI. Also comes with a Dockerfile so one doesn't have to muck around setting up httpd.

## Running

The Dockerfile sorts out all the dependencies (Rust and httpd). 

```
docker build . -t file-scanner
docker run file-scanner/latest
```

## Using

The server accepts a comma delimited string of hashes which can be a mix of MD5, SHA1 or SHA256. I'm pretty sure that the entire system has about all the safety of a sheep in a tiger pen, but ah well.

For instance, enter this url in one's browser: `http://<host>/file-scanner.cgi?hashes=<hashes>`. Passing no hashes will return a 400.

Alternatively, if one doesn't want to lower one's standards in life with weird web server things, one can always just compile it directly with rust:

```bash
# Set hashes to be compiled. Replace hashes with user-settable-character-delimited string of hashes
export FILE_SCANNER_HASHES=${hashes}
export FILE_SCANNER_DELIMITER=,
cargo build --release --target x86_64-pc-windows-gnu
```

The compiler flags have been set to make the build a static binary (hopefully). For Linux, compile with target `x86_64-unknown-linux-musl` instead. If one specifies a string for the delimiter, only the first character will be used. Empty strings default to a comma.
