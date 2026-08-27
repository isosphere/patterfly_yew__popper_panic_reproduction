#![recursion_limit = "1024"]

use browser_panic_hook::{CustomBody, IntoPanicHook};
use wasm_bindgen::prelude::*;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[hook]
pub fn use_open<IN>(url: impl Into<String>, target: impl Into<String>) -> Callback<IN, ()>
where
    IN: 'static,
{
    use_callback((url.into(), target.into()), |_, (url, target)| {
        let _ = gloo_utils::window().open_with_url_and_target(url, target);
    })
}

// this does not reproduce the problem :/
#[function_component(Reproduction)]
fn dropdown() -> Html {
    let toaster = use_toaster().unwrap();
    let show_toast = use_callback(toaster, |text, toaster| toaster.toast(text));

    html!{
        <Dropdown text={html!{"Foo"}}>
            <MenuAction onclick={show_toast.reform(|_|"Clicked Foo")}>{"Foo"}</MenuAction>
        </Dropdown>        
    }
}

#[function_component(Application)]
fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Reproduction />
            </ToastViewer>
        </BackdropViewer>
    }
}

fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::set_custom_panic_hook(
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
