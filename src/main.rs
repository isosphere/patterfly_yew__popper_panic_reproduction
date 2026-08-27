use browser_panic_hook::{CustomBody, IntoPanicHook};
use wasm_bindgen::prelude::*;
use patternfly_yew::prelude::*;
use yew::prelude::*;

mod dropdown;

// this does not reproduce the problem :/
#[function_component(Reproduction)]
fn dropdown() -> Html {
    html!{
        <dropdown::Dropdown text={html!{"Foo"}}>
            <MenuAction>{"Foo"}</MenuAction>
        </dropdown::Dropdown>        
    }
}

#[function_component(Application)]
fn app() -> Html {
    html! {
        <Reproduction />
    }
}

fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace)); // removing this does fix the panic
    yew::set_custom_panic_hook( // removing this does not fix the panic  
        CustomBody(Box::new(|details| {
            format!(
                r#"
<div class="pf-v6-l-bullseye">
  <div class="pf-v6-l-bullseye__item">
    <div class="pf-v6-c-alert pf-m-danger" aria-label="Application panicked">
      <div class="pf-v6-c-alert__icon">
        <i class="fas fa-fw fa-exclamation-circle" aria-hidden="true"></i>
      </div>
      <p class="pf-v6-c-alert__title">
        <span class="pf-v6-screen-reader">Panick alert:</span>
        Application panicked
      </p>
      <div class="pf-v6-c-alert__description">
        <p>The application failed critically and cannot recover.</p>
        <p>Reason: <pre>{message}</pre></p>
      </div>
    </div>
  </div>
</div>
"#,
                message = details.message()
            )
        }))
        .into_panic_hook(),
    );    
    yew::Renderer::<Application>::new().render();
    Ok(())
}
