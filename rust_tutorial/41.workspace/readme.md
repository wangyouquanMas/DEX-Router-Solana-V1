目标：
1. 理解what's the role of 
[workspace]


内容：
1. what's the role of  [workspace]
What [workspace] does
Purpose: Defines a Cargo workspace (a group of packages built together).


Your repo root (/root/raydium-cpi/raydium-amm-v3/Cargo.toml) already declares a workspace:
[workspace]
members = ["programs/*", "client"]

When you run cargo inside client/src/instructions/practice, Cargo walks up to find the nearest [workspace] (the root) and expects your crate to be in workspace.members. Since practice wasn’t a member, Cargo errored and suggested one of these fixes: