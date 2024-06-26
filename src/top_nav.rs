use yew::prelude::*;

#[function_component]
pub fn TopNav() -> Html {
  html!{
    <nav class="bg-white border-gray-200 dark:bg-gray-900">
          <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
            <a href="/" class="flex items-center space-x-3 rtl:space-x-reverse">
              <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"Cartesi-Vault"}</span>
            </a>
            <div class="flex md:order-2 space-x-3 md:space-x-0 rtl:space-x-reverse">
            <button type="button" class="font-medium rounded-lg text-sm px-4 py-3 text-center bg-white text-black">
              {"Connect Wallet"}
            </button>
              // {!wallet?.accounts ?
              //   <button onClick={() => connect()} type="button" class="font-medium rounded-lg text-sm px-4 py-3 text-center bg-white text-black">
              //     Connect Wallet
              //   </button>
              //   :
              //   <button onClick={() => disconnect()} type="button" class="font-medium rounded-lg text-sm px-4 py-3 text-center bg-white text-black">
              //     Disconnect Wallet
              //   </button>
              // }
              // <button onClick={() => disconnect()} type="button" class="font-medium rounded-lg text-sm px-4 py-3 text-center bg-white text-black">
              //   Disconnect Wallet
              // </button>
              <button data-collapse-toggle="navbar-cta" type="button" class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-cta" aria-expanded="false">
                <span class="sr-only">{"Open main menu"}</span>
                <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15" />
                </svg>
              </button>
            </div>
          </div>
        </nav>
  }
}
