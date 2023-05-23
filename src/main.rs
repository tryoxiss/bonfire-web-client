#![recursion_limit = "256"]

mod webapp;

// Rust LOCAL imports:
//
// mod filename;
// where you want to call it:
// filename::function();
// The function needs to be public.

// If you want this to *just* be a server implenentation and not have a web client avalible by going to your hosting domain, simply exclude the webapp.rs and webapp::render_app() from the code before compiling! Then it will *just* include the backend implementation!
// For testing and almost all use cases, you should *keep* the frontend app. The only reason not to is if you are 100% certain everyone on your instance is using an external client, and you want to save a marginal amount of storage space.
// To make *just* a client, I think just remove the backend file but I haven't tested it.
// We may end up seperating just a client and keeping this as just the server implementation.

fn main() {
    webapp::render_app();
}

// for login code: don't write out own, use an existing crate (obv)
//
// Also allow auth via OAuth 2 through the following services:
//
// - GitHub
// - GitLab
// - Apple
// - GitHub
// - Twitch
// - DeviantArt
// - Discord
// - Google
//
// Yes, a lot of these go against our whole goal and idea, data farms especially.
// But I think its worth it for the ease of onboarding.
// Also allow people to very simply migrate to a non-oauth account via adding a password
// and (verified) email adress, and chosing to migrate thier account.
//
// For discord: We can take thier username and discriminator and ask them if they would like
// to use that for thier profile. And other stuff like bio, just to make it super simple.
//
// Also have a don't store my password option, where we send them a one-time code for each
// login.
