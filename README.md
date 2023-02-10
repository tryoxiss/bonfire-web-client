# bonfire
(TEMPORARY NAME) repo for decentralized discord-like federated chat! following all common conventions like markdown and naming schemes. Written in pure rust! 

<div style="display: flex; justify-content: center;">
<a href="CODESTYLE.md" style="display: inline-block">
<div style="height: 2rem; margin: 1rem 0.25rem;">
    <div style="color: rgb(30, 30, 46); background-color: rgb(180, 190, 254);padding: 0.5rem; display: inline-block">Code Style</div><div style="color: rgb(180, 190, 254);background-color: rgb(30, 30, 46);padding: 0.5rem; display: inline-block">Beautiful</div>
</div>
</a>
<div style="height: 2rem; margin: 1rem 0.25rem;" style="display: inline-block">
    <div style="color: rgb(30, 30, 46); background-color: rgb(135, 176, 249);padding: 0.5rem; display: inline-block">Build</div><div style="color: rgb(135, 176, 249);background-color: rgb(30, 30, 46);padding: 0.5rem; display: inline-block">0.1.0</div>
</div>
<div style="height: 2rem; margin: 1rem 0.25rem;" style="display: inline-block">
    <div style="color: rgb(30, 30, 46); background-color: rgb(148, 226, 213);padding: 0.5rem; display: inline-block">Stability</div><div style="color: rgb(148, 226, 213);background-color: rgb(30, 30, 46);padding: 0.5rem; display: inline-block">Great</div>
</div>
<div style="height: 2rem; margin: 1rem 0.25rem;" style="display: inline-block">
    <div style="color: rgb(30, 30, 46); background-color: rgb(243, 139, 168);padding: 0.5rem; display: inline-block">Security</div><div style="color: rgb(243, 139, 168);background-color: rgb(30, 30, 46);padding: 0.5rem; display: inline-block">None (Terrible)</div>
</div>
<div style="height: 2rem; margin: 1rem 0.25rem;" style="display: inline-block">
    <div style="color: rgb(30, 30, 46); background-color: rgb(166, 227, 161);padding: 0.5rem; display: inline-block">Code Health</div><div style="color: rgb(166, 227, 161);background-color: rgb(30, 30, 46);padding: 0.5rem; display: inline-block">Good</div>
</div>
<div style="height: 2rem; margin: 1rem 0.25rem;" style="display: inline-block">
    <div style="color: rgb(30, 30, 46); background-color: rgb(135, 176, 249);padding: 0.5rem; display: inline-block">Contributers</div><div style="color: rgb(135, 176, 249);background-color: rgb(30, 30, 46);padding: 0.5rem; display: inline-block">2</div>
</div>
</div>

Code health = Cleanliness, speed, etc. 

Levels for things that use that: 
- Amazing! (6/6) `rgb(137, 220, 235)`
- Great (5/6) `rgb(148, 226, 213)`
- Good (4/6) `rgb(166, 227, 161)`
- Mediocre (3/6) `rgb(249, 226, 175)`
- Poor (2/6) `rgb(250, 179, 135)`
- Terrible (1/6) `rgb(243, 139, 168)`
- Needs < BLANK > Immeditely (0/6)

Versioning is `major:minor:patch::build_status`. Breaking changes may occur, but should be avoided as much as possible. 

## Why not XMPP?

XMPP is great: so great in fact, we want to make Bonfire interoperable with XMPP to the extent possible. However, it does not support the hubs and channels format that we want to use. So, unfortuntely, we need to make our own spec. We will still use XMPP for direct messages, or at least make them interoperable with it, but we can't use it for the bulk of Bonfire.

## Community Team

Tryo - admin, project manager, coordinator, art, etc
Khaim - lead programmer

## For Testing

`trunk serve`
