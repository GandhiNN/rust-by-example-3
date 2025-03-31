# reqwest-spotify

Note: Access token expires in 1 hour

To get a new one, run the following CURL command:
```
curl -X POST "https://accounts.spotify.com/api/token" \
     -H "Content-Type: application/x-www-form-urlencoded" \
     -d "grant_type=client_credentials&client_id=<your-client-id>&client_secret=<your-client-secret>"
```

You can grab your client-id and client-secret via your Spotify app dashboard:
https://developer.spotify.com/dashboard/

## 2025-03-31
```
{"access_token":"BQDCsPOOXDCKYFuj1TbMJAuLwG7_ei5je_KsGC_36-I6qE_r7i4bgHWqOiL6jEIhsuncAMTz5ArRBVy7OcgigMvouH2cP-jKVSfWanFUMbyRlp68szpy57G_nYOXOLsXfNWX_KMNhU8","token_type":"Bearer","expires_in":3600}
```