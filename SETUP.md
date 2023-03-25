# How to set up and run bonfire

This document outlines and should help you with setting up your own instnace, or helping develop bonfire!

## Installing for Production

No production versions yet, sorry!

Though we will likely include intructions for: 
- CentOS
- Fedora Server
- Ubuntu Server

## Set up for developemnt

We now have a nice shell script for setting up your dev enviornment. You will want to run it the first time you open the project, after cloning. Other than that just run `trunk serve` in the directory to serve the page.

`rustup default nightly` (ew, whatever yew won't work without it)
`rustup target add wasm32-unknown-unknown`
`cargo run`

Everyt time to be served (IT WILL HOT RELOAD!): 
`trunk serve`
