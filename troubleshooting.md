# Troubleshooting

## Docker/Podman credential error

    error getting credentials - err: exec: "docker-credential-desktop": executable file not found in $PATH, out: ``

or

    error getting credentials - err: exec: "docker-credential-osxkeychain": executable file not found in $PATH, out: ``

These failures occur on the macOS server because:
1. Docker Desktop is not running within an SSH session
2. Apple prevents SSH users from accessing the macOS keychain

Mitigate the issue by simply removing the "credsStore" property form `~/.docker/config.json`.
