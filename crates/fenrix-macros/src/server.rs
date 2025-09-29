use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType};

pub fn server_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // Ensure the function is marked as `async`. Server functions must be async
    // because the client-side version needs to `await` a network request.
    if func.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &func.sig,
            "functions annotated with `#[server]` must be `async`",
        )
        .to_compile_error()
        .into();
    }

    let func_vis = &func.vis;
    let func_sig = &func.sig;
    let func_name = &func.sig.ident;
    let func_body = &func.block;

    // Extract argument identifiers to be serialized on the client.
    let arg_names: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(PatType { pat, .. }) = arg {
                if let Pat::Ident(pat_ident) = &**pat {
                    return Some(&pat_ident.ident);
                }
            }
            None
        })
        .collect();

    let func_name_str = func_name.to_string();
    let api_url = format!("/api/{}", func_name_str);

    let expanded = quote! {
        // Server-side (native) implementation: Keep the original function body.
        #[cfg(not(target_arch = "wasm32"))]
        #func_vis #func_sig {
            #func_body
        }

        // Client-side (Wasm) implementation: Generate a fetch call.
        #[cfg(target_arch = "wasm32")]
        #func_vis #func_sig {
            // This code runs on the client. It serializes the arguments,
            // sends them to the server, and deserializes the response.
            use wasm_bindgen::JsCast;

            // 1. Serialize arguments into a JSON string.
            // The arguments are packed into a tuple for serialization.
            let args_tuple = (#(#arg_names,)*);
            let body_json = serde_json::to_string(&args_tuple)
                .expect("Failed to serialize server function arguments.");

            // 2. Prepare the `fetch` request.
            // 2. Prepare the `fetch` request.
            let mut opts = ::web_sys::RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(::web_sys::RequestMode::Cors);
            let body_js_value = wasm_bindgen::JsValue::from_str(&body_json);
            opts.set_body(&body_js_value);

            let request = ::web_sys::Request::new_with_str_and_init(#api_url, &opts)
                .expect("Failed to create request.");

            request.headers().set("Content-Type", "application/json")
                .expect("Failed to set Content-Type header.");

            // 3. Execute the request and await the response.
            let window = ::web_sys::window().expect("No window found for fetch.");
            let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
                .await
                .expect("Network request for server function failed.");

            let resp: ::web_sys::Response = resp_value
                .dyn_into()
                .expect("Could not cast JsValue to Response.");

            // 4. Check for HTTP errors.
            if !resp.ok() {
                let status = resp.status();
                let status_text = resp.status_text();
                panic!("Server function call failed with status: {} {}", status, status_text);
            }

            // 5. Deserialize the response body.
            let json_promise = resp.json().expect("Failed to get JSON promise from response.");
            let json_value = wasm_bindgen_futures::JsFuture::from(json_promise)
                .await
                .expect("Failed to resolve JSON promise from server function.");

            // Use `serde_wasm_bindgen` to deserialize the JsValue directly.
            serde_wasm_bindgen::from_value(json_value)
                .expect("Failed to deserialize server function response from JSON.")
        }
    };

    TokenStream::from(expanded)
}