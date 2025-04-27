# PowerShell script to install rtop-windows.exe from GitHub latest release

# Define installation directory
$INSTALL_DIR = "$env:USERPROFILE\bin"
$BINARY_NAME = "rtop.exe"
$TEMP_DIR = [System.IO.Path]::GetTempPath() + "rtop_install"

# Create directories
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null
New-Item -ItemType Directory -Force -Path $TEMP_DIR | Out-Null

# Fetch the latest release download URL using GitHub API
$REPO = "gohyuhan/rtop"
$API_URL = "https://api.github.com/repos/$REPO/releases/latest"
try {
    $release = Invoke-RestMethod -Uri $API_URL
    $DOWNLOAD_URL = ($release.assets | Where-Object { $_.name -eq "rtop-windows.exe" }).browser_download_url
} catch {
    Write-Error "Failed to query GitHub API: $_"
    Remove-Item -Path $TEMP_DIR -Recurse -Force
    exit 1
}

if (-not $DOWNLOAD_URL) {
    Write-Error "Failed to find rtop-windows.exe in the latest release of $REPO"
    Remove-Item -Path $TEMP_DIR -Recurse -Force
    exit 1
}

# Download the binary
Write-Host "Downloading rtop-windows.exe from $REPO latest release..."
try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile "$TEMP_DIR\$BINARY_NAME"
} catch {
    Write-Error "Failed to download rtop binary: $_"
    Remove-Item -Path $TEMP_DIR -Recurse -Force
    exit 1
}

# Move the binary to the installation directory
Write-Host "Installing rtop to $INSTALL_DIR..."
Move-Item -Path "$TEMP_DIR\$BINARY_NAME" -Destination "$INSTALL_DIR\$BINARY_NAME" -Force
if (-not $?) {
    Write-Error "Failed to install rtop binary"
    Remove-Item -Path $TEMP_DIR -Recurse -Force
    exit 1
}

# Clean up
Remove-Item -Path $TEMP_DIR -Recurse -Force

# Check if PATH already includes INSTALL_DIR
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$INSTALL_DIR*") {
    Write-Host "Adding $INSTALL_DIR to User PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$INSTALL_DIR", "User")
}

# Verify installation
if (Get-Command rtop -ErrorAction SilentlyContinue) {
    Write-Host "rtop installed successfully! You can now run it by typing 'rtop'."
} else {
    Write-Host "Installation completed, but rtop is not in PATH. Please restart your terminal or log out and log back in."
}