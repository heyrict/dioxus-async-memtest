# Dioxus async memtest

A simple test case to reproduce memory expanding bug in dioxus.

## Setup

```bash
# generate text for use
mkdir -p /tmp/temp;
cd /tmp/temp
for f in $(seq 1 100); do echo $f > $f.txt; done
# Start a static server of your like on port 8080
python -m http.server 8080

# In another process, launch the test app
cd /path/to/dioxus-async-memtest && cargo run
```
