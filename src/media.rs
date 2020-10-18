extern crate wasm_bindgen;
//use crate::log;
use wasm_bindgen::prelude::*;
use game::Direction;
use wasm_bindgen::JsCast;
//use wasm_bindgen::closure::Closure;
//use std::sync::mpsc::Sender;
use std::sync::mpsc;
pub struct WasmMedia {
    // document: web_sys::Document,
    // canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}


impl WasmMedia {
    pub fn new(height: u32, width: u32, sender: mpsc::Sender<Direction>) -> WasmMedia {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("game-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_height(height);
        canvas.set_width(width);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        add_keyboard_listener(&document, sender).unwrap();
        WasmMedia { /*document, canvas,*/ context }
    }

    pub fn get_context(&self) -> &web_sys::CanvasRenderingContext2d { &self.context }
}

pub fn add_keyboard_listener(doc: &web_sys::Document, sender: mpsc::Sender<Direction>) -> Result<(), JsValue>
{
    let closure = wasm_bindgen::closure::Closure::wrap(
        Box::new(move |event: web_sys::KeyboardEvent| {
            match &event.key()[..] {
                "ArrowUp" => sender.send(Direction::Up).unwrap(),
                "ArrowDown" => sender.send(Direction::Down).unwrap(),
                "ArrowLeft" => sender.send(Direction::Left).unwrap(),
                "ArrowRight" => sender.send(Direction::Right).unwrap(),
                _ => { }
                 }
        }) as Box<dyn FnMut(_)>
    );
    doc.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}


