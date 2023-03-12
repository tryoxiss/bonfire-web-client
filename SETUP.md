# How to set up and run bonfire

This document outlines and should help you with setting up your own instnace, or helping develop bonfire!

## Installing for Production

No production versions yet, sorry!

Though we will likely include intructions for: 
- Docker Container (if/when we have one)
- CentOS
- Fedora Server
- Ubuntu Server

## Set up for developemnt

On first run: 
`rustup default nightly` (ew, whatever yew won't work without it)
`rustup target add wasm32-unknown-unknown`
`cargo run`

Everyt time to be served (IT WILL HOT RELOAD!): 
`trunk serve`