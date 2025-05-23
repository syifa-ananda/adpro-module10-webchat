use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gradient-to-br from-black via-gray-900 to-black min-h-screen w-screen flex items-center justify-center">
            <div class="bg-gradient-to-br from-gray-950 via-black to-gray-900 p-8 rounded-xl shadow-2xl border border-indigo-700 w-full max-w-sm mx-4 text-center space-y-6">
                <div class="flex justify-center items-center space-x-4 text-yellow-400 text-4xl">
                    <h1 class="font-bold tracking-widest text-white text-3xl">{"YEWCHAT"}</h1>
                </div>
                <p class="text-gray-400 text-sm italic">{"Making new friends is just a click away."}</p>
                <form class="flex rounded-lg overflow-hidden shadow-inner border border-indigo-600 bg-black bg-opacity-60 backdrop-blur-sm">
                    <input 
                        {oninput} 
                        class="w-full px-4 py-3 text-white placeholder-gray-500 bg-transparent focus:outline-none focus:ring-2 focus:ring-indigo-500" 
                        placeholder="Username"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button 
                            {onclick} 
                            disabled={username.len() < 1}
                            class="px-6 py-3 text-sm font-bold uppercase text-white bg-indigo-700 hover:bg-indigo-800 transition-colors"
                        >
                            {"🚀 Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
       
}