use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console::log_1;
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use super::top_nav::TopNav;

#[derive(Debug, PartialEq)]
pub enum Msg {
  // ModalClose,
  // InputFocused,
  InputChanged { new_value: String },
  // InputKeyPress { key_code: u32 },
}

#[derive(Default, Debug)]
pub struct Landing {
  connected_account: String,
  input_text: String,
  modal_visible: bool,
}

impl Component for Landing {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let connected_account = LocalStorage::get("account").unwrap_or_else(|_| "".to_string());
    Self {
      connected_account,
      input_text: String::default(),
      modal_visible: false,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
        Msg::InputChanged { new_value } => {
          self.input_text = new_value;
        },
    }
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
      html! {
        <div>
        <TopNav />
        <div class="border-gray-200 bg-gray-900 w-full md:max-w-xl mx-auto mt-20 py-10">
          <div class="px-4 pb-4">
            <span class="text-white text-lg md:text-2xl text-center block w-full font-bold">{"Transfer BUSD"}</span>
          </div>
  
          <div class="pt-4 max-w-sm mx-auto">
            <div class="inline-grid w-full">
              <label class="text-white font-medium text-lg pb-1">{"Ethereum Address"}</label>
              <input
                type="text"
                class="h-12 w-full py-1 px-3 border rounded-md focus:outline-none"
                placeholder={"Enter ethereum address"}
                value={self.input_text.clone()}
                oninput={
                  ctx.link().callback(move |ev: InputEvent| {
                    let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
                    let new_value = input.value();
                    log_1(&JsValue::from_str(&new_value));
                    Msg::InputChanged { new_value }
                  })
                }
              />
            </div>
            <div class="inline-grid w-full mt-4">
              <label class="text-white font-medium text-lg pb-1">{"BUSD Amount"}</label>
  
              <div class="flex relative">
                <input type="number" class="h-12 w-full py-1 px-3 border rounded-md focus:outline-none appearance-none" placeholder={"Enter BUSD amount"} />
                <div class="absolute inset-y-0 right-0 flex items-center bg-black text-white hover:cursor-pointer border border-l-2 border-dotted border-black">
                  <span class="font-black text-sm px-3">{"MAX"}</span>
                </div>
              </div>
            </div>
          </div>
  
          <div class="max-w-sm mx-auto mt-12">
            // {!!address ?
            //   <button onClick={transferBUSD} disabled={disabled} class={`bg-blue-500 w-full h-12 text-white font-bold text-xl rounded-md disabled:cursor-not-allowed disabled bg-blue-800 disabled:opacity-50`}>{isLoading ? "Processing" : "Transfer BUSD"}</button>
            //   :
            //   <button onClick={() => open()} class={`bg-blue-500 w-full h-12 text-white font-bold text-xl rounded-md`}>Connect</button>
            // }
          </div>
        </div>
        </div>
      }
  }
}

