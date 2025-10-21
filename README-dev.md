Developer quickstart

Run tests

# Run the full test suite
cargo test

# Run only integration tests
cargo test --test routes_integration

Run the mock server (Windows PowerShell)

# Start mock server in foreground (Ctrl+C to stop)
cargo run --bin mock_server

# Windows PowerShell: create body.json and post with curl.exe (works reliably)
Set-Content -Path body.json -Value '{"model-name":"mock-model","model-dir":"models/","prompt":"hello","repeat-penalty":1.0,"stop":[]}' -NoNewline
curl.exe -N -H "Content-Type: application/json" --data-binary "@body.json" http://127.0.0.1:3000/chat/completions
Remove-Item body.json

# Alternative (here-string) — safer in scripts
@'
{"model-name":"mock-model","model-dir":"models/","prompt":"hello","repeat-penalty":1.0,"stop":[]}
'@ | curl.exe -N -H "Content-Type: application/json" --data-binary "@-" http://127.0.0.1:3000/chat/completions

Notes
- Use curl.exe (not the PowerShell alias) to avoid Invoke-WebRequest interfering with arguments.
- If port 3000 is in use, either stop the other process or change the port in `src/bin/mock_server.rs`.
