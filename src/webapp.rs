#![recursion_limit = "256"]

// use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub author: String,

    #[prop_or_default]
    pub content: String,

    #[prop_or_default]
    pub icon: String,

    #[prop_or_default]
    pub header_name: String,

    #[prop_or_default]
    pub title: String,
}

// Then somewhere else you can use the component inside `html!`
// #[function_component]
// fn HelloWorld() -> Html {
//     html! { <p>{ "Hello world" }</p> }
// }

#[function_component]
fn MessageRoot(props: &Props) -> Html {
    html! { 
        <message>
            <a class="pfp"><img class="pfp" src={ props.icon.clone() }/></a>
            <fill>
                <author><a>{ props.author.clone() }</a></author>
                <content>
                { props.content.clone() }
                </content>
            </fill>
        </message> 
    }
}

#[function_component]
fn Message(props: &Props) -> Html {
    html! { 
        <message>
            <fill>
                <content>
                { props.content.clone() }
                </content>
            </fill>
        </message> 
    }
}

#[function_component]
fn CommandResponse(props: &Props) -> Html {
    html! { 
        <message class="bot-message">
            <a class="pfp"><img class="pfp" src={ props.icon.clone() }/></a>
            <fill>
                <author><a>{ props.author.clone() }</a><span class="bot-tag">{ "BOT" }</span></author>
                <content>
                { props.content.clone() }

                    <p class="text-small"><span>{"Only you can see this messgae. It will disapear when your client is restarted or the tab is closed. "}</span><a>{"Dismiss it now."}</a></p>
                </content>
            </fill>
        </message> 
    }
}

#[function_component]
fn DrawHeader(props: &Props) -> Html {
    html! { 
        <header>
            { props.header_name.clone() }
        </header>
    }
}

#[function_component]
fn TypeMessage(_props: &Props) -> Html {
    html! { 
        <message_area>
            <textarea name="message_box" id="MessageArea" cols="30" rows="1" class="message_box"></textarea>
            // { props.content.clone() }
        </message_area>
    }
}

#[function_component]
fn App() -> Html {
    html! { 
        <app>
            <nav class="hubs">
                <a href="/app/#/home">
                <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" viewBox="0 0 576 576"><path d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"/></svg> // <!--! Font Awesome Pro 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. -->
                </a>

                <hr />

                <a href="/app/#/hub/903019301230"><img src="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" alt="Khaim's Space:tm:" /></a>
            </nav>

            <nav class="channels">
                <DrawHeader header_name="Direct Messages"> </DrawHeader>
            </nav>

            <main>
                <DrawHeader header_name="@khaim0919"> </DrawHeader>

                <display_messages>
                    <MessageRoot author="khaim0919" icon="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" content="You're so gay i'd MARRY you!" />
                    <Message content="You are like, sosososo cuuute!!" />
                    <Message content="I want to smother you in CUDDLES!!!" />
                    <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content="*meows curiously*"/>

                    <MessageRoot author="khaim0919" icon="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" content="You're so gay i'd MARRY you!" />
                    <Message content="You are like, sosososo cuuute!!" />
                    <Message content="I want to smother you in CUDDLES!!!" />
                    <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content="*meows curiously*"/>
                    <MessageRoot author="khaim0919" icon="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" content="You're so gay i'd MARRY you!" />
                    <Message content="You are like, sosososo cuuute!!" />
                    <Message content="I want to smother you in CUDDLES!!!" />
                    <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content="*meows curiously*"/>
                    <MessageRoot author="khaim0919" icon="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" content="You're so gay i'd MARRY you!" />
                    <Message content="You are like, sosososo cuuute!!" />
                    <Message content="I want to smother you in CUDDLES!!!" />
                    <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content="*meows curiously*"/>
                    <MessageRoot author="khaim0919" icon="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" content="You're so gay i'd MARRY you!" />
                    <Message content="You are like, sosososo cuuute!!" />
                    <Message content="I want to smother you in CUDDLES!!!" />
                    <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content="*meows curiously*"/>
                    <MessageRoot author="khaim0919" icon="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" content="You're so gay i'd MARRY you!" />
                    <Message content="You are like, sosososo cuuute!!" />
                    <Message content="I want to smother you in CUDDLES!!!" />
                    <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content="*meows curiously*"/>
                    <MessageRoot author="khaim0919" icon="https://cdn.discordapp.com/avatars/226932670473568256/ceffffd4818c5c53709c3da7e8bbf7ab.png?size=1024" content="You're so gay i'd MARRY you!" />
                    <Message content="You are like, sosososo cuuute!!" />
                    <Message content="I want to smother you in CUDDLES!!!" />
                    <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content="*meows curiously*"/>

                    <CommandResponse author="Azerty" icon="https://pm1.narvii.com/5884/e1682398c02d689122d4306c106b41cfffaa051c_hq.jpg" content="Sucess! Your nickname has been changed to 'KittenNyas!'!" />
                </display_messages>

                <TypeMessage content="Message @khaim0919#8008" />
            </main>

            <members>
            
            </members>

            <your_profile>
                <MessageRoot author="tryoxiss" icon="https://cdn.discordapp.com/avatars/490544642559836163/b1963913984aaaaf07df937e7e2a2116.png?size=1024" content=".has_a_status" />
            </your_profile>

            // <HelloWorld is_loading={ true } />
        </app>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
