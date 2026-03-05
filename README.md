# drubi
As the description says, that's a simple implementation of "Drunken Bishop" algorithm. The Drunken Bishop algorithm is a visual fingerprinting algorithm originally implemented by OpenSSH for visualizing key fingerprints.
## why?
because i liked it
## Building?
clone the git repository and cd into it:
```bash
git clone https://github.com/3-days-debugging-wasted/drubi.git
cd drubi
```
then, compile
```bash
cargo build --release
```
and copy the binary to /usr/local/bin!
```bash
sudo cp -v target/release/drubi /usr/local/bin
```
## how to use?
you could either put input in arguments:
```bash
drubi hello, world!
```
or 
```bash
drubi "hello, world!"
```
you also can pipe it through stdout:
```bash
echo -n "hello, world!" | drubi
```
