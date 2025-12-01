$ErrorActionPreference = "Stop"
$baseUrl = "http://127.0.0.1:3000" # Use IP to avoid IPv6 localhost issues
$sessionId = [Guid]::NewGuid().ToString()

Write-Host "Testing with Session ID: $sessionId"

# 0. Check if port is open
Write-Host "[0] Checking connectivity to 127.0.0.1:3000..."
try {
    $tcp = New-Object System.Net.Sockets.TcpClient
    $connect = $tcp.BeginConnect("127.0.0.1", 3000, $null, $null)
    $wait = $connect.AsyncWaitHandle.WaitOne(2000, $false)
    if (-not $wait) {
        Write-Error "   Port 3000 is not reachable."
        Write-Host "   SUGGESTION: Please ensure you have run 'cargo run --bin server' in a separate terminal."
        Write-Host "   SUGGESTION: Wait until you see 'Listening on http://127.0.0.1:3000' before running this script."
        exit 1
    }
    $tcp.EndConnect($connect)
    $tcp.Close()
    Write-Host "   Port 3000 is open."
} catch {
    Write-Error "   Could not connect to port 3000."
    Write-Error "   Error details: $_"
    exit 1
}

# 1. Send Chat Message
Write-Host "`n[1] Sending Chat Message..."
# Note: The Rust backend uses #[serde(rename_all = "kebab-case")]
# So we must use kebab-case keys (e.g. "model-name", "session-id")
$payload = @{
    "model-name" = "Qwen/Qwen2.5-0.5B-Instruct"
    "prompt" = "My name is AutoTest."
    "session-id" = $sessionId
    "max-token" = 10
} | ConvertTo-Json

try {
    # Increased timeout to 300s (5 mins) because first run might download/load model
    Write-Host "   Sending request (Timeout: 300s)..."
    Write-Host "   Note: If this is the first request, the server might be downloading the model. This can take a while."
    $response = Invoke-WebRequest -Uri "$baseUrl/chat/completions" -Method Post -Body $payload -ContentType "application/json" -TimeoutSec 300
    Write-Host "   Response received (Status: $($response.StatusCode))"
} catch {
    Write-Host "   [ERROR] Failed to send message." -ForegroundColor Red
    Write-Host "   [ERROR] Exception Message: $($_.Exception.Message)" -ForegroundColor Red
    
    if ($_.Exception.Response) {
        try {
            $stream = $_.Exception.Response.GetResponseStream()
            if ($stream) {
                $reader = New-Object System.IO.StreamReader $stream
                $errBody = $reader.ReadToEnd()
                Write-Host "   [SERVER ERROR DETAILS]: $errBody" -ForegroundColor Yellow
            }
        } catch {
            Write-Host "   [ERROR] Could not read server error response." -ForegroundColor Red
        }
    }
    exit 1
}

# 2. Check History API
Write-Host "`n[2] Checking History API..."
try {
    $history = Invoke-RestMethod -Uri "$baseUrl/chat/history/$sessionId" -Method Get
    if ($history -and $history.Count -ge 2) {
        Write-Host "   Success: History retrieved. Message count: $($history.Count)"
        $userMsg = $history | Where-Object { $_.role -eq "user" } | Select-Object -Last 1
        if ($userMsg.content -eq "My name is AutoTest.") {
            Write-Host "   Success: Content verified."
        } else {
            Write-Warning "   Warning: Content mismatch. Got: $($userMsg.content)"
        }
    } else {
        Write-Error "   Failed: History is empty or invalid."
    }
} catch {
    Write-Error "   Failed to retrieve history."
    Write-Error "   Exception: $($_.Exception.Message)"
}

# 3. Check File Persistence
Write-Host "`n[3] Checking sessions.json..."
if (Test-Path "sessions.json") {
    try {
        $jsonContent = Get-Content "sessions.json" -Raw | ConvertFrom-Json
        # Access the property dynamically using the session ID
        if ($jsonContent.PSObject.Properties[$sessionId]) {
            Write-Host "   Success: Session found in sessions.json"
        } else {
            Write-Warning "   Warning: Session ID not found in sessions.json"
        }
    } catch {
        Write-Warning "   Warning: Could not parse sessions.json"
    }
} else {
    Write-Warning "   Warning: sessions.json not found in current directory."
}

Write-Host "`nTest Complete."
