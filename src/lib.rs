use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(module = "/modules/myChart.js")]
extern "C" {
    type MyChart;

    #[wasm_bindgen(constructor)]
    fn new() -> MyChart;

    #[wasm_bindgen(method)]
    fn draw(this: &MyChart);
}

enum Msg {
    Draw,
}

struct Model {
    chart: MyChart,
    is_clicked: bool
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            chart: MyChart::new(),
            is_clicked: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                if !self.is_clicked {
                    self.chart.draw();
                    self.is_clicked = true;
                    console_log!("self.is_clicked: {}", self.is_clicked);
                } else {
                    console_log!("chart was drawn");
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::Draw)}>{ "draw" }</button>
                <br/>
                <canvas id="myChart" width="400" height="400"></canvas>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<Model>();
}
