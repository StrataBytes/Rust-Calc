#![windows_subsystem = "windows"]
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 480.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };
    eframe::run_native(
        "Rust Calc",
        options,
        Box::new(|_cc| Ok(Box::new(CalcApp::default()))),
    )
}

//Class in rust is named struct (structured)
#[derive(Default)]
struct CalcApp {
    display: String,
    //f64 is a floating point of 64 bits
    accumulator: f64,
    pending_op: Option<char>,
    reset_display: bool,
    //Opton<String> vs String. Option (Optional) means that there will or not be a value assign to it. Which means it could be Null
    //String means that there will be a mandatory presence of a value in the variable
    error: Option<String>,
}

//Implementation (Spot to make all your functions of this class)
impl CalcApp {
    fn clear_all(&mut self) {
        self.display.clear();
        self.accumulator = 0.0;
        self.pending_op = None;
        self.reset_display = false;
        self.error = None;
    }

    fn push_digit(&mut self, d:char){
        if self.reset_display || self.display == "0"{
            self.display.clear();
            self.reset_display = false;
        }
        if self.display.len() < 20{
            self.display.push(d);
        }
    }

    fn push_dot(&mut self){
        if self.reset_display{
            self.display.clear();
            self.reset_display=false;
        }
        if !self.display.contains('.'){
            if self.display.is_empty(){
                self.display.push('0');
            }
            self.display.push('.');
        }
    }

    fn current_value(&self) -> Option<f64>{
        if self.display.is_empty(){
            Some(0.0)
        }
        else{
            self.display.parse::<f64>().ok()
        }
    }

    fn apply_pending(&mut self, rhs:f64) -> Result<f64, String>{
        //LHS maps to the stored accumulator (the running total). RHS maps to the current display value (the new input).
        let lhs = self.accumulator;

        match self.pending_op{
            Some('+') => Ok(lhs + rhs),
            Some('-') => Ok(lhs - rhs), 
            Some('×') | Some('x') | Some('*') => Ok(lhs * rhs),
            Some('÷') | Some('/') => {
                if rhs == 0.0{
                    //.into converts the datatype to whatever it sees fit.
                    Err("Cannot divide by zero!".into())
                }else {
                    Ok(lhs / rhs)
                }
            }
            None => Ok(rhs),
            Some (op) => Err(format!(" Unknown operator: {op}")),

        }
    }

    fn set_op(&mut self, op:char){
        self.error = None;

        if self.pending_op.is_some() && self.reset_display {
            self.pending_op = Some(op);
            return;
        }

        match self.current_value(){
            //Val is just a variable that does not have to be define to be used, it will be part of the logic to make sure if there is op or not
            Some(val) => {
                if self.pending_op.is_none(){
                    self.accumulator = val;
                } else {
                    //If there is an action, start the apply pending operation function
                    match self.apply_pending(val){
                        Ok(result) => self.accumulator = result,
                        Err(message) => {
                            self.error = Some(message);
                            self.display = "ERR".into();
                            self.pending_op = None;
                            //Return = Break
                            return;
                        }
                    }
                }
                self.pending_op = Some(op);
                self.reset_display = true;
                self.display = format_number(self.accumulator);
            }
            None => {
                // fixes invalid displays to be a zero
                self.error = Some("Invalid number".into());
                self.display = "ERR".into();
            }
        }
    }


    //Function of EQUAL -> Is a action and not a return function
    
    fn equals(&mut self){
        //rst any prior error state before doing the operation
        self.error = None;
        //Pull the current number from the display 
        match self.current_value(){
             //worked? -> bind the number to rhs
            Some(rhs) => 
            //apply the pending operator between the accumulator (LHS) and rhs.
            match self.apply_pending(rhs){
                 //success: got a numeric result from the operation
                Ok(result) => {
                    self.accumulator = result;
                    self.display = format_number(result);
                    self.pending_op = None;
                    self.reset_display = true;
                }
                //fail: division by zero or unknown operator
                Err(message) => {
                    self.error = Some(message);
                    self.display = "ERR".into();
                    self.pending_op = None;
                }
            },
            None => {
                self.error = Some("Invalid number".into());
                self.display = "ERR".into();
            }
        }
    }

    //[DONE] We need to apply the symbols, [DONE] then set up the chain operation and [DONE] lastly the equal operation.

    //Backspace function

    fn backspace (&mut self){
        if self.reset_display{
            return;
        }
        self.display.pop();

    }
    

    //Plus minus function

    fn plus_minus(&mut self){
        if let Some(val) = self.current_value() {
            let flipped = -val;
            self.display = format_number(flipped);
        }
    }

    //Setting up EFRAME

}

//impl: implementing, eframe: library of GUI, App: product or method of the library, for CalcApp: who is this for? this is for our struct of CalcApp

impl eframe::App for CalcApp{
    //ctx: is the global UI context (styles, repaints, inputs, etc)
    //_frame: is the window
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Centralpanel is a blank canvas basically a tk.Canvas from python
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                //Title header
                /**/
                ui.add_space(10.0);
                ui.heading("Rust Calc");
                ui.add_space(10.0);
                
                let display_text = if self.display.is_empty() {"0".to_owned()} else {self.display.clone()};
                
                //background frame
                ui.allocate_ui(egui::vec2(320.0, 60.0), |ui| {
                    egui::Frame::new()
                        .fill(egui::Color32::from_gray(40))
                        .corner_radius(5.0)
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                //show operator if exists
                                if let Some(op) = self.pending_op {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        ui.label(egui::RichText::new(op.to_string())
                                            .size(14.0)
                                            .color(egui::Color32::from_rgb(100, 180, 255)));
                                    });
                                }
                                //main display
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(egui::RichText::new(&display_text)
                                        .monospace()
                                        .size(28.0)
                                        .color(egui::Color32::from_gray(240)));
                                });
                            });
                        });
                });
                
                //error display
                if let Some(err) = &self.error {
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new(err)
                        .color(egui::Color32::from_rgb(255, 100, 100))
                        .size(12.0));
                }
                
                ui.add_space(15.0);

                //button grid
                let btn_size: egui::Vec2 = egui::vec2(70.0, 50.0);
                let btn_spacing = 5.0;

                let btn_w = btn_size.x;
                let gap = btn_spacing;
                let cols = 4.0;

                let row_width = (btn_w * cols) + (gap * (cols - 1.0));
                let letf_padding = ((ui.available_width() - row_width) / 2.0).max(0.0);
                
                //Row 1: Clear, Plus/Minus, Backspace, Divide
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.add_space(letf_padding);
                    if ui.add(egui::Button::new("AC").min_size(btn_size)).clicked() {
                        self.clear_all();
                    }
                    if ui.add(egui::Button::new("+/-").min_size(btn_size)).clicked() {
                        self.plus_minus();
                    }
                    if ui.add(egui::Button::new("<-").min_size(btn_size)).clicked() {
                        self.backspace();
                    }
                    let div_btn = egui::Button::new("÷").min_size(btn_size);
                    let div_btn = if self.pending_op == Some('÷') {
                        div_btn.fill(egui::Color32::from_rgb(70, 130, 180))
                    } else { div_btn };
                    if ui.add(div_btn).clicked() {
                        self.set_op('÷');
                    }
                });
                ui.add_space(btn_spacing);

                //Row 2: 7, 8, 9, Multiply
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.add_space(letf_padding);
                    if ui.add(egui::Button::new("7").min_size(btn_size)).clicked() {
                        self.push_digit('7');
                    }
                    if ui.add(egui::Button::new("8").min_size(btn_size)).clicked() {
                        self.push_digit('8');
                    }
                    if ui.add(egui::Button::new("9").min_size(btn_size)).clicked() {
                        self.push_digit('9');
                    }
                    let mul_btn = egui::Button::new("×").min_size(btn_size);
                    let mul_btn = if self.pending_op == Some('×') {
                        mul_btn.fill(egui::Color32::from_rgb(70, 130, 180))
                    } else { mul_btn };
                    if ui.add(mul_btn).clicked() {
                        self.set_op('×');
                    }
                });
                ui.add_space(btn_spacing);

                //Row 3: 4, 5, 6, Minus
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.add_space(letf_padding);
                    if ui.add(egui::Button::new("4").min_size(btn_size)).clicked() {
                        self.push_digit('4');
                    }
                    if ui.add(egui::Button::new("5").min_size(btn_size)).clicked() {
                        self.push_digit('5');
                    }
                    if ui.add(egui::Button::new("6").min_size(btn_size)).clicked() {
                        self.push_digit('6');
                    }
                    let sub_btn = egui::Button::new("-").min_size(btn_size);
                    let sub_btn = if self.pending_op == Some('-') {
                        sub_btn.fill(egui::Color32::from_rgb(70, 130, 180))
                    } else { sub_btn };
                    if ui.add(sub_btn).clicked() {
                        self.set_op('-');
                    }
                });
                ui.add_space(btn_spacing);

                //Row 4: 1, 2, 3, Plus
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.add_space(letf_padding);
                    if ui.add(egui::Button::new("1").min_size(btn_size)).clicked() {
                        self.push_digit('1');
                    }
                    if ui.add(egui::Button::new("2").min_size(btn_size)).clicked() {
                        self.push_digit('2');
                    }
                    if ui.add(egui::Button::new("3").min_size(btn_size)).clicked() {
                        self.push_digit('3');
                    }
                    let add_btn = egui::Button::new("+").min_size(btn_size);
                    let add_btn = if self.pending_op == Some('+') {
                        add_btn.fill(egui::Color32::from_rgb(70, 130, 180))
                    } else { add_btn };
                    if ui.add(add_btn).clicked() {
                        self.set_op('+');
                    }
                });
                ui.add_space(btn_spacing);

                //Row 5: 0, decimal, equals
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.add_space(letf_padding);
                    let wide_btn = egui::vec2(btn_size.x * 2.0 + btn_spacing, btn_size.y);
                    if ui.add(egui::Button::new("0").min_size(wide_btn)).clicked() {
                        self.push_digit('0');
                    }
                    if ui.add(egui::Button::new(".").min_size(btn_size)).clicked() {
                        self.push_dot();
                    }
                    if ui.add(egui::Button::new("=").min_size(btn_size)).clicked() {
                        self.equals();
                    }
                });

                ui.input(|i| {
                    for ev in &i.events {
                        if let egui::Event::Text(t) = ev {
                            for ch in t.chars() {
                                match ch {
                                    '0'..='9' => self.push_digit(ch),
                                    '.' => self.push_dot(),
                                    '+' => self.set_op('+'),
                                    '-' => self.set_op('-'),
                                    '/' => self.set_op('÷'),
                                    '*' => self.set_op('×'),
                                    _ => {}
                                }
                            }
                        }
                    }
                    if i.key_pressed(egui::Key::Backspace) { self.backspace(); }
                    if i.key_pressed(egui::Key::Enter) { self.equals(); }
                });

                ui.add_space(8.0);
            });
        });
    }

}


fn format_number(n: f64) -> String {
    let mut s = format!("{:.12}", n);
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    if s.is_empty() {
        s.push('0');
    }
    s
}