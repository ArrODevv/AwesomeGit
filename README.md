# Awesome Git
A git CLI clone using the git API. Designed to look better and have extra functions.
written in [Rust](https://rust-lang.org).

## Compiling from source
Make sure you have rust installed (easiest way is to use [rustup](https://rustup.rs/)).
First clone this repo (`git clone https://github.com/ArrODevv/AwesomeGit.git`) or download sources from a [release](https://github.com/ArrODevv/AwesomeGit/release).
Then run:
```bash
cargo build
```
Now you should have a `awesome_git.exe` or `awesome_git` file in [target/debug/](target/debug/)...

## Installing with installer
Coming soon...

## Installing from binary release
- Place the executable file(s) in the desired location.
- Add the folder containing said files to your PATH variable. [how do I do that?](#how-to-add-a-folder-to-path)

### How to add a folder to PATH

#### On Linux 
You can use any editor, here for example [nano](https://nano-editor.org/):
```bash
cd ~
nano .profile #or: "nano .bashrc", it depends on your distro and/or configuration.
```
Now in your text editor edit the file like this:
```bash_profile
export PATH="$PATH:your/installation/folder"
```
Save and close the modified file (for nano: press Ctrl+O > Enter > Ctrl+X)
And then you're done!

#### On Windows:
You can use any editor.
- Open Settings
- Navigate to:
  - System > Info > Advanced System Settings >
  - Environment Variables
- In System Variables click double on Path
- Click on the button labeled "New"
- Enter the path to the folder containing your .exe (e.g.: `C:\Users\ArrODevv\Program Files\ArrODevv\AwesomeGit`)
- Press Enter
- Close the Windows including settings
And then you're done!

#### On MacOS
Coming soon...
