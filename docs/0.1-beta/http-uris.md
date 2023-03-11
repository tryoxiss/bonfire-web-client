# HTTP URIs

This document defines the URIs used by this implementation on the web client. 

The app (`app/`) can be removed, but it is reccommended you keep the `/#/` to prevent the page from refresing every time. It's not needed, but it is reccommended. and instead mounted directly to the index document. In fact: This is common practice.

## Group URIs

https://sub.domain.tld/#app/group-part#channel-part

The Group Part represents the group, it is its group custom URI if they have one, or its GUID if they don't. The group part will be `/@/` if it is app related or DMs and not your group. This can be anything you want, really. 

The channel part is the channels name. An example URI is: https://app.instance.tld/app/the-campsite@instance.tld#rules

If you are on the same instance as the group, or you are only in one group with that ID, the @instance.tld can be removed. 