# bonfire
(TEMPORARY NAME) repo for decentralized discord-like chat federated using ActivityPub, following all common conventions like markdown and naming schemes. Written in rust! 

EDITORS NOTE: The licence will probably be one of the appache's or CC BY-NC-SA aligned (can't use it for code, not well at least). Not decided yet, so all rights reserved for now, but it will be better later. 

```
Bonfire Software Stack (WIP): 

Server-Side software: 
.------------------------------------------------------------------.
| bonfire-server                                                   |
'------------------------------------------------------------------'
.--------------. .----------. .------. .-------. .-----------------.
| Bonfire Spec | | maria db | | Rust | | HTTPS | | Web Client      | 
'--------------' '----------' '------' '-------' '-----------------'

Client-Side software: 
.------------------------------------------------------------------.
| (offical) bonfire-client (will be native eventually but not now) |
'------------------------------------------------------------------'
.--------------. .--------------------------------. .--------------.
| Bonfire Spec | | Yew                            | | Electron     |
|              | '--------------------------------' '--------------'
|              | .------. .------. .------. .-----. .--------------.
|              | | Rust | | HTML | | WASM | | CSS | | chromium     |
'--------------' '------' '------' '------' '-----' '--------------'

Bonfire Standards Stack: 

Specification: 
.-------------------------------------------------------------------.
| Bonfire Specification                                             |
'-------------------------------------------------------------------'
.----------------------. .--------------------. .----------. .------.
| Federation Standards | | Username Standards | | Markdown | | SQL  |
'----------------------' '--------------------' '----------' '------'

Fragments Mentioned: 
.-------------------------------------------------------------------.
| Federation Standards                                              |
'-------------------------------------------------------------------'
.------------------------------------------. 
| diaspora* federation protocol            | 
'------------------------------------------' 

.-------------------------------------------------------------------.
| Username Standards (@username#0001+subname@instance.tld           |
'-------------------------------------------------------------------'
.-----------------------------. .------------. .--------------------.
| @user@instance.tld (ActPub) | | sub-emails | | @username#0001     |
'-----------------------------' '------------' '--------------------'
```
