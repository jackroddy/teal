# teal

A Rust command-line tool: teal explores architectural layout. Only the CLI
exists so far. What teal reads, what it shows, and how, are all still to be
decided.

Work and facts live in foam, not here: `foam ready` for what is open, `foam
memories` for what earlier sessions found. This file holds the rules.

## Scope

The first design goes through a foam issue before it goes into code. Until
then this file carries no claims about layers or output.

## Branches

All working changes go on `dev`. Do not open a feature branch unless I ask for
one.

`main` carries releases and nothing else. Push to it when cutting a release,
and leave it alone the rest of the time.

## Releases

teal follows [semantic versioning](https://semver.org/spec/v2.0.0.html). While
the crate is below 1.0, a breaking change takes the minor number.

Tag every release, annotated, as `vMAJOR.MINOR.PATCH`, on the commit that was
published.

## Formatting

`rustfmt` is the format. Run `cargo fmt` before committing.

## Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Anything that changes what someone running the tool sees goes under
`[Unreleased]` in the commit that changes it.

## Temporary files

Scratch goes in `tmp-claude/` at the root, one directory per task, and never
into git.
