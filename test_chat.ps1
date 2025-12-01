$headers = @{
    "Content-Type" = "application/json"
}

$body = @{
    "model-name" = "Qwen/Qwen2.5-0.5B-Instruct"
    "prompt" = "Hello, who are you?"
    "session-id" = "debug-session-001"
    "max-token" = 100
    "temperature" = 0.7
    "device" = "cuda"
} | ConvertTo-Json

Write-Host "Sending request to http://localhost:3000/chat/completions..."
try {
    Invoke-RestMethod -Uri "http://localhost:3000/chat/completions" -Method Post -Headers $headers -Body $body -TimeoutSec 600
} catch {
    Write-Host "Error: $_"
    Write-Host $_.Exception.Response
}
