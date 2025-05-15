# NEBULA CHAT

nebula chat is a spin over irc, using the iroh rust library.<br>
this chat relies on a p2p network. One of the biggest features are:
* E2E encryption already provided by iroh
* "server" and "client" provided in the same binary.

# Build from source
Building from source using nix
```bash
git clone https://github.com/System-Nebula/nebula-chat
nix build # this will leave the resulting binary in a result directory
```
Installing the binary in your computer using a flake:

```nix
# flake.nix
{
    description = "";
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        nebula-chat.url = "github:System-Nebula/nebula-chat?ref=main";
    };
    outputs = {
        # your other outputs
        self,
        nebula-chat,
        # your other outputs
    };
}
# inside of your home-manager config or configuration.nix
# specify the resulting package
nebula-chat.packages.default.<system-arch>
```

Building without using nix:
```bash
cargo build --release
```
# How to use it?

```bash
nebula-chat --name <name> join <token> # this allows you to join a chatroom
nebula-chat --name <name> open # this allows you to create a chatroom
```
