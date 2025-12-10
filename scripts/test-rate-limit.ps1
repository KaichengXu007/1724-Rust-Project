param(
  [string]$Url = 'http://127.0.0.1:3000/sessions',
  [int]$Count = 70,
  [int]$DelayMs = 100
)

Write-Host "Testing rate limit against: $Url (requests: $Count, delay: ${DelayMs}ms)"

for ($i = 1; $i -le $Count; $i++) {
  try {
    $r = Invoke-WebRequest -Uri $Url -Method Get -ErrorAction Stop
    $code = [int]$r.StatusCode
    Write-Host "ok $i ($code)"
    Write-Host "  X-RateLimit-Limit:" $($r.Headers['X-RateLimit-Limit']) "Remaining:" $($r.Headers['X-RateLimit-Remaining'])
  } catch {
    $err = $_.Exception
    if ($err.Response -ne $null) {
      $resp = $err.Response
      $code = [int]$resp.StatusCode
      $reader = New-Object System.IO.StreamReader($resp.GetResponseStream())
      $body = $reader.ReadToEnd()
      Write-Host "error $i ($code): $body"
      foreach ($hk in $resp.Headers.AllKeys) {
        Write-Host ("  {0}: {1}" -f $hk, $resp.Headers[$hk])
      }
    } else {
      Write-Host ("error {0}: Exception with no Response: {1}" -f $i, $err.Message)
    }
  }
  Start-Sleep -Milliseconds $DelayMs
}

Write-Host "Done testing."
