# rtop

A lightweight, terminal-based system monitoring tool built in Rust, inspired by top and btop.

## Installation Guide
This guide explains how to install `rtop` on Mac, Linux, and Windows.  
Follow the instructions below for your platform to install `rtop`. Each command downloads and executes an installation script from the latest release of the [gohyuhan/rtop](https://github.com/gohyuhan/rtop) repository.  

### Mac and Linux

Run the following command in a terminal:  

```bash
curl -L https://github.com/gohyuhan/rtop/releases/latest/download/install.sh | bash
```

**Note**: You may need `sudo` permissions to install to `/usr/local/bin`.  

### Windows

Run the following command in PowerShell:  

```powershell
Invoke-WebRequest -Uri https://github.com/gohyuhan/rtop/releases/latest/download/install.ps1 -OutFile "$env:TEMP\install.ps1"; Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy Bypass -Force; & "$env:TEMP\install.ps1"
```

**Note**: Open a new PowerShell or Command Prompt after installation to use `rtop`.  