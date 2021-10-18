# colorful-uname
Another tool you didn't know you want<br>

## Examples:
- Default operation:<br>
![Default](https://github.com/S0raWasTaken/colorful-uname/blob/master/images/default.png)<br>
- "All" operation:<br>
![All](https://github.com/S0raWasTaken/colorful-uname/blob/master/images/all.png)<br>
- Help message:<br>
![Help](https://github.com/S0raWasTaken/colorful-uname/blob/master/images/help.png)<br>

## Building and installation:
> Build process depends on RUST
### Building
```bash
git clone https://github.com/S0raWasTaken/colorful-uname
cd colorful-uname
cargo build --release
```
### Installation
```bash
# I will assume you are on the repo's main folder
sudo mv target/release/unamec /usr/local/bin/unamec
```
### Extra:
You could add an alias to your uname, so that it will always be colorful<br>
This can be done by adding this line to the end of your `~/.bashrc`, `~/.zshrc` or your aliases file

```bash
alias uname=unamec
```
