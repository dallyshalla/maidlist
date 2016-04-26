# maidlist
display omni properties by balance

### Building from source

##### Ubuntu 14.04, 15.04, 15.10

#### Install Rust Stable

```bash

# install rust stable
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh

# install stable and make it default
sudo multirust update stable
sudo multirust default stable
```

##### OSX with Homebrew

```bash
# install multirust
brew update
brew install multirust

# install stable and make it default
multirust update stable && multirust default stable
```

#### Get balances on MaidSafeCoins

```bash
# download and build maidlist
git clone https://github.com/dallyshalla/maidlist
cd maidlist
cargo run
```