# bonfire
(TEMPORARY NAME) repo for decentralized discord-like federated chat! following all common conventions like markdown and naming schemes. Written in pure rust! 

(Please note: Badges are updated manually)

[Code Style: Beautiful](repo-style/badges/code-style.png) [Build: 0.1.0](repo-style/badges/version.png) [Stability: Great](repo-style/badges/stability.png) [Code Health: Good](repo-style/badges/code-health.png) [Security: Terrible](repo-style/badges/security.png) [Contibuters: 2](repo-style/badges/contributers.png) [Documentation: Poor](repo-style/badges/documentation.png)


Versioning is `major:minor:patch::build_status`. Breaking changes may occur, but should be avoided as much as possible. 

## Why not XMPP?

XMPP is great: so great in fact, we want to make Bonfire interoperable with XMPP to the extent possible. However, it does not support the hubs and channels format that we want to use. So, unfortuntely, we need to make our own spec. We will still use XMPP for direct messages, or at least make them interoperable with it, but we can't use it for the bulk of Bonfire.

## Community Team

Tryo - admin, project manager, coordinator, art, etc
Khaim - lead programmer

## For Testing

`trunk serve`
