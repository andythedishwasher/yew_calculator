use yew::{prelude::*};

enum Msg {
    Add, Subtract, Multiply, Divide, Clear, Equals, Decimal,
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine
}

struct CounterComponent {
    display: String,
    operands: Vec<f64>,
    last_op: Option<char>,
    edit_mode: bool
}

fn str_2_f64 (str:&String) -> f64 {
    str.parse::<f64>().unwrap_or_default()
}

fn equals (comp: &CounterComponent ) -> f64 {
    let display = &comp.display;
    let mut result = 0.0;
    match comp.last_op {
        None => { 
            result = str_2_f64(&display);
        },
        Some('+') => {
            result = comp.operands[1] + comp.operands[0];
        },
        Some('-') => {
            result = comp.operands[1] - comp.operands[0];
        },
        Some('*') => {
            result = comp.operands[1] * comp.operands[0];
        },
        Some('/') => {
            result = comp.operands[1] / comp.operands[0];
        },
        _ => { 
            result = str_2_f64(&display);
        }
    }
    result
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            display: String::from("0"),
            operands: Vec::from([]),
            last_op: None,
            edit_mode: false 
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg {
            Msg::Add => {
                if self.edit_mode == true {
                    self.operands.insert(0, str_2_f64(&self.display));
                    if self.operands.len() > 2 {
                        self.operands.pop();
                    }
                }
                let result = equals(&self);
                self.operands.insert(0, result);
                if self.operands.len() > 2 {
                    self.operands.pop();
                }
                self.last_op = Some('+');
                self.display = 0.to_string();
                self.edit_mode = false;
                true
            },
            Msg::Subtract => {
                if self.edit_mode == true {
                    self.operands.insert(0, str_2_f64(&self.display));
                    if self.operands.len() > 2 {
                        self.operands.pop();
                    }
                }
                let result = equals(&self);
                self.operands.insert(0, result);
                if self.operands.len() > 2 {
                    self.operands.pop();
                }
                self.last_op = Some('-');
                self.display = 0.to_string();
                self.edit_mode = false;
                true                
            },
            Msg::Multiply => {
                if self.edit_mode == true {
                    self.operands.insert(0, str_2_f64(&self.display));
                    if self.operands.len() > 2 {
                        self.operands.pop();
                    }
                }
                let result = equals(&self);
                self.operands.insert(0, result);
                if self.operands.len() > 2 {
                    self.operands.pop();
                }
                self.last_op = Some('*');
                self.display = 0.to_string();
                self.edit_mode = false;
                true
            },
            Msg::Divide => {
                if self.edit_mode == true {
                    self.operands.insert(0, str_2_f64(&self.display));
                    if self.operands.len() > 2 {
                        self.operands.pop();
                    }
                }
                let result = equals(&self);
                self.operands.insert(0, result);
                if self.operands.len() > 2 {
                    self.operands.pop();
                }
                self.last_op = Some('/');
                self.display = 0.to_string();
                self.edit_mode = false;
                true               
            },
            Msg::Clear => {
                self.display = 0.to_string();
                self.operands = Vec::from([]);
                self.last_op = None;
                self.edit_mode = false;
                true
            },
            Msg::Equals => {
                self.operands.insert(0, str_2_f64(&self.display));
                if self.operands.len() > 2 {
                    self.operands.pop();
                }
                let result = equals(&self);
                self.display = result.to_string();
                self.last_op = None;
                self.operands.insert(0, result);
                if self.operands.len() > 2 {
                    self.operands.pop();
                }
                self.edit_mode = false;
                true
            },
            Msg::Decimal => {
                if self.edit_mode == false {
                    self.display = "0.".to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,".").to_string();
                    true
                }
            },
            Msg::Zero => {
                if self.edit_mode == false {
                    self.display = 0.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,0).to_string();
                    true
                }
            },
            Msg::One => {
                if self.edit_mode == false {
                    self.display = 1.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,1).to_string();
                    true
                }
            },
            Msg::Two => {
                if self.edit_mode == false {
                    self.display = 2.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,2).to_string();
                    true
                }
            },
            Msg::Three => {
                if self.edit_mode == false {
                    self.display = 3.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,3).to_string();
                    true
                }
            },
            Msg::Four => {
                if self.edit_mode == false {
                    self.display = 4.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,4).to_string();
                    true
                }
            },
            Msg::Five => {
                if self.edit_mode == false {
                    self.display = 5.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,5).to_string();
                    true
                }
            },
            Msg::Six => {
                if self.edit_mode == false {
                    self.display = 6.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,6).to_string();
                    true
                }
            },
            Msg::Seven => {
                if self.edit_mode == false {
                    self.display = 7.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,7).to_string();
                    true
                }
            },
            Msg::Eight => {
                if self.edit_mode == false {
                    self.display = 8.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,8).to_string();
                    true
                }
            },
            Msg::Nine => {
                if self.edit_mode == false {
                    self.display = 9.to_string();
                    self.edit_mode = true;
                    true
                }
                else {
                    self.display = format!("{}{}",self.display,9).to_string();
                    true
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <div class="calculator">
                    <p class="display">{ &self.display }</p>
                    <div class="buttons">
                        <button onclick={link.callback(|_| Msg::One)}>{"1"}</button>
                        <button onclick={link.callback(|_| Msg::Two)}>{"2"}</button>
                        <button onclick={link.callback(|_| Msg::Three)}>{"3"}</button>
                        <button class="operator" onclick={link.callback(|_| Msg::Add)}>{"+"}</button>
                        
                        <button onclick={link.callback(|_| Msg::Four)}>{"4"}</button>
                        <button onclick={link.callback(|_| Msg::Five)}>{"5"}</button>
                        <button onclick={link.callback(|_| Msg::Six)}>{"6"}</button>
                        <button class="operator" onclick={link.callback(|_| Msg::Subtract)}>{"-"}</button>
                        
                        <button onclick={link.callback(|_| Msg::Seven)}>{"7"}</button>
                        <button onclick={link.callback(|_| Msg::Eight)}>{"8"}</button>
                        <button onclick={link.callback(|_| Msg::Nine)}>{"9"}</button>
                        <button class="operator" onclick={link.callback(|_| Msg::Multiply)}>{"*"}</button>
                        
                        <button onclick={link.callback(|_| Msg::Zero)}>{"0"}</button>
                        <button onclick={link.callback(|_| Msg::Decimal)}>{"."}</button>
                        <button class="all-clear" onclick={link.callback(|_| Msg::Clear)}>{"C"}</button>
                        <button class="operator" onclick={link.callback(|_| Msg::Divide)}>{"/"}</button>

                        <button class="equal-sign" onclick={link.callback(|_| Msg::Equals)}>{"="}</button>
                    </div>
                </div>
            </>
        }
    }
}
fn main() {
    yew::start_app::<CounterComponent>();
}
