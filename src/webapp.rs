
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct AppComponent {
    count: i128,
    content: Vec<i32>,
}

impl Component for AppComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            count: 0,
            content: vec![1]        
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 10;
                self.content.push(1);
                true // re-render component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! { 
<app class="ctp-mocha">

    <div class="absolute-pane">
        <ContextMenu />
    </div>

    <nav role="groups">
        <ul>
            <li><a href="#1" style="background-image: url(https://picsum.photos/id/237/200/300);"></a></li>
            <li><a href="#2" style="background-image: url(https://picsum.photos/id/217/200/300);"></a></li>
        </ul>
        <hr />
        <ul>
            <li><a href="#1" style="background-image: url(https://picsum.photos/id/237/200/300);"></a></li>
            <li><a href="#2" style="background-image: url(https://picsum.photos/id/217/200/300);"></a></li>
            <li><a href="#3" style="background-image: url(https://picsum.photos/id/237/1920/1080);"></a></li>
            <li><a href="#4" style="background-image: url(https://picsum.photos/id/291/200/300);"></a></li>
            <li><a href="#5" style="background-image: url(https://picsum.photos/id/221/200/300);"></a></li>
            <li><a href="#6" style="background-image: url(https://picsum.photos/id/237/200/300);"></a></li>
            <li><a href="#7" style="background-image: url(https://picsum.photos/id/297/200/300);"></a></li>
            <li><a href="#8" style="background-image: url(https://picsum.photos/id/2197/200/300);"></a></li>
        </ul>
    </nav>
    <nav role="channels"></nav>

    <main>
        <header class="med-padding">
            <h1><span class="muted">{"@"}</span><span>{"doggo"}</span><span class="muted">{"@instance.tld"}</span></h1>
            <vr/>
            <span style="margin-left: auto;"><button><FaPushpinIcon /></button></span>
        </header>

        <ul>
            <MessageRoot />
        
            {
            
            for self.content.iter().map(|x| {
                html! {
                    <MessageRoot />
                }
            })
                
            }
            
        </ul>

        <div class="message-area">
            <button class="round-button"><FaPlusIcon /></button>

            <div 
                class="expandable-textarea"
                role="textbox"
                contenteditable="true"
                placeholder="send a message">
            </div>

            <button class="round-button" onclick={link.callback(|_| Msg::AddOne)}><FaSendIcon /></button>
        </div>

    </main>

    <div role="profile"></div>
</app>
        }
    }
}


// use yew::{function_component, html, Html, Properties};

// #[derive(Properties, PartialEq)]
// pub struct Props {
//     #[prop_or_default]
//     pub author: String,

//     #[prop_or_default]
//     pub content: String,

//     #[prop_or_default]
//     pub icon: String,

//     #[prop_or_default]
//     pub header_name: String,

//     #[prop_or_default]
//     pub title: String,
// }

#[derive(Properties, PartialEq)]
#[derive(Default)]
pub struct AppStruct {
    theme: String,
}

// props: &Props

// Then somewhere else you can use the component inside `html!`
// #[function_component]
// fn HelloWorld() -> Html {
//     html! { <p>{ "Hello world" }</p> }
// }

//# Everything prefixed with Fa is from font awesome Free
// They provide this licence disclaimer. 
// This will also be featured in the credits popup.
// Font Awesome Free 6.3.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2023 Fonticons, Inc. 
// https://fontawesome.com/search?m=free&o=r
#[function_component]
fn FaPlusIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M240 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H176V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H384c17.7 0 32-14.3 32-32s-14.3-32-32-32H240V80z"></path></svg> }
} 
#[function_component]
fn FaSendIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M498.1 5.6c10.1 7 15.4 19.1 13.5 31.2l-64 416c-1.5 9.7-7.4 18.2-16 23s-18.9 5.4-28 1.6L284 427.7l-68.5 74.1c-8.9 9.7-22.9 12.9-35.2 8.1S160 493.2 160 480V396.4c0-4 1.5-7.8 4.2-10.7L331.8 202.8c5.8-6.3 5.6-16-.4-22s-15.7-6.4-22-.7L106 360.8 17.7 316.6C7.1 311.3 .3 300.7 0 288.9s5.9-22.8 16.1-28.7l448-256c10.7-6.1 23.9-5.5 34 1.4z"/></svg> }
} 
#[function_component]
fn FaUserIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M224 256A128 128 0 1 0 224 0a128 128 0 1 0 0 256zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z"/></svg> }
} 
#[function_component]
fn FaSearchIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/></svg> }
} 
#[function_component]
fn FaPushpinIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M32 32C32 14.3 46.3 0 64 0H320c17.7 0 32 14.3 32 32s-14.3 32-32 32H290.5l11.4 148.2c36.7 19.9 65.7 53.2 79.5 94.7l1 3c3.3 9.8 1.6 20.5-4.4 28.8s-15.7 13.3-26 13.3H32c-10.3 0-19.9-4.9-26-13.3s-7.7-19.1-4.4-28.8l1-3c13.8-41.5 42.8-74.8 79.5-94.7L93.5 64H64C46.3 64 32 49.7 32 32zM160 384h64v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V384z"/></svg> }
} 
#[function_component]
fn FaInvisibleIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M40.1 467.1l-11.2 9c-3.2 2.5-7.1 3.9-11.1 3.9C8 480 0 472 0 462.2V192C0 86 86 0 192 0S384 86 384 192V462.2c0 9.8-8 17.8-17.8 17.8c-4 0-7.9-1.4-11.1-3.9l-11.2-9c-13.4-10.7-32.8-9-44.1 3.9L269.3 506c-3.3 3.8-8.2 6-13.3 6s-9.9-2.2-13.3-6l-26.6-30.5c-12.7-14.6-35.4-14.6-48.2 0L141.3 506c-3.3 3.8-8.2 6-13.3 6s-9.9-2.2-13.3-6L84.2 471c-11.3-12.9-30.7-14.6-44.1-3.9zM160 192a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm96 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"/></svg> }
}
#[function_component]
fn FaHomeIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"/></svg> }
}
#[function_component]
fn FaHtmlIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M392.8 1.2c-17-4.9-34.7 5-39.6 22l-128 448c-4.9 17 5 34.7 22 39.6s34.7-5 39.6-22l128-448c4.9-17-5-34.7-22-39.6zm80.6 120.1c-12.5 12.5-12.5 32.8 0 45.3L562.7 256l-89.4 89.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l112-112c12.5-12.5 12.5-32.8 0-45.3l-112-112c-12.5-12.5-32.8-12.5-45.3 0zm-306.7 0c-12.5-12.5-32.8-12.5-45.3 0l-112 112c-12.5 12.5-12.5 32.8 0 45.3l112 112c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L77.3 256l89.4-89.4c12.5-12.5 12.5-32.8 0-45.3z"/></svg> }
}
#[function_component]
fn FaChevronRight() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M310.6 233.4c12.5 12.5 12.5 32.8 0 45.3l-192 192c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L242.7 256 73.4 86.6c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l192 192z"/></svg> }
}
#[function_component]
fn FaFingerprint() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M454.4 93c7.3 8.8 6.1 21.6-2 29.7c-10.6 10.6-28.2 8.6-38-2.7C376.2 75.9 319.9 48 257 48C142.1 48 49 141.1 49 256v24.9c0 6.1-.2 12.2-.6 18.3C47.7 311.2 37.6 320 25.6 320C11.1 320 .1 307 .7 292.5c.2-3.9 .3-7.7 .3-11.6V256C1 114.6 115.6 0 257 0c79.4 0 150.4 36.2 197.4 93zm19.3 89.6c13.1-6.5 29-.2 32.4 14.1c4.5 19.1 6.9 39 6.9 59.4v24.9c0 5.4-.1 10.9-.2 16.3C512.6 310 502 320 489.2 320c-13.7 0-24.6-11.5-24.4-25.3c.1-4.6 .1-9.2 .1-13.8V256c0-15.1-1.6-29.8-4.6-43.9c-2.5-11.8 2.5-24.2 13.3-29.6zM257 80c97.2 0 176 78.8 176 176v24.9c0 27.7-1.7 55.3-5 82.7c-1.4 11.7-11.5 20.3-23.3 20.3c-14.7 0-25.9-13.2-24.2-27.8c3-24.9 4.4-50.1 4.4-75.3V256c0-70.7-57.3-128-128-128c-11.6 0-22.8 1.5-33.4 4.4c-10.6 2.9-22.3 .4-29.4-7.9c-10.4-12.1-6.9-30.9 8.3-35.9C219.6 83 238 80 257 80zM151.7 148.7c8.2 9.6 7.5 23.8 .2 34.2C137.5 203.6 129 228.8 129 256v24.9c0 28.9-3.3 57.7-9.7 85.8C116.9 377 107.6 384 97.1 384c-15.9 0-27.3-15.6-23.9-31.1c5.2-23.6 7.8-47.7 7.8-71.9V256c0-40.6 13.7-78 36.8-107.7c8.5-11 24.8-10.2 33.9 .4zM257 160c53 0 96 43 96 96v24.9c0 39.7-3.9 79.3-11.6 118.1c-2 10-10.8 17-21 17c-14.2 0-24.5-13.3-21.8-27.2c6.9-35.5 10.4-71.6 10.4-107.9V256c0-28.7-23.3-52-52-52s-52 23.3-52 52v24.9c0 40.5-5.3 80.7-15.9 119.7c-2.5 9.2-10.9 15.4-20.4 15.4c-14.8 0-25.3-14.6-21.5-29c9.1-34.6 13.8-70.2 13.8-106.1V256c0-53 43-96 96-96zm24 96v24.9c0 65.8-12.1 131-35.7 192.4l-5.9 15.3c-4.8 12.4-18.6 18.5-31 13.8s-18.5-18.6-13.8-31l5.9-15.3C222 400.2 233 340.8 233 280.9V256c0-13.3 10.7-24 24-24s24 10.7 24 24z"/></svg> }
}
#[function_component]
fn FaReply() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M205 34.8c11.5 5.1 19 16.6 19 29.2v64H336c97.2 0 176 78.8 176 176c0 113.3-81.5 163.9-100.2 174.1c-2.5 1.4-5.3 1.9-8.1 1.9c-10.9 0-19.7-8.9-19.7-19.7c0-7.5 4.3-14.4 9.8-19.5c9.4-8.8 22.2-26.4 22.2-56.7c0-53-43-96-96-96H224v64c0 12.6-7.4 24.1-19 29.2s-25 3-34.4-5.4l-160-144C3.9 225.7 0 217.1 0 208s3.9-17.7 10.6-23.8l160-144c9.4-8.5 22.9-10.6 34.4-5.4z"/></svg> }
}
#[function_component]
fn FaBookmarkFilled() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M0 48V487.7C0 501.1 10.9 512 24.3 512c5 0 9.9-1.5 14-4.4L192 400 345.7 507.6c4.1 2.9 9 4.4 14 4.4c13.4 0 24.3-10.9 24.3-24.3V48c0-26.5-21.5-48-48-48H48C21.5 0 0 21.5 0 48z"/></svg> }
}
#[function_component]
fn FaBookmarkEmpty() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M0 48C0 21.5 21.5 0 48 0l0 48V441.4l130.1-92.9c8.3-6 19.6-6 27.9 0L336 441.4V48H48V0H336c26.5 0 48 21.5 48 48V488c0 9-5 17.2-13 21.3s-17.6 3.4-24.9-1.8L192 397.5 37.9 507.5c-7.3 5.2-16.9 5.9-24.9 1.8S0 497 0 488V48z"/></svg> }
}
#[function_component]
fn FaEnvelopeClosed() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48H48zM0 176V384c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V176L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/></svg> }
}
#[function_component]
fn FaEnvelopeOpen() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M215.4 96H144 107.8 96v8.8V144v40.4 89L.2 202.5c1.6-18.1 10.9-34.9 25.7-45.8L48 140.3V96c0-26.5 21.5-48 48-48h76.6l49.9-36.9C232.2 3.9 243.9 0 256 0s23.8 3.9 33.5 11L339.4 48H416c26.5 0 48 21.5 48 48v44.3l22.1 16.4c14.8 10.9 24.1 27.7 25.7 45.8L416 273.4v-89V144 104.8 96H404.2 368 296.6 215.4zM0 448V242.1L217.6 403.3c11.1 8.2 24.6 12.7 38.4 12.7s27.3-4.4 38.4-12.7L512 242.1V448v0c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64v0zM176 160H336c8.8 0 16 7.2 16 16s-7.2 16-16 16H176c-8.8 0-16-7.2-16-16s7.2-16 16-16zm0 64H336c8.8 0 16 7.2 16 16s-7.2 16-16 16H176c-8.8 0-16-7.2-16-16s7.2-16 16-16z"/></svg> }
}
#[function_component]
fn FaCopy() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M0 448c0 35.3 28.7 64 64 64H288c35.3 0 64-28.7 64-64V384H224c-53 0-96-43-96-96V160H64c-35.3 0-64 28.7-64 64V448zm224-96H448c35.3 0 64-28.7 64-64V64c0-35.3-28.7-64-64-64H224c-35.3 0-64 28.7-64 64V288c0 35.3 28.7 64 64 64z"/></svg> }
}
#[function_component]
fn FaLink() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M562.8 267.7c56.5-56.5 56.5-148 0-204.5c-50-50-128.8-56.5-186.3-15.4l-1.6 1.1c-14.4 10.3-17.7 30.3-7.4 44.6s30.3 17.7 44.6 7.4l1.6-1.1c32.1-22.9 76-19.3 103.8 8.6c31.5 31.5 31.5 82.5 0 114L405.3 334.8c-31.5 31.5-82.5 31.5-114 0c-27.9-27.9-31.5-71.8-8.6-103.8l1.1-1.6c10.3-14.4 6.9-34.4-7.4-44.6s-34.4-6.9-44.6 7.4l-1.1 1.6C189.5 251.2 196 330 246 380c56.5 56.5 148 56.5 204.5 0L562.8 267.7zM43.2 244.3c-56.5 56.5-56.5 148 0 204.5c50 50 128.8 56.5 186.3 15.4l1.6-1.1c14.4-10.3 17.7-30.3 7.4-44.6s-30.3-17.7-44.6-7.4l-1.6 1.1c-32.1 22.9-76 19.3-103.8-8.6C57 372 57 321 88.5 289.5L200.7 177.2c31.5-31.5 82.5-31.5 114 0c27.9 27.9 31.5 71.8 8.6 103.9l-1.1 1.6c-10.3 14.4-6.9 34.4 7.4 44.6s34.4 6.9 44.6-7.4l1.1-1.6C416.5 260.8 410 182 360 132c-56.5-56.5-148-56.5-204.5 0L43.2 244.3z"/></svg> }
}
#[function_component]
fn FaTrash() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M135.2 17.7C140.6 6.8 151.7 0 163.8 0H284.2c12.1 0 23.2 6.8 28.6 17.7L320 32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32h96l7.2-14.3zM32 128H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V128zm96 64c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16z"/></svg> }
}
#[function_component]
fn FaFlag() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M64 32C64 14.3 49.7 0 32 0S0 14.3 0 32V64 368 480c0 17.7 14.3 32 32 32s32-14.3 32-32V352l64.3-16.1c41.1-10.3 84.6-5.5 122.5 13.4c44.2 22.1 95.5 24.8 141.7 7.4l34.7-13c12.5-4.7 20.8-16.6 20.8-30V66.1c0-23-24.2-38-44.8-27.7l-9.6 4.8c-46.3 23.2-100.8 23.2-147.1 0c-35.1-17.6-75.4-22-113.5-12.5L64 48V32z"/></svg> }
}
// #[function_component]
// fn FaTrash() -> Html {
//     html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M135.2 17.7C140.6 6.8 151.7 0 163.8 0H284.2c12.1 0 23.2 6.8 28.6 17.7L320 32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32h96l7.2-14.3zM32 128H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V128zm96 64c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16z"/></svg> }
// }
// #[function_component]
// fn FaTrash() -> Html {
//     html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M135.2 17.7C140.6 6.8 151.7 0 163.8 0H284.2c12.1 0 23.2 6.8 28.6 17.7L320 32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32h96l7.2-14.3zM32 128H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V128zm96 64c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16z"/></svg> }
// }



#[function_component]
fn MessageRoot() -> Html {
    html! {
    <li>
        <div style="background-image: url(https://picsum.photos/id/237/200/300);" class="pfp" />

        <div class="content">
            <header>
                <text class="author">{"doggo"}</text>
                <time>{"07 Feb 2023 at 23:09"}</time>
            </header>
            <div class="content">
                <text>{"lorem"}</text>
            </div>
        </div>
    </li>
    }
}

// pub struct Props { 
//     public: String,
//     main: String,
//     args: String,
// }

#[function_component]
fn Message() -> Html {
    html! {
    <li>
    <time>{"23:09"}</time>
        <div class="content">
        <text>{"lorem"}</text>
        </div>
    </li>
    }
}

// #[function_component]
// fn RoundButton() -> Html {
//     html! {
//     <button class="round-button">
//         <FaPushpinIcon />
//     </button>
//     }
// }

#[function_component]
fn ContextMenu() -> Html {
    html! {
    <div id="contextMenu" class="context-menu" style="display: none"> 
        <ul class="menu"> 
            <ContextMenuMessage />

            <ul class="menu dev-mode"> 
                <li class=""><button href="#"><label for="name_muted_checkbox">{"Mute Channel"}</label> <input id="name_muted_checkbox" type="checkbox" /></button></li> 
                <li class="trash"><button href="#">{"Copy Text"} <FaCopy /></button></li> 
                <li class="trash"><button href="#">{"Copy Link"} <FaLink /></button></li> 
                <li class="danger"><button href="#">{"Delete Message"} <FaTrash /></button></li> 
            </ul> 

            <hr />
            <ul class="menu dev-mode"> 
                <li class="trash"><button href="#">{"Copy GUID"} <FaFingerprint /></button></li> 
                <li class="trash"><button href="#">{"Inspect Element"} <FaHtmlIcon /></button></li> 
            </ul> 
        </ul> 
    </div> 
    }
}



#[function_component]
fn ContextMenuMessage() -> Html {
    html! {
        <ul class="menu"> 
            <li class="trash"><button href="#">{"Add Reaction"} <FaChevronRight /></button></li> 
            <li class="trash"><button href="#">{"Bookmark Message"} <FaBookmarkEmpty /></button></li> 
            <li class="trash"><button href="#">{"Reply"} <FaReply /></button></li> 
        </ul> 
    }
}

#[function_component]
fn ContextMenuManageMessage() -> Html {
    html! {
        <ul class="menu"> 
            <li class="trash"><button href="#">{"Inspect Element"}</button></li> 
        </ul> 
    }
}

#[function_component]
fn ContextMenuChannel() -> Html {
    html! {
        <ul class="menu"> 
            <li class="trash"><button href="#">{"Inspect Element"}</button></li> 
        </ul> 
    }
}

#[function_component]
fn ContextMenuManageChannel() -> Html {
    html! {
        <ul class="menu"> 
            <li class="trash"><button href="#">{"Inspect Element"}</button></li> 
        </ul> 
    }
}


pub fn render_app() {
    // yew::Renderer::<App>::new().render();
    yew::Renderer::<AppComponent>::new().render();
}