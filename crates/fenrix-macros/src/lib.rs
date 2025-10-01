extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, parenthesized,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Expr, Ident, ItemFn, LitStr, Path, Result, Token,
};

/// Represents the overall RSX structure. For now, a single root node.
struct RsxInput {
    root: Node,
}

impl Parse for RsxInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RsxInput {
            root: input.parse()?,
        })
    }
}

/// Represents a node in the RSX tree.
enum Node {
    Element(Element),
    Component(ComponentElement),
    Text(LitStr),
    ReactiveText(Expr),
    RenderedNode(Expr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) {
            let fork = input.fork();
            fork.parse::<Token![<]>()?;
            let path: Path = fork.parse()?;
            let first_char = path
                .segments
                .first()
                .unwrap()
                .ident
                .to_string()
                .chars()
                .next()
                .unwrap();

            if first_char.is_ascii_uppercase() {
                return Ok(Node::Component(input.parse()?));
            } else {
                return Ok(Node::Element(input.parse()?));
            }
        } else if input.peek(LitStr) {
            Ok(Node::Text(input.parse()?))
        } else if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            // Convention: if the expression is parenthesized, it's a rendered node.
            // We unwrap the outer parentheses here to avoid a compiler warning.
            if let Expr::Paren(paren_expr) = expr {
                Ok(Node::RenderedNode(*paren_expr.expr))
            } else {
                Ok(Node::ReactiveText(expr))
            }
        } else {
            Err(input.error(
                "Expected an element (`<... />`), a string literal (`\"...\"`), or a rust expression (`{...}`)",
            ))
        }
    }
}

/// Represents an attribute key. Can be a simple identifier or a parenthesized event name.
enum AttrName {
    Standard(Path),
    Event(Ident),
    Binding(Ident),
}

impl Parse for AttrName {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            return Ok(AttrName::Event(content.parse()?));
        }

        let fork = input.fork();
        if let Ok(path) = fork.parse::<Path>() {
            if let Some(ident) = path.get_ident() {
                if ident == "bind" && fork.peek(Token![:]) {
                    // It's a binding. Consume from the real input stream.
                    input.parse::<Ident>()?; // consume "bind"
                    input.parse::<Token![:]>()?; // consume ":"
                    return Ok(AttrName::Binding(input.parse()?));
                }
            }
        }

        // Otherwise, it's a standard attribute. Use `parse_any` to allow keywords,
        // and `.into()` to convert the resulting `Ident` into a `Path`.
        Ok(AttrName::Standard(syn::Ident::parse_any(input)?.into()))
    }
}

/// Represents an attribute value. Can be a literal string or a Rust expression in braces.
enum AttrValue {
    Literal(LitStr),
    Expr(Expr),
}

impl Parse for AttrValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            Ok(AttrValue::Expr(content.parse()?))
        } else {
            Ok(AttrValue::Literal(input.parse()?))
        }
    }
}

impl ToTokens for AttrValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttrValue::Literal(lit) => lit.to_tokens(tokens),
            AttrValue::Expr(expr) => {
                tokens.extend(quote! { &format!("{}", #expr) });
            }
        }
    }
}

/// Represents a single attribute on an element.
struct Attribute {
    name: AttrName,
    value: AttrValue,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Attribute { name, value })
    }
}

/// Represents an HTML element like `<div id="main">...</div>`.
struct Element {
    name: Ident,
    attrs: Vec<Attribute>,
    children: Vec<Node>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;
        let name: Ident = input.parse()?;

        let mut attrs = Vec::new();
        while !input.peek(Token![>]) && !input.peek(Token![/]) {
            attrs.push(input.parse()?);
        }

        if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
            return Ok(Element {
                name,
                attrs,
                children: Vec::new(),
            });
        }
        input.parse::<Token![>]>()?;

        let mut children = Vec::new();
        while !input.peek(Token![<]) || !input.peek2(Token![/]) {
            children.push(input.parse()?);
        }

        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let closing_name: Ident = input.parse()?;
        if closing_name != name {
            let error_message = format!(
                "Mismatched closing tag: expected `{}`, found `{}`",
                name, closing_name
            );
            return Err(input.error(error_message));
        }
        input.parse::<Token![>]>()?;

        Ok(Element {
            name,
            attrs,
            children,
        })
    }
}

/// Represents a component element like `<MyComponent to="/about">Click Me</MyComponent>`.
struct ComponentElement {
    name: Path,
    props: Vec<Attribute>,
    children: Vec<Node>,
}

impl Parse for ComponentElement {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse opening tag: `<ComponentName`
        input.parse::<Token![<]>()?;
        let name: Path = input.parse()?;

        // Parse props
        let mut props = Vec::new();
        while !input.peek(Token![>]) && !input.peek(Token![/]) {
            props.push(input.parse()?);
        }

        // Handle self-closing `/>`
        if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
            return Ok(ComponentElement {
                name,
                props,
                children: Vec::new(),
            });
        }
        input.parse::<Token![>]>()?;

        // Parse children
        let mut children = Vec::new();
        while !input.peek(Token![<]) || !input.peek2(Token![/]) {
            children.push(input.parse()?);
        }

        // Parse closing tag: `</ComponentName>`
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let closing_name: Path = input.parse()?;
        if closing_name != name {
            let error_message = format!(
                "Mismatched closing tag: expected `{}`, found `{}`",
                quote!(#name).to_string(),
                quote!(#closing_name).to_string()
            );
            return Err(input.error(error_message));
        }
        input.parse::<Token![>]>()?;

        Ok(ComponentElement {
            name,
            props,
            children,
        })
    }
}

impl ToTokens for RsxInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.root.to_tokens(tokens);
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Node::Element(el) => el.to_tokens(tokens),
            Node::Component(comp) => comp.to_tokens(tokens),
            Node::Text(text) => {
                tokens.extend(quote! {
                    fenrix_dom::create_text_node(#text).into()
                });
            }
            Node::ReactiveText(expr) => {
                tokens.extend(quote! {
                    fenrix_dom::create_reactive_text_node(move || format!("{}", #expr)).into()
                });
            }
            Node::RenderedNode(expr) => {
                tokens.extend(quote! {
                    {
                        // The effect runs once immediately, so current_node will hold the real node
                        // after the effect is created. We can then return it to be appended to the DOM.
                        let current_node = ::std::rc::Rc::new(::std::cell::RefCell::new(None::<::web_sys::Node>));
                        let effect_current_node = ::std::rc::Rc::clone(&current_node);

                        fenrix_core::create_effect(move || {
                            let new_node: ::web_sys::Node = #expr;

                            if let Some(old_node) = effect_current_node.borrow().as_ref() {
                                if let Some(parent) = old_node.parent_node() {
                                    parent.replace_child(&new_node, old_node).unwrap();
                                }
                            }

                            *effect_current_node.borrow_mut() = Some(new_node);
                        });

                        let borrowed_node = current_node.borrow();
                        borrowed_node.as_ref().unwrap().clone()
                    }
                });
            }
        }
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_name = self.name.to_string();
        let children = &self.children;

        let mut event_handlers = Vec::new();
        let mut standard_attrs = Vec::new();
        let mut bindings = Vec::new();

        for attr in &self.attrs {
            match &attr.name {
                AttrName::Event(_) => event_handlers.push(attr),
                AttrName::Binding(_) => bindings.push(attr),
                AttrName::Standard(_) => standard_attrs.push(attr),
            }
        }

        let set_attributes_code = standard_attrs.iter().map(|attr| {
            if let AttrName::Standard(name) = &attr.name {
                let name_str = quote!(#name).to_string();
                let value = &attr.value;
                quote! { element.set_attribute(#name_str, #value).unwrap(); }
            } else {
                quote! {}
            }
        });

        let add_event_listeners_code = event_handlers.iter().map(|attr| {
            if let AttrName::Event(name) = &attr.name {
                if let AttrValue::Expr(handler) = &attr.value {
                    let event_name = name.to_string();
                    quote! {
                        let closure = ::wasm_bindgen::prelude::Closure::wrap(Box::new(#handler) as Box<dyn FnMut(_)>);
                        element.add_event_listener_with_callback(#event_name, closure.as_ref().unchecked_ref()).unwrap();
                        closure.forget();
                    }
                } else {
                    quote! { compile_error!("Event handler must be a closure in braces."); }
                }
            } else {
                quote! {}
            }
        });

        let add_bindings_code = bindings.iter().map(|attr| {
            if let AttrName::Binding(name) = &attr.name {
                if name == "value" {
                    if let AttrValue::Expr(signal_expr) = &attr.value {
                        quote! {
                            let (getter, setter) = #signal_expr;
                            let _el = element.clone();
                            fenrix_core::create_effect(move || {
                                let el = _el.dyn_ref::<::web_sys::HtmlInputElement>().unwrap();
                                el.set_value(&getter());
                            });
                            let closure = ::wasm_bindgen::prelude::Closure::wrap(Box::new(move |event: ::web_sys::InputEvent| {
                                let target = event.target().unwrap();
                                let el = target.dyn_into::<::web_sys::HtmlInputElement>().unwrap();
                                setter(el.value());
                            }) as Box<dyn FnMut(_)>);
                            element.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).unwrap();
                            closure.forget();
                        }
                    } else {
                        quote! { compile_error!("Binding value must be a signal expression."); }
                    }
                } else {
                    quote! { compile_error!("Only `bind:value` is currently supported."); }
                }
            } else {
                quote! {}
            }
        });

        tokens.extend(quote! {
            {
                let element = fenrix_dom::create_element(#tag_name);
                #(#set_attributes_code)*
                #(#add_event_listeners_code)*
                #(#add_bindings_code)*
                #(
                    let child_node: web_sys::Node = #children;
                    fenrix_dom::append_child(&element, &child_node);
                )*
                element.into()
            }
        });
    }
}

impl ToTokens for ComponentElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_str = quote!(#name).to_string();

        if name_str == "Link" {
            // Special handling for the built-in <Link> component.
            let mut to_prop = None;
            for prop in &self.props {
                if let AttrName::Standard(prop_name) = &prop.name {
                    if quote!(#prop_name).to_string() == "to" {
                        to_prop = Some(&prop.value);
                        break;
                    }
                }
            }

            if let Some(to_value) = to_prop {
                let href_value_tokens = match to_value {
                    AttrValue::Literal(lit) => quote! { #lit },
                    AttrValue::Expr(expr) => quote! { #expr },
                };

                let href = quote! { format!("#{}", #href_value_tokens) };
                let children = &self.children;

                tokens.extend(quote! {
                    {
                        let element = fenrix_dom::create_element("a");
                        element.set_attribute("href", &#href).unwrap();

                        #(
                            let child_node: web_sys::Node = #children;
                            fenrix_dom::append_child(&element, &child_node);
                        )*

                        element.into()
                    }
                });
            } else {
                tokens.extend(quote! {
                    compile_error!("<Link> component requires a 'to' prop.")
                });
            }
        } else {
            // TODO: Implement passing props and children to user-defined components.
            tokens.extend(quote! {
                #name()
            });
        }
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as RsxInput);
    let expanded = quote! {
        {
            #parsed_input
        }
    };
    TokenStream::from(expanded)
}

mod server;

#[proc_macro_attribute]
pub fn server(attr: TokenStream, item: TokenStream) -> TokenStream {
    server::server_macro(attr, item)
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);
    let original_block = func.block;
    let new_block_tokens = quote! {
        {
            fenrix_core::with_component_context(|| #original_block)
        }
    };
    func.block = Box::new(syn::parse2(new_block_tokens).unwrap());
    let expanded = quote! {
        #func
    };
    TokenStream::from(expanded)
}